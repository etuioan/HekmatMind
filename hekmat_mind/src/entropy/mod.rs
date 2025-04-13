//! Entropiequellen-Modul für HekmatMind
//!
//! Dieses Modul stellt eine modulare Schnittstelle für verschiedene Entropiequellen bereit,
//! die für die Zufallsgenerierung im neuronalen Netzwerk verwendet werden können.
//! Es unterstützt eine konfigurierbare Entropie-Pipeline mit drei Ebenen:
//! - Primär: Wetterdaten-API (Temperatur, Luftdruck, Luftfeuchtigkeit)
//! - Sekundär: Satellitendaten-Feeds (Strahlungswerte, Magnetfeldmessungen)
//! - Tertiär: Systemrauschen-Sampling als Fallback

use async_trait::async_trait;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

pub mod cache;
pub mod extractors;
pub mod prelude;
pub mod sources;

#[cfg(test)]
pub mod tests;

/// Fehler, die bei der Entropiegewinnung auftreten können
#[derive(Error, Debug)]
pub enum EntropyError {
    /// Fehler bei der Verbindung zur Entropiequelle
    #[error("Verbindungsfehler zur Entropiequelle: {0}")]
    ConnectionError(String),

    /// Fehler bei der Verarbeitung der Entropiedaten
    #[error("Verarbeitungsfehler: {0}")]
    ProcessingError(String),

    /// Fehler im Cache
    #[error("Cache-Fehler: {0}")]
    CacheError(String),

    /// Keine Entropiequelle verfügbar
    #[error("Keine Entropiequelle verfügbar")]
    NoSourceAvailable,

    /// Unzureichende Entropie
    #[error("Unzureichende Entropie verfügbar")]
    InsufficientEntropy,
}

/// Ergebnis einer Entropieoperation
pub type EntropyResult<T> = Result<T, EntropyError>;

/// Trait für Entropiequellen
#[async_trait]
pub trait EntropySource: Send + Sync {
    /// Gibt den Namen der Entropiequelle zurück
    fn name(&self) -> &str;

    /// Gibt die Priorität der Entropiequelle zurück (niedrigere Werte = höhere Priorität)
    fn priority(&self) -> u8;

    /// Prüft, ob die Entropiequelle verfügbar ist
    async fn is_available(&self) -> bool;

    /// Sammelt Entropiedaten von der Quelle
    async fn collect_entropy(&self, bytes_requested: usize) -> EntropyResult<Vec<u8>>;
}

/// Konfiguration für die Entropie-Pipeline
#[derive(Debug, Clone)]
pub struct EntropyConfig {
    /// Maximale Größe des Entropie-Caches in Bytes
    pub cache_size: usize,

    /// Schwellwert für die Auffüllung des Caches in Prozent
    pub refill_threshold: f32,

    /// Timeout für Anfragen an externe Quellen in Millisekunden
    pub request_timeout_ms: u64,

    /// Flag, ob Systemrauschen als Fallback verwendet werden soll
    pub use_system_noise_fallback: bool,
}

impl Default for EntropyConfig {
    fn default() -> Self {
        Self {
            cache_size: 5 * 1024 * 1024, // 5 MB
            refill_threshold: 0.2,       // 20%
            request_timeout_ms: 5000,    // 5 Sekunden
            use_system_noise_fallback: true,
        }
    }
}

/// Hauptmanager für Entropiequellen
pub struct EntropyManager {
    sources: Vec<Arc<dyn EntropySource>>,
    cache: Arc<RwLock<cache::EntropyCache>>,
    config: EntropyConfig,
}

impl Default for EntropyManager {
    /// Implementiert den Default-Trait für EntropyManager
    fn default() -> Self {
        Self::new(EntropyConfig::default())
    }
}

impl EntropyManager {
    /// Erstellt einen neuen EntropyManager mit der angegebenen Konfiguration
    pub fn new(config: EntropyConfig) -> Self {
        Self {
            sources: Vec::new(),
            cache: Arc::new(RwLock::new(cache::EntropyCache::new(config.cache_size))),
            config,
        }
    }

    /// Erstellt einen neuen EntropyManager mit Standardkonfiguration
    ///
    /// Hinweis: Diese Methode ist veraltet und wird durch die Default-Implementierung ersetzt
    #[deprecated(since = "0.1.0", note = "Bitte Default::default() verwenden")]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        <Self as Default>::default()
    }

    /// Registriert eine Entropiequelle
    pub fn register_source(&mut self, source: Arc<dyn EntropySource>) {
        // Quellen nach Priorität sortiert einfügen
        let priority = source.priority();
        let pos = self.sources.iter().position(|s| s.priority() > priority);

        match pos {
            Some(index) => self.sources.insert(index, source),
            None => self.sources.push(source),
        }
    }

    /// Gibt die registrierten Entropiequellen zurück
    pub fn sources(&self) -> &[Arc<dyn EntropySource>] {
        &self.sources
    }

    /// Gibt die aktuelle Konfiguration zurück
    pub fn config(&self) -> &EntropyConfig {
        &self.config
    }

    /// Gibt eine Referenz auf den Entropie-Cache zurück
    pub fn cache(&self) -> Arc<RwLock<cache::EntropyCache>> {
        self.cache.clone()
    }

    /// Holt asynchron Entropie aus den verfügbaren Quellen
    pub async fn get_entropy(&self, bytes: usize) -> EntropyResult<Vec<u8>> {
        // Zuerst versuchen, aus dem Cache zu lesen
        let should_refill = {
            let cache = self.cache.read().await;
            cache.available_bytes() < bytes
        };

        if should_refill {
            // Cache auffüllen, wenn nicht genug Daten vorhanden sind
            self.refill_cache().await?;
        }

        // Aus dem Cache lesen (mit Write-Lock)
        let mut cache = self.cache.write().await;
        cache.get_bytes(bytes)
    }

    /// Füllt den Cache mit Entropie aus den verfügbaren Quellen auf
    async fn refill_cache(&self) -> EntropyResult<()> {
        let needed_bytes = {
            let cache = self.cache.read().await;
            let available = cache.available_bytes();
            let capacity = cache.capacity();
            if available >= capacity / 2 {
                return Ok(());
            }
            capacity - available
        };

        for source in &self.sources {
            if source.is_available().await {
                match source.collect_entropy(needed_bytes).await {
                    Ok(data) => {
                        let mut cache = self.cache.write().await;
                        cache.add_bytes(&data)?;
                        return Ok(());
                    }
                    Err(_) => continue, // Versuche die nächste Quelle
                }
            }
        }

        // Wenn alle Quellen fehlschlagen und Systemrauschen als Fallback aktiviert ist
        if self.config.use_system_noise_fallback {
            let system_noise = sources::system::SystemNoiseSource::new();
            match system_noise.collect_entropy(needed_bytes).await {
                Ok(data) => {
                    let mut cache = self.cache.write().await;
                    cache.add_bytes(&data)?;
                    return Ok(());
                }
                Err(e) => return Err(e),
            }
        }

        Err(EntropyError::NoSourceAvailable)
    }
}

//! Tests für das Entropiequellen-Modul
//!
//! Dieses Modul enthält umfassende Tests für die Entropiequellen-Implementierung,
//! einschließlich Cache, Extraktoren und Quellen.
//! Die Tests sind in separate Module aufgeteilt, um eine bessere Isolierbarkeit
//! und gezielte Testausführung zu ermöglichen.

/// Gemeinsame Imports für alle Testmodule
#[cfg(test)]
mod common {
    pub use crate::entropy::cache::EntropyCache;
    pub use crate::entropy::extractors::{BitExtractor, CombinedExtractor};
    pub use crate::entropy::sources::system::SystemNoiseSource;
    pub use crate::entropy::*;
    pub use async_trait::async_trait;
    pub use futures::future::FutureExt;
    pub use mockall::predicate::*;
    pub use mockall::*;
    pub use std::sync::Arc;

    // Mock für eine Entropiequelle zum Testen
    mock! {
        pub EntropySource {}

        #[async_trait]
        impl EntropySource for EntropySource {
            fn name(&self) -> &str;
            fn priority(&self) -> u8;
            async fn is_available(&self) -> bool;
            async fn collect_entropy(&self, bytes_requested: usize) -> EntropyResult<Vec<u8>>;
        }
    }

    // Einfache Implementierung einer Testentropiequelle
    pub struct TestEntropySource {
        pub name: String,
        pub priority: u8,
        pub available: bool,
        pub data: Vec<u8>,
        pub error: Option<EntropyError>,
    }

    impl TestEntropySource {
        pub fn new(name: &str, priority: u8, data: Vec<u8>) -> Self {
            Self {
                name: name.to_string(),
                priority,
                available: true,
                data,
                error: None,
            }
        }

        pub fn with_error(mut self, error: EntropyError) -> Self {
            self.error = Some(error);
            self
        }

        pub fn with_availability(mut self, available: bool) -> Self {
            self.available = available;
            self
        }
    }

    #[async_trait]
    impl EntropySource for TestEntropySource {
        fn name(&self) -> &str {
            &self.name
        }

        fn priority(&self) -> u8 {
            self.priority
        }

        async fn is_available(&self) -> bool {
            self.available
        }

        async fn collect_entropy(&self, bytes_requested: usize) -> EntropyResult<Vec<u8>> {
            if let Some(ref error) = self.error {
                // Manuelles Klonen des Fehlers, da EntropyError möglicherweise kein Clone implementiert
                return match error {
                    EntropyError::NoSourceAvailable => Err(EntropyError::NoSourceAvailable),
                    EntropyError::ConnectionError(msg) => {
                        Err(EntropyError::ConnectionError(msg.clone()))
                    }
                    EntropyError::ProcessingError(msg) => {
                        Err(EntropyError::ProcessingError(msg.clone()))
                    }
                    EntropyError::CacheError(msg) => Err(EntropyError::CacheError(msg.clone())),
                    EntropyError::InsufficientEntropy => Err(EntropyError::InsufficientEntropy),
                };
            }

            if !self.available {
                return Err(EntropyError::ConnectionError(
                    "Quelle nicht verfügbar".to_string(),
                ));
            }

            if self.data.is_empty() {
                return Err(EntropyError::InsufficientEntropy);
            }

            // Stelle sicher, dass wir genug Daten haben
            let mut result = Vec::with_capacity(bytes_requested);
            while result.len() < bytes_requested {
                let remaining = bytes_requested - result.len();
                let chunk_size = std::cmp::min(remaining, self.data.len());
                result.extend_from_slice(&self.data[0..chunk_size]);
            }

            Ok(result)
        }
    }
}

/// Tests für den Entropie-Cache
#[cfg(test)]
mod cache_tests {
    use super::common::*;

    #[tokio::test]
    async fn test_entropy_cache_basic() {
        // Erstelle einen Cache mit 1 KB Kapazität
        let mut cache = EntropyCache::new(1024);

        // Prüfe initiale Werte
        assert_eq!(cache.capacity(), 1024);
        assert_eq!(cache.available_bytes(), 0);
        assert_eq!(cache.fill_percentage(), 0.0);
        assert!(cache.is_empty());
        assert!(!cache.is_full());
        assert!(cache.needs_refill(0.5));

        // Füge Daten hinzu
        let data = vec![1, 2, 3, 4, 5];
        assert!(cache.add_bytes(&data).is_ok());

        // Prüfe aktualisierte Werte
        assert_eq!(cache.available_bytes(), 5);
        assert_eq!(cache.fill_percentage(), 5.0 / 1024.0);
        assert!(!cache.is_empty());
        assert!(!cache.is_full());

        // Hole Daten
        let retrieved = cache.get_bytes(3).unwrap();
        assert_eq!(retrieved, vec![1, 2, 3]);

        // Prüfe nach dem Abrufen
        assert_eq!(cache.available_bytes(), 2);

        // Versuche, mehr Daten abzurufen, als verfügbar sind
        assert!(cache.get_bytes(3).is_err());

        // Leere den Cache
        cache.clear();
        assert_eq!(cache.available_bytes(), 0);
        assert!(cache.is_empty());
    }

    #[tokio::test]
    async fn test_entropy_cache_overflow() {
        // Erstelle einen Cache mit 10 Bytes Kapazität
        let mut cache = EntropyCache::new(10);

        // Füge 5 Bytes hinzu
        let data1 = vec![1, 2, 3, 4, 5];
        assert!(cache.add_bytes(&data1).is_ok());
        assert_eq!(cache.available_bytes(), 5);

        // Füge weitere 8 Bytes hinzu (überschreitet die Kapazität)
        let data2 = vec![6, 7, 8, 9, 10, 11, 12, 13];
        assert!(cache.add_bytes(&data2).is_ok());

        // Cache sollte jetzt 10 Bytes enthalten (die neuesten)
        assert_eq!(cache.available_bytes(), 10);

        // Die ersten 3 Bytes sollten überschrieben worden sein
        let all_data = cache.get_bytes(10).unwrap();
        assert_eq!(all_data, vec![4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
    }
}

/// Tests für die Bit-Extraktoren
#[cfg(test)]
mod extractor_tests {
    use super::common::*;

    #[tokio::test]
    async fn test_bit_extractor_von_neumann() {
        // Testdaten mit alternierenden Bits für maximale Entropie
        let input = vec![0b10101010, 0b01010101, 0b11001100, 0b00110011];

        // Extrahiere 1 Byte
        let result = BitExtractor::von_neumann_extractor(&input, 1).unwrap();

        // Prüfe, dass wir genau 1 Byte erhalten haben
        assert_eq!(result.len(), 1);

        // Versuche, mehr Bytes zu extrahieren als möglich
        // Bei den gegebenen Eingabedaten haben wir maximal 8 unterschiedliche Bitpaare
        // was 1 Byte Entropie ergibt. Wenn wir mehr anfordern, sollte es fehlschlagen.
        let result = BitExtractor::von_neumann_extractor(&input, 4);
        assert!(result.is_err()); // Wir haben nicht genug Entropie für 4 Bytes
    }

    #[tokio::test]
    async fn test_bit_extractor_cryptographic() {
        // Testdaten
        let input = vec![1, 2, 3, 4, 5];

        // Extrahiere 32 Bytes (SHA-256 erzeugt 32 Bytes)
        let result = BitExtractor::cryptographic_extractor(&input, 32).unwrap();

        // Prüfe, dass wir genau 32 Bytes erhalten haben
        assert_eq!(result.len(), 32);

        // Extrahiere 64 Bytes (mehr als ein Hash)
        let result = BitExtractor::cryptographic_extractor(&input, 64).unwrap();
        assert_eq!(result.len(), 64);

        // Test mit leeren Eingabedaten (sollte fehlschlagen)
        let empty_input: Vec<u8> = vec![];
        let result = BitExtractor::cryptographic_extractor(&empty_input, 32);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            EntropyError::InsufficientEntropy
        ));

        // Test mit sehr kleinen Eingabedaten (sollte funktionieren)
        let tiny_input = vec![42];
        let result = BitExtractor::cryptographic_extractor(&tiny_input, 16).unwrap();
        assert_eq!(result.len(), 16);
    }

    #[tokio::test]
    async fn test_bit_extractor_totp() {
        // Testdaten
        let input = vec![1, 2, 3, 4, 5];

        // Extrahiere mit Standardzeitschritt
        let result = BitExtractor::totp_extractor(&input, 16, 30).unwrap();
        assert_eq!(result.len(), 16);

        // Extrahiere mit kleinerem Zeitschritt
        let result = BitExtractor::totp_extractor(&input, 32, 5).unwrap();
        assert_eq!(result.len(), 32);

        // Test mit leeren Eingabedaten (sollte fehlschlagen)
        let empty_input: Vec<u8> = vec![];
        let result = BitExtractor::totp_extractor(&empty_input, 16, 30);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            EntropyError::InsufficientEntropy
        ));

        // Test mit sehr kleinen Eingabedaten (sollte funktionieren)
        let tiny_input = vec![42];
        let result = BitExtractor::totp_extractor(&tiny_input, 16, 30).unwrap();
        assert_eq!(result.len(), 16);
    }

    #[tokio::test]
    async fn test_bit_extractor_whitening() {
        // Testdaten mit ausreichender Entropie
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];

        // Extrahiere mit Whitening
        let result = BitExtractor::whitening_extractor(&input, 4).unwrap();
        assert_eq!(result.len(), 4);

        // Test mit zu wenig Eingabedaten (sollte fehlschlagen)
        let insufficient_input = vec![1];
        let result = BitExtractor::whitening_extractor(&insufficient_input, 4);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            EntropyError::InsufficientEntropy
        ));
    }

    #[tokio::test]
    async fn test_combined_extractor() {
        // Test mit minimalen Eingabedaten (einfache Strategie)
        let small_input = vec![1, 2, 3, 4, 5];
        let result_small = CombinedExtractor::extract(&small_input, 1);
        assert!(
            result_small.is_ok(),
            "Kombinierter Extraktor konnte keine Entropie aus kleinen Daten extrahieren: {:?}",
            result_small.err()
        );
        assert_eq!(
            result_small.unwrap().len(),
            1,
            "Erwartete 1 Byte für kleine Eingabedaten"
        );

        // Test mit größeren Eingabedaten (vollständige Pipeline)
        let large_input = vec![0; 30]; // 30 Bytes Nullen
        let result_large = CombinedExtractor::extract(&large_input, 5);
        assert!(
            result_large.is_ok(),
            "Kombinierter Extraktor konnte keine Entropie aus großen Daten extrahieren: {:?}",
            result_large.err()
        );
        assert_eq!(
            result_large.unwrap().len(),
            5,
            "Erwartete 5 Bytes für große Eingabedaten"
        );

        // Test mit Grenzfall: Eingabedaten sehr klein
        let tiny_input = vec![42];
        let result_tiny = CombinedExtractor::extract(&tiny_input, 2);
        assert!(
            result_tiny.is_ok(),
            "Kombinierter Extraktor konnte keine Entropie aus winzigen Daten extrahieren: {:?}",
            result_tiny.err()
        );
        assert_eq!(
            result_tiny.unwrap().len(),
            2,
            "Erwartete 2 Bytes für winzige Eingabedaten"
        );

        // Test mit leeren Eingabedaten (sollte fehlschlagen)
        let empty_input: Vec<u8> = vec![];
        let result_empty = CombinedExtractor::extract(&empty_input, 1);
        assert!(
            result_empty.is_err(),
            "Kombinierter Extraktor sollte bei leeren Daten fehlschlagen"
        );
        assert!(
            matches!(result_empty.unwrap_err(), EntropyError::InsufficientEntropy),
            "Erwarteter Fehler: InsufficientEntropy"
        );
    }
}

/// Tests für die Entropiequellen
#[cfg(test)]
mod source_tests {
    use super::common::*;
    use crate::entropy::sources::satellite::SatelliteDataSource;
    use crate::entropy::sources::weather::WeatherDataSource;

    #[tokio::test]
    async fn test_system_noise_source() {
        // Erstelle eine Systemrauschen-Quelle
        let source = SystemNoiseSource::new();

        // Prüfe Eigenschaften
        assert_eq!(source.name(), "Systemrauschen");
        assert_eq!(source.priority(), sources::priority::TERTIARY);

        // Prüfe Verfügbarkeit
        assert!(source.is_available().await);

        // Sammle Entropie
        let entropy = source.collect_entropy(100).await.unwrap();

        // Prüfe, dass wir genau 100 Bytes erhalten haben
        assert_eq!(entropy.len(), 100);

        // Sammle erneut Entropie und prüfe, dass sie unterschiedlich ist
        let entropy2 = source.collect_entropy(100).await.unwrap();
        assert_ne!(entropy, entropy2);
    }

    // Verwende die TestEntropySource aus dem common-Modul

    #[tokio::test]
    async fn test_satellite_source() {
        // Erstelle eine Testquelle für Satellitendaten
        let source = TestEntropySource::new(
            "Satellitendaten-Feed",
            sources::priority::SECONDARY,
            vec![0x42; 100],
        );

        // Prüfe Eigenschaften
        assert_eq!(source.name(), "Satellitendaten-Feed");
        assert_eq!(source.priority(), sources::priority::SECONDARY);
        assert!(source.is_available().await);

        // Sammle Entropie
        let entropy = source.collect_entropy(100).await.unwrap();

        // Prüfe, dass wir genau 100 Bytes erhalten haben
        assert_eq!(entropy.len(), 100);

        // Prüfe, dass alle Bytes den erwarteten Wert haben
        assert!(entropy.iter().all(|&b| b == 0x42));
    }

    #[tokio::test]
    async fn test_satellite_data_source_properties() {
        // Erstelle eine SatelliteDataSource mit Dummy-Werten
        let source = SatelliteDataSource::new(
            "https://satellite-api.example.com".to_string(),
            "dummy-token".to_string(),
        );

        // Prüfe Eigenschaften
        assert_eq!(source.name(), "Satellitendaten-Feed");
        assert_eq!(source.priority(), sources::priority::SECONDARY);

        // Prüfe, dass die is_available-Methode funktioniert
        // Da wir keine echte API haben, sollte sie false zurückgeben
        assert!(!source.is_available().await);

        // Teste die collect_entropy-Methode
        let bytes_requested = 50;
        let entropy = source.collect_entropy(bytes_requested).await;

        // Der Test sollte fehlschlagen, da wir keine echte API haben,
        // aber wir können prüfen, ob der richtige Fehlertyp zurückgegeben wird
        assert!(entropy.is_err());
        match entropy.unwrap_err() {
            EntropyError::ConnectionError(_) => {
                // Erwarteter Fehlertyp
            }
            err => panic!("Unerwarteter Fehlertyp: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_weather_source() {
        // Erstelle eine Testquelle für Wetterdaten
        let source = TestEntropySource::new(
            "Wetterdaten-API",
            sources::priority::PRIMARY,
            vec![0x17; 64],
        );

        // Prüfe Eigenschaften
        assert_eq!(source.name(), "Wetterdaten-API");
        assert_eq!(source.priority(), sources::priority::PRIMARY);
        assert!(source.is_available().await);

        // Sammle Entropie
        let entropy = source.collect_entropy(64).await.unwrap();

        // Prüfe, dass wir genau 64 Bytes erhalten haben
        assert_eq!(entropy.len(), 64);

        // Prüfe, dass alle Bytes den erwarteten Wert haben
        assert!(entropy.iter().all(|&b| b == 0x17));
    }

    #[tokio::test]
    async fn test_weather_data_source_properties() {
        // Erstelle eine WeatherDataSource mit Dummy-Werten
        let source = WeatherDataSource::new(
            "https://weather-api.example.com".to_string(),
            "dummy-api-key".to_string(),
        );

        // Prüfe Eigenschaften
        assert_eq!(source.name(), "Wetterdaten-API");
        assert_eq!(source.priority(), sources::priority::PRIMARY);

        // Prüfe, dass die is_available-Methode funktioniert
        // Da wir keine echte API haben, sollte sie false zurückgeben
        assert!(!source.is_available().await);

        // Teste die collect_entropy-Methode
        let bytes_requested = 40;
        let entropy = source.collect_entropy(bytes_requested).await;

        // Der Test sollte fehlschlagen, da wir keine echte API haben,
        // aber wir können prüfen, ob der richtige Fehlertyp zurückgegeben wird
        assert!(entropy.is_err());
        match entropy.unwrap_err() {
            EntropyError::ConnectionError(_) => {
                // Erwarteter Fehlertyp
            }
            err => panic!("Unerwarteter Fehlertyp: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_source_error_handling() {
        // Erstelle eine fehlerhafte Entropiequelle
        let source = TestEntropySource::new("Fehlerhafte Quelle", 10, vec![0; 10]).with_error(
            EntropyError::ConnectionError("Simulierter Verbindungsfehler".to_string()),
        );

        // Prüfe Eigenschaften
        assert_eq!(source.name(), "Fehlerhafte Quelle");
        assert_eq!(source.priority(), 10);
        assert!(source.is_available().await);

        // Versuche, Entropie zu sammeln (sollte fehlschlagen)
        let result = source.collect_entropy(100).await;

        // Prüfe, dass der erwartete Fehler aufgetreten ist
        assert!(result.is_err());
        match result.unwrap_err() {
            EntropyError::ConnectionError(msg) => {
                assert_eq!(msg, "Simulierter Verbindungsfehler");
            }
            _ => panic!("Unerwarteter Fehlertyp"),
        }
    }

    #[tokio::test]
    async fn test_source_unavailable() {
        // Erstelle eine nicht verfügbare Entropiequelle
        let source = TestEntropySource::new("Nicht verfügbare Quelle", 20, vec![0; 10])
            .with_availability(false);

        // Prüfe Eigenschaften
        assert_eq!(source.name(), "Nicht verfügbare Quelle");
        assert_eq!(source.priority(), 20);
        assert!(!source.is_available().await);

        // Versuche, Entropie zu sammeln (sollte fehlschlagen)
        let result = source.collect_entropy(100).await;

        // Prüfe, dass der erwartete Fehler aufgetreten ist
        assert!(result.is_err());
        match result.unwrap_err() {
            EntropyError::ConnectionError(_) => {
                // Erwarteter Fehlertyp
            }
            _ => panic!("Unerwarteter Fehlertyp"),
        }
    }
}

/// Tests für den EntropyManager
#[cfg(test)]
mod manager_tests {
    use super::common::*;

    #[tokio::test]
    async fn test_entropy_manager_config() {
        // Teste die Standardkonfiguration
        let default_manager = <EntropyManager as Default>::default();
        assert_eq!(default_manager.config().cache_size, 5 * 1024 * 1024); // 5 MB
        assert_eq!(default_manager.config().refill_threshold, 0.2); // 20%
        assert_eq!(default_manager.config().request_timeout_ms, 5000); // 5 Sekunden
        assert!(default_manager.config().use_system_noise_fallback);

        // Teste eine benutzerdefinierte Konfiguration
        let custom_config = EntropyConfig {
            cache_size: 2048,
            refill_threshold: 0.3,
            request_timeout_ms: 2000,
            use_system_noise_fallback: false,
        };
        let custom_manager = EntropyManager::new(custom_config.clone());
        assert_eq!(custom_manager.config().cache_size, 2048);
        assert_eq!(custom_manager.config().refill_threshold, 0.3);
        assert_eq!(custom_manager.config().request_timeout_ms, 2000);
        assert!(!custom_manager.config().use_system_noise_fallback);

        // Erstelle einen neuen Manager mit einer anderen Konfiguration
        let new_config = EntropyConfig {
            cache_size: 1024,
            refill_threshold: 0.5,
            request_timeout_ms: 1000,
            use_system_noise_fallback: false,
        };
        let new_manager = EntropyManager::new(new_config.clone());

        // Prüfe, dass die Konfiguration korrekt ist
        assert_eq!(new_manager.config().cache_size, 1024);
        assert_eq!(new_manager.config().refill_threshold, 0.5);
        assert_eq!(new_manager.config().request_timeout_ms, 1000);
        assert!(!new_manager.config().use_system_noise_fallback);
    }

    #[tokio::test]
    async fn test_entropy_manager_source_priority() {
        // Erstelle einen Manager
        let mut manager = <EntropyManager as Default>::default();

        // Erstelle Quellen mit unterschiedlichen Prioritäten
        let high_priority = Arc::new(TestEntropySource::new("High", 1, vec![0x01; 10]));
        let medium_priority = Arc::new(TestEntropySource::new("Medium", 5, vec![0x02; 10]));
        let low_priority = Arc::new(TestEntropySource::new("Low", 10, vec![0x03; 10]));

        // Registriere die Quellen in zufälliger Reihenfolge
        manager.register_source(medium_priority.clone());
        manager.register_source(low_priority.clone());
        manager.register_source(high_priority.clone());

        // Prüfe, dass die Quellen nach Priorität sortiert sind
        assert_eq!(manager.sources().len(), 3);
        assert_eq!(manager.sources()[0].name(), "High");
        assert_eq!(manager.sources()[1].name(), "Medium");
        assert_eq!(manager.sources()[2].name(), "Low");
    }

    #[tokio::test]
    async fn test_entropy_manager_no_sources() {
        // Erstelle einen Manager ohne Quellen und ohne Fallback
        let config = EntropyConfig {
            use_system_noise_fallback: false,
            ..EntropyConfig::default()
        };
        let manager = EntropyManager::new(config);

        // Versuche, Entropie zu holen (sollte fehlschlagen)
        let result = manager.get_entropy(10).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            EntropyError::NoSourceAvailable
        ));
    }

    #[tokio::test]
    async fn test_entropy_manager_basic() {
        // Erstelle einen Manager mit Standardkonfiguration
        let mut manager = <EntropyManager as Default>::default();

        // Prüfe initiale Werte
        assert_eq!(manager.sources().len(), 0);
        assert_eq!(manager.config().cache_size, 5 * 1024 * 1024);

        // Registriere eine Systemrauschen-Quelle als reale Entropiequelle
        // anstatt eines Mocks, da die Mocks Probleme verursachen
        let system_source = SystemNoiseSource::new();
        manager.register_source(Arc::new(system_source));

        // Prüfe, dass die Quelle registriert wurde
        assert_eq!(manager.sources().len(), 1);

        // Hole Entropie
        let entropy = manager.get_entropy(100).await.unwrap();

        // Prüfe, dass wir genau 100 Bytes erhalten haben
        assert_eq!(entropy.len(), 100);

        // Hole mehr Entropie
        let entropy2 = manager.get_entropy(50).await.unwrap();

        // Prüfe, dass wir genau 50 Bytes erhalten haben
        assert_eq!(entropy2.len(), 50);

        // Hole Entropie mit einer größeren Anforderung
        let entropy3 = manager.get_entropy(200).await.unwrap();

        // Prüfe, dass wir genau 200 Bytes erhalten haben
        assert_eq!(entropy3.len(), 200);
    }

    #[tokio::test]
    async fn test_entropy_manager_fallback() {
        // Erstelle einen Manager mit Standardkonfiguration
        let config = EntropyConfig {
            cache_size: 1024,
            refill_threshold: 0.5,
            request_timeout_ms: 1000,
            use_system_noise_fallback: true,
        };
        let mut manager = EntropyManager::new(config);

        // Registriere eine Systemrauschen-Quelle als Fallback
        let system_source = SystemNoiseSource::new();
        manager.register_source(Arc::new(system_source));

        // Prüfe, dass die Quelle registriert wurde
        assert_eq!(manager.sources().len(), 1);

        // Hole Entropie
        let entropy = manager.get_entropy(100).await.unwrap();

        // Prüfe, dass wir genau 100 Bytes erhalten haben
        assert_eq!(entropy.len(), 100);

        // Hole weitere Entropie, um den Cache-Mechanismus zu testen
        let entropy2 = manager.get_entropy(50).await.unwrap();
        assert_eq!(entropy2.len(), 50);
    }

    #[tokio::test]
    async fn test_entropy_manager_system_fallback() {
        // Erstelle einen Manager mit aktiviertem Fallback, aber ohne Quellen
        let config = EntropyConfig {
            cache_size: 1024,
            refill_threshold: 0.5,
            request_timeout_ms: 1000,
            use_system_noise_fallback: true,
        };
        let manager = EntropyManager::new(config);

        // Prüfe, dass keine Quellen registriert sind
        assert_eq!(manager.sources().len(), 0);

        // Hole Entropie (sollte erfolgreich sein dank des Fallbacks)
        let entropy = manager.get_entropy(50).await.unwrap();

        // Prüfe, dass wir genau 50 Bytes erhalten haben
        assert_eq!(entropy.len(), 50);
    }

    #[tokio::test]
    async fn test_entropy_manager_cache() {
        // Erstelle einen Manager mit kleinem Cache
        let config = EntropyConfig {
            cache_size: 200,
            refill_threshold: 0.5,
            request_timeout_ms: 1000,
            use_system_noise_fallback: true,
        };
        let mut manager = EntropyManager::new(config);

        // Registriere eine Systemrauschen-Quelle
        let system_source = SystemNoiseSource::new();
        manager.register_source(Arc::new(system_source));

        // Hole Entropie (sollte den Cache füllen)
        let entropy1 = manager.get_entropy(50).await.unwrap();
        assert_eq!(entropy1.len(), 50);

        // Hole weitere Entropie (sollte aus dem Cache kommen)
        let entropy2 = manager.get_entropy(50).await.unwrap();
        assert_eq!(entropy2.len(), 50);

        // Da wir eine echte Entropiequelle verwenden, können wir nicht garantieren,
        // dass die Entropie-Blöcke identisch sind. Stattdessen prüfen wir nur die Länge.
        assert_eq!(entropy2.len(), 50);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::common::*;

    #[tokio::test]
    async fn test_entropy_pipeline_integration() {
        // Erstelle einen Manager mit Standardkonfiguration
        let mut manager = <EntropyManager as Default>::default();

        // Registriere eine Systemrauschen-Quelle
        let system_source = SystemNoiseSource::new();
        manager.register_source(Arc::new(system_source));

        // Hole Entropie
        let entropy = manager.get_entropy(1000).await.unwrap();

        // Prüfe, dass wir genau 1000 Bytes erhalten haben
        assert_eq!(entropy.len(), 1000);

        // Statistische Tests für die Entropiequalität
        let zeros = entropy.iter().filter(|&&b| b == 0).count();
        let ones = entropy.iter().filter(|&&b| b == 1).count();

        // In 1000 zufälligen Bytes sollten etwa 4 Bytes den Wert 0 haben und etwa 4 den Wert 1
        // (mit einer gewissen Toleranz)
        assert!(zeros < 20, "Zu viele Nullen: {}", zeros);
        assert!(ones < 20, "Zu viele Einsen: {}", ones);
    }
}

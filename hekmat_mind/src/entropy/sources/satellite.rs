//! Satellitendaten-Entropiequelle
//!
//! Implementiert eine Entropiequelle, die Satellitendaten von Feeds abruft
//! und daraus Entropie extrahiert. Diese Quelle verwendet Strahlungswerte
//! und Magnetfeldmessungen als Entropiequellen.

use crate::entropy::sources::priority;
use crate::entropy::{EntropyError, EntropyResult, EntropySource};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

/// Struktur für die Deserialisierung von Satellitendaten
#[derive(Debug, Deserialize)]
struct SatelliteData {
    timestamp: u64,
    radiation_level: f32,
    magnetic_field_strength: f32,
    particle_count: u32,
    orbital_position: [f32; 3],
    // Weitere Felder je nach Feed
}

/// Satellitendaten-Entropiequelle
pub struct SatelliteDataSource {
    /// Name der Quelle
    name: String,

    /// Feed-URL
    feed_url: String,

    /// Zugriffstoken
    access_token: String,

    /// HTTP-Client
    client: Client,
}

impl SatelliteDataSource {
    /// Erstellt eine neue Satellitendaten-Entropiequelle
    pub fn new(feed_url: String, access_token: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .unwrap_or_default();

        Self {
            name: "Satellitendaten-Feed".to_string(),
            feed_url,
            access_token,
            client,
        }
    }

    /// Ruft Satellitendaten vom Feed ab
    async fn fetch_satellite_data(&self) -> EntropyResult<SatelliteData> {
        let url = format!("{}?token={}", self.feed_url, self.access_token);

        let response = self.client.get(&url).send().await.map_err(|e| {
            EntropyError::ConnectionError(format!("Fehler beim Abrufen von Satellitendaten: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(EntropyError::ConnectionError(format!(
                "Feed-Fehler: HTTP {}",
                response.status()
            )));
        }

        response.json::<SatelliteData>().await.map_err(|e| {
            EntropyError::ProcessingError(format!(
                "Fehler beim Deserialisieren der Satellitendaten: {}",
                e
            ))
        })
    }

    /// Extrahiert Entropie aus Satellitendaten
    fn extract_entropy_from_satellite(
        &self,
        data: &SatelliteData,
        bytes_requested: usize,
    ) -> Vec<u8> {
        let mut result = Vec::with_capacity(bytes_requested);

        // Extrahiere Bytes aus den Werten
        let timestamp_bytes = data.timestamp.to_le_bytes();
        let radiation_bytes = data.radiation_level.to_le_bytes();
        let magnetic_bytes = data.magnetic_field_strength.to_le_bytes();
        let particle_bytes = data.particle_count.to_le_bytes();

        // Sammle alle Bytes
        let mut all_bytes = Vec::new();
        all_bytes.extend_from_slice(&timestamp_bytes);
        all_bytes.extend_from_slice(&radiation_bytes);
        all_bytes.extend_from_slice(&magnetic_bytes);
        all_bytes.extend_from_slice(&particle_bytes);

        // Füge Bytes aus der Orbitalposition hinzu
        for &pos in &data.orbital_position {
            all_bytes.extend_from_slice(&pos.to_le_bytes());
        }

        // Wende eine einfache Whitening-Funktion an, um die Entropiequalität zu verbessern
        let mut last_byte = 0u8;
        for (i, &byte) in all_bytes.iter().enumerate() {
            // XOR mit dem Index und dem vorherigen Byte für bessere Verteilung
            let whitened = byte ^ (i as u8) ^ last_byte;
            if result.len() < bytes_requested {
                result.push(whitened);
                last_byte = whitened;
            } else {
                break;
            }
        }

        // Fülle mit Zufallsdaten auf, falls nicht genug Bytes vorhanden sind
        while result.len() < bytes_requested {
            // XOR mit Systemzeit für zusätzliche Entropie
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .subsec_nanos();

            let byte = ((now & 0xFF) as u8) ^ last_byte;
            result.push(byte);
            last_byte = byte;
        }

        result
    }
}

#[async_trait]
impl EntropySource for SatelliteDataSource {
    fn name(&self) -> &str {
        &self.name
    }

    fn priority(&self) -> u8 {
        priority::SECONDARY
    }

    async fn is_available(&self) -> bool {
        match self
            .client
            .get(&self.feed_url)
            .timeout(Duration::from_millis(800))
            .send()
            .await
        {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }

    async fn collect_entropy(&self, bytes_requested: usize) -> EntropyResult<Vec<u8>> {
        let satellite_data = self.fetch_satellite_data().await?;
        let entropy = self.extract_entropy_from_satellite(&satellite_data, bytes_requested);

        Ok(entropy)
    }
}

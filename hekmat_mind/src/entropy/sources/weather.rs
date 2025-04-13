//! Wetterdaten-Entropiequelle
//!
//! Implementiert eine Entropiequelle, die Wetterdaten von einer API abruft
//! und daraus Entropie extrahiert. Diese Quelle verwendet Temperatur, Luftdruck
//! und Luftfeuchtigkeit als Entropiequellen.

use crate::entropy::sources::priority;
use crate::entropy::{EntropyError, EntropyResult, EntropySource};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

/// Struktur für die Deserialisierung von Wetterdaten
#[derive(Debug, Deserialize)]
struct WeatherData {
    temperature: f32,
    humidity: f32,
    pressure: f32,
    wind_speed: f32,
    wind_direction: f32,
    precipitation: f32,
    // Weitere Felder je nach API
}

/// Wetterdaten-Entropiequelle
pub struct WeatherDataSource {
    /// Name der Quelle
    name: String,

    /// API-Endpunkt
    api_endpoint: String,

    /// API-Schlüssel
    api_key: String,

    /// HTTP-Client
    client: Client,
}

impl WeatherDataSource {
    /// Erstellt eine neue Wetterdaten-Entropiequelle
    pub fn new(api_endpoint: String, api_key: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        Self {
            name: "Wetterdaten-API".to_string(),
            api_endpoint,
            api_key,
            client,
        }
    }

    /// Ruft Wetterdaten von der API ab
    async fn fetch_weather_data(&self) -> EntropyResult<WeatherData> {
        let url = format!("{}?key={}", self.api_endpoint, self.api_key);

        let response = self.client.get(&url).send().await.map_err(|e| {
            EntropyError::ConnectionError(format!("Fehler beim Abrufen von Wetterdaten: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(EntropyError::ConnectionError(format!(
                "API-Fehler: HTTP {}",
                response.status()
            )));
        }

        response.json::<WeatherData>().await.map_err(|e| {
            EntropyError::ProcessingError(format!(
                "Fehler beim Deserialisieren der Wetterdaten: {}",
                e
            ))
        })
    }

    /// Extrahiert Entropie aus Wetterdaten
    fn extract_entropy_from_weather(&self, data: &WeatherData, bytes_requested: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(bytes_requested);

        // Extrahiere Bytes aus den Gleitkommazahlen
        let mut add_float_bytes = |value: f32| {
            let bytes = value.to_le_bytes();
            for byte in bytes {
                if result.len() < bytes_requested {
                    result.push(byte);
                }
            }
        };

        // Verwende alle verfügbaren Wetterdaten als Entropiequellen
        add_float_bytes(data.temperature);
        add_float_bytes(data.humidity);
        add_float_bytes(data.pressure);
        add_float_bytes(data.wind_speed);
        add_float_bytes(data.wind_direction);
        add_float_bytes(data.precipitation);

        // Fülle mit Zufallsdaten auf, falls nicht genug Bytes vorhanden sind
        while result.len() < bytes_requested {
            // XOR mit Systemzeit für zusätzliche Entropie
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .subsec_nanos();

            let byte = (now & 0xFF) as u8;
            result.push(byte);
        }

        result
    }
}

#[async_trait]
impl EntropySource for WeatherDataSource {
    fn name(&self) -> &str {
        &self.name
    }

    fn priority(&self) -> u8 {
        priority::PRIMARY
    }

    async fn is_available(&self) -> bool {
        match self
            .client
            .get(&self.api_endpoint)
            .timeout(Duration::from_millis(500))
            .send()
            .await
        {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }

    async fn collect_entropy(&self, bytes_requested: usize) -> EntropyResult<Vec<u8>> {
        let weather_data = self.fetch_weather_data().await?;
        let entropy = self.extract_entropy_from_weather(&weather_data, bytes_requested);

        Ok(entropy)
    }
}

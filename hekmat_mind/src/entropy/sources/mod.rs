//! Entropiequellen-Implementierungen
//!
//! Dieses Modul enthält Implementierungen für verschiedene Entropiequellen:
//! - Wetterdaten-API (primäre Quelle)
//! - Satellitendaten-Feeds (sekundäre Quelle)
//! - Systemrauschen-Sampling (tertiäre Quelle / Fallback)

pub mod satellite;
pub mod system;
pub mod weather;

// Re-export der Quellen für einfacheren Zugriff
pub use satellite::SatelliteDataSource;
pub use system::SystemNoiseSource;
pub use weather::WeatherDataSource;

/// Prioritätsstufen für Entropiequellen
pub mod priority {
    /// Priorität für primäre Quellen (Wetterdaten)
    pub const PRIMARY: u8 = 1;

    /// Priorität für sekundäre Quellen (Satellitendaten)
    pub const SECONDARY: u8 = 2;

    /// Priorität für tertiäre Quellen (Systemrauschen)
    pub const TERTIARY: u8 = 3;
}

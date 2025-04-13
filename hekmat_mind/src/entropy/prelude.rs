//! Prelude-Modul für Entropiequellen
//!
//! Dieses Modul exportiert häufig verwendete Typen und Traits
//! für die Arbeit mit Entropiequellen.

pub use crate::entropy::{
    EntropyConfig, EntropyError, EntropyManager, EntropyResult, EntropySource,
};

pub use crate::entropy::cache::EntropyCache;
pub use crate::entropy::extractors::{BitExtractor, CombinedExtractor};
pub use crate::entropy::sources::{
    SatelliteDataSource, SystemNoiseSource, WeatherDataSource, priority,
};

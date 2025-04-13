//! Neuronales Wachstumsmodul
//!
//! Dieses Modul implementiert biologisch inspirierte Wachstumsmechanismen
//! für Neuronen mit Fokus auf empirischer Validierbarkeit.

pub mod axon;
pub mod dendritic_growth;
pub mod types;

pub use axon::AxonGrowth;
pub use dendritic_growth::{
    DendriteResourceManager, DendriticSegment, DendriticTree, NeuralGrowth, Synapse, SynapseState,
};
pub use types::Position;

// Re-export von Typen für einfacheren Zugriff
pub use axon::{FactorType, GrowthFactor, GrowthMeasurement};

#[cfg(test)]
mod tests;

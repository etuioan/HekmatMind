//! # Synapsen-Modul
//!
//! Dieses Modul implementiert synaptische Verbindungen zwischen Neuronen,
//! mit Unterstützung für synaptische Plastizität (Hebbsches Lernen).
//!
//! ## Überblick
//!
//! Synapsen sind die Verbindungspunkte zwischen Neuronen und bestimmen, wie Signale
//! von einem Neuron zum anderen fließen. Sie haben folgende Eigenschaften:
//!
//! - Verbinden jeweils ein präsynaptisches mit einem postsynaptischen Neuron
//! - Modulieren das Signal durch ein Gewicht (Stärke der Verbindung)
//! - Fügen eine Verzögerung bei der Signalübertragung ein
//! - Passen ihre Stärke durch Hebbsches Lernen an (synaptische Plastizität)
//!
//! ## Biologische Inspiration
//!
//! Das Modell basiert auf biologischen Synapsen, verwendet aber Vereinfachungen für
//! Effizienz. Wir modellieren sowohl erregende als auch hemmende Effekte
//! durch positive und negative Gewichte.

pub mod model;
pub mod tests;

pub use model::{Synapse, SynapseBuilder, constants};

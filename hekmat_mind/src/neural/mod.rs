//! # Neuronales System
//!
//! Dieses Modul implementiert biologisch inspirierte neuronale Komponenten,
//! die als Grundbausteine für emergentes Verhalten im HekmatMind-Framework dienen.
//!
//! ## Komponenten
//!
//! ### Neuronen
//!
//! Die [`Neuron`]-Struktur modelliert ein einzelnes Neuron mit folgenden Eigenschaften:
//!
//! - Biologisch inspirierte Zustände (inaktiv, aktiv, refraktär)
//! - Adaptive Schwellwerte durch Plastizität
//! - Geschwindigkeits- und kapazitätsbasierte Informationsverarbeitung
//!
//! ### Synapsen
//!
//! Die [`Synapse`]-Struktur modelliert die Verbindung zwischen Neuronen:
//!
//! - Gewichtete Signalübertragung zwischen Neuronen
//! - Bidirektionale Plastizität (Hebbsches Lernen)
//! - Realistische Signalverzögerung
//!
//! ### Neuronale Netzwerke
//!
//! Das [`Network`]-Modul verbindet Neuronen und Synapsen zu funktionalen Einheiten:
//!
//! - Signalpropagation zwischen verbundenen Neuronen
//! - Hebbsche Plastizität auf Netzwerkebene
//! - Aufbau komplexer neuronaler Strukturen
//!
//! ### Geplante Komponenten
//!
//! - Neuronale Schichten für organisierte Informationsverarbeitung
//! - Neuronale Netzwerke für komplexe kognitive Funktionen
//!
//! ## Biologische Inspiration
//!
//! Die Implementierung orientiert sich an:
//!
//! - Refraktärphasen biologischer Neuronen
//! - Homöostatische Plastizität für langfristige Stabilität
//! - Energieeffizienz durch adaptive Schwellwerte
//!
//! ## Beispiel
//!
//! ```rust
//! use hekmat_mind::Neuron;
//!
//! // Neuron mit mittlerer Geschwindigkeit erstellen
//! let mut neuron = Neuron::new(500);
//!
//! // Mehrere Eingabesignale senden
//! neuron.receive_input(0.3);
//! neuron.receive_input(0.3);
//!
//! // Aktivierungszyklus durchführen
//! let output = neuron.cycle();
//! ```

pub mod growth;
pub mod network;
pub mod neuron;
pub mod synapse;

pub use network::model::Network;
pub use network::model::NetworkBuilder;
pub use neuron::model::Neuron;
pub use neuron::model::NeuronState;
pub use synapse::model::Synapse;

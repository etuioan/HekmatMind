//! # Neuron-Modul
//!
//! Dieses Modul implementiert ein biologisch inspiriertes Neuron-Modell,
//! das die Grundlage für neuronale Verarbeitung im HekmatMind-Framework bildet.
//!
//! ## Modellierung
//!
//! Das Neuron-Modell ist eine vereinfachte Repräsentation biologischer Neuronen mit:
//!
//! - Zuständen: inaktiv, aktiv und refraktär
//! - Aktivierungsschwellwert mit homöostatischer Plastizität
//! - Geschwindigkeitsbasierter Informationsverarbeitung
//! - Einzigartiger Identifikation durch UUIDs
//!
//! ## Lebenszyklus eines Neurons
//!
//! 1. **Inaktiver Zustand**: Das Neuron sammelt Energie durch Eingabesignale
//! 2. **Aktivierung**: Wenn die Energie den Schwellwert überschreitet, wird das Neuron aktiviert
//! 3. **Signalaussendung**: Während des Aktivierungszyklus sendet das Neuron ein Signal
//! 4. **Refraktärphase**: Nach der Aktivierung tritt das Neuron in eine Erholungsphase ein
//! 5. **Rückkehr**: Nach einem weiteren Zyklus kehrt das Neuron zum inaktiven Zustand zurück
//!
//! ## Plastizität
//!
//! Neuronen passen ihren Schwellwert basierend auf ihrer Aktivitätsrate an:
//!
//! - Bei übermäßiger Aktivität erhöht sich der Schwellwert
//! - Bei zu geringer Aktivität sinkt der Schwellwert
//!
//! Diese homöostatische Plastizität sorgt für ein ausgewogenes Aktivitätsniveau.
//!
//! ## Beispiel
//!
//! ```rust
//! use hekmat_mind::Neuron;
//!
//! // Neuron mit angepassten Parametern erstellen
//! let mut neuron = Neuron::with_params(
//!     500,      // Geschwindigkeit
//!     0.7,      // Aktivierungsschwellwert
//!     0.05,     // Plastizitätsrate
//! );
//!
//! // Aktivieren und einen Zyklus durchführen
//! neuron.receive_input(1.0);  // Überschreitet den Schwellwert
//! let output = neuron.cycle(); // Gibt die Aktivierungsenergie aus
//!
//! // Schwellwert anpassen
//! neuron.adapt_threshold(true, 0.2); // Zu aktiv, Schwellwert erhöhen
//! ```

pub mod model;
pub mod tests;

// Re-exportiere die Kernkomponenten
pub use model::Neuron;
pub use model::NeuronState;
pub use model::constants;

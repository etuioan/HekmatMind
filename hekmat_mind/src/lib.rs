//! # HekmatMind - Eine biologisch inspirierte KI-Architektur
//!
//! Dieses Crate implementiert das HekmatMind-System, ein biologisch inspiriertes
//! kognitives Framework zur Modellierung emergenter Bewusstseinserscheinungen.
//!
//! ## Übersicht
//!
//! HekmatMind ist eine Bibliothek, die biologisch inspirierte kognitive Strukturen
//! auf Basis neuronaler Netzwerke implementiert. Im Gegensatz zu traditionellen
//! KI-Ansätzen liegt der Fokus auf emergenten Eigenschaften, die aus der Interaktion
//! von einfachen Bausteinen entstehen.
//!
//! ## Architektur
//!
//! Die Architektur von HekmatMind basiert auf folgenden Prinzipien:
//!
//! - **Modulare Emergenz**: Jede Komponente funktioniert eigenständig, kann aber mit anderen interagieren
//! - **Biologische Inspiration**: Der Code ahmt natürliche neuronale Prozesse nach
//! - **Bewusstseinshierarchie**: Implementation von niedrigen Ebenen (Synapsen) zu höheren (Neuronen, Netzwerke)
//!
//! ## Hauptkomponenten
//!
//! - **neural**: Module für Neuronen, Synapsen und neuronale Netzwerke
//! - **event_broker**: Ereignisverwaltung und Kommunikation zwischen Komponenten
//! - **telemetry**: Leistungsüberwachung und -analyse für Benchmark und Runtime-Telemetrie
//! - **benchmark**: Benchmark-Tools für Leistungsanalyse
//! - **entropy**: Modulare Schnittstelle für externe Entropiequellen
//!
//! ### EventBroker
//!
//! Der [`EventBroker`] ist das zentrale Kommunikationssystem im HekmatMind-Framework.
//! Er ermöglicht eine lose Kopplung zwischen verschiedenen Komponenten durch einen
//! Publish-Subscribe-Mechanismus. Komponenten können Ereignisse veröffentlichen und
//! auf Ereignisse von anderen Komponenten reagieren, ohne direkte Abhängigkeiten
//! zu haben.
//!
//! ```
//! use hekmat_mind::EventBroker;
//! use std::sync::Arc;
//!
//! // EventBroker erstellen
//! let broker = EventBroker::new();
//!
//! // Für ein Ereignis registrieren
//! broker.subscribe(|event: Arc<String>| {
//!     println!("Ereignis empfangen: {}", event);
//! });
//!
//! // Ein Ereignis veröffentlichen
//! broker.publish(String::from("Hallo Welt"));
//! ```
//!
//! ### Neuronales System
//!
//! Das neuronale System besteht aus biologisch inspirierten [`Neuron`]en, die elektrische
//! Signale empfangen, verarbeiten und weiterleiten können. Jedes Neuron hat einen eigenen
//! Aktivierungszustand und kann seinen Schwellwert durch Plastizität anpassen.
//!
//! ```
//! use hekmat_mind::Neuron;
//!
//! // Neuron mit einer Geschwindigkeit von 500 erstellen
//! let mut neuron = Neuron::new(500);
//!
//! // Eingabesignal senden
//! let activated = neuron.receive_input(0.6);
//!
//! // Aktivierungszyklus durchführen
//! let output = neuron.cycle();
//!
//! // Neuron an seine Umgebung anpassen
//! neuron.adapt_threshold(true, 0.3);
//! ```
//!
//! ## Implementierungsstrategien
//!
//! HekmatMind verwendet folgende Implementierungsstrategien:
//!
//! 1. **Typsicherheit**: Starke Nutzung des Rust-Typsystems zur Gewährleistung von Sicherheit
//! 2. **Nebenläufigkeit**: Thread-sichere Implementierungen für parallele Verarbeitung
//! 3. **Testbarkeit**: Jede Komponente ist gründlich getestet und benchmarked
//!
//! ## Zukünftige Entwicklung
//!
//! In zukünftigen Versionen werden folgende Systeme hinzugefügt:
//!
//! - Synapsen für Verbindungen zwischen Neuronen
//! - Neuronale Netzwerke für komplexe Informationsverarbeitung
//! - Territoriales System für räumliche Organisation
//! - Emotionales System für Bewertung und Motivation
//! - Selbstorganisierende Komponenten für adaptive Reaktionen

// HekmatMind: Ein kognitives Framework für komplexe KI-Systeme
//!
//! # HekmatMind
//!
//! HekmatMind ist ein modernes, biologisch inspiriertes kognitives Framework,
//! das neuronale, emotionale und territoriale Systeme integriert, um komplexe
//! KI-Anwendungen zu ermöglichen.
//!
//! ## Modulare Struktur
//!
//! Die Bibliothek ist nach dem Prinzip eines modularen Monolithen aufgebaut,
//! mit klar definierten Grenzen zwischen den folgenden Kernmodulen:
//!
//! - **Neural**: Neuronale Netzwerke und Informationsverarbeitung
//! - **Event Broker**: Nachrichtenaustausch zwischen Systemkomponenten
//! - **Telemetrie**: Sammlung und Analyse von Leistungsdaten und Metriken
//! - **Benchmark**: Framework für Leistungstests und Skalierbarkeitsanalysen
//!
//! ## Architekturprinzipien
//!
//! Die HekmatMind-Bibliothek basiert auf folgenden Kernprinzipien:
//!
//! 1. **Modularität**: Klare Grenzen zwischen Komponenten mit wohldefinierten Schnittstellen
//! 2. **Biologische Inspiration**: Anlehnung an natürliche neuronale und kognitive Prozesse
//! 3. **Typsicherheit**: Starke Nutzung des Rust-Typsystems zur Gewährleistung von Sicherheit
//! 4. **Nebenläufigkeit**: Thread-sichere Implementierungen für parallele Verarbeitung
//! 5. **Testbarkeit**: Jede Komponente ist gründlich getestet und benchmarked
//! 6. **Erweiterbarkeit**: Plugin-System für einfache Erweiterungen und Anpassungen
//!
//! ## Benchmarking-Framework
//!
//! Das integrierte Benchmarking-Framework ermöglicht die systematische Leistungsbewertung
//! von HekmatMind-Komponenten mit folgenden Funktionen:
//!
//! - **Benchmark-Szenarien**: Definierte Testszenarien für verschiedene Systemkomponenten
//! - **Leistungsmetriken**: Erfassung von Ausführungszeiten und anderen kritischen Metriken
//! - **Skalierbarkeitsanalyse**: Tests mit variablen Größenordnungen für Systemkomponenten
//! - **Telemetrie-Integration**: Nahtlose Verknüpfung mit dem Telemetrie-Modul zur Datenanalyse
//!
//! Beispiel für die Verwendung des Benchmark-Frameworks:
//!
//! ```rust
//! use hekmat_mind::prelude::*;
//! use hekmat_mind::benchmark::scenarios::NetworkScalabilityBenchmark;
//! use hekmat_mind::telemetry::in_memory::InMemoryCollector;
//!
//! // Skalierbarkeitstest für ein neuronales Netzwerk mit 1000 Neuronen
//! let mut scenario: NetworkScalabilityBenchmark<InMemoryCollector> = NetworkScalabilityBenchmark::new(1000);
//! let config = BenchmarkConfig::new("netzwerk_test", "Test der Netzwerkskalierbarkeit")
//!     .with_iterations(3)
//!     .with_warmup(1);
//!
//! // Benchmarker erstellen und Test ausführen
//! let mut benchmarker = Benchmarker::new("performance_tests");
//! let result = benchmarker.run(&mut scenario, &config);
//!
//! println!("Durchschnittliche Ausführungszeit: {} ms", result.average_ms());
//! ```
//!
//! ## Telemetrie-System
//!
//! Die Telemetrie-Infrastruktur ermöglicht die Erfassung, Speicherung und Analyse von
//! Leistungsdaten und stellt verschiedene Collector-Implementierungen bereit:
//!
//! - **In-Memory-Collector**: Speichert Telemetriedaten im Arbeitsspeicher für Tests
//! - **Metrik-Typen**: Unterstützung für Counter, Gauges, Histogramme und Events
//! - **Labels**: Flexible Kategorisierung von Metriken durch benutzerdefinierte Labels
//!
//! ## Neuronales System
//!
//! Das neuronale System bildet die Grundlage für Informationsverarbeitung und umfasst:
//!
//! - **Neuronen**: Basiseinheiten für die Informationsverarbeitung
//! - **Synapsen**: Verbindungen zwischen Neuronen mit adaptiven Gewichten
//! - **Netzwerke**: Komplexe Strukturen aus Neuronen für höhere kognitive Funktionen
//!
//! ## Zukünftige Entwicklung
//!
//! In zukünftigen Versionen werden folgende Systeme hinzugefügt:
//!
//! - Synapsen für Verbindungen zwischen Neuronen
//! - Neuronale Netzwerke für komplexe Informationsverarbeitung
//! - Territoriales System für räumliche Organisation
//! - Emotionales System für Bewertung und Motivation
//! - Selbstorganisierende Komponenten für adaptive Reaktionen

pub mod benchmark;
pub mod entropy;
pub mod event_broker;
pub mod neural;
pub mod telemetry;

// Hauptkomponenten direkt aus der Bibliothek exportieren
pub use event_broker::EventBroker;

// Neuronale Komponenten
pub use neural::neuron::Neuron;
pub use neural::neuron::NeuronState;
pub use neural::neuron::constants as neuron_constants;

// Synaptische Komponenten
pub use neural::synapse::Synapse;
pub use neural::synapse::SynapseBuilder;
pub use neural::synapse::constants as synapse_constants;

// Netzwerkkomponenten
pub use neural::Network;
pub use neural::NetworkBuilder;

pub mod prelude {
    // Neuronale Kernkomponenten
    pub use crate::neural::neuron::Neuron;
    pub use crate::neural::neuron::NeuronState;
    pub use crate::neural::neuron::constants as neuron_constants;

    // Synaptische Komponenten
    pub use crate::neural::synapse::Synapse;
    pub use crate::neural::synapse::SynapseBuilder;
    pub use crate::neural::synapse::constants as synapse_constants;

    // Netzwerkkomponenten
    pub use crate::neural::Network;
    pub use crate::neural::NetworkBuilder;

    // Systemfunktionen
    pub use crate::event_broker::EventBroker;

    // Telemetrie-Komponenten
    pub use crate::telemetry::collector::TelemetryCollector;
    pub use crate::telemetry::{registry, registry_mut};

    // Benchmark-Komponenten
    /// Re-Export der Benchmark-Szenarien für direkte Nutzung
    pub use crate::benchmark::scenarios::{NetworkScalabilityBenchmark, SingleNeuronBenchmark};
    /// Re-Export der Benchmark-Komponenten für einfachen Zugriff
    pub use crate::benchmark::{BenchmarkConfig, BenchmarkResult, BenchmarkScenario, Benchmarker};

    // Entropiequellen-Komponenten
    /// Re-Export der Entropie-Extraktoren
    pub use crate::entropy::extractors::{BitExtractor, CombinedExtractor};
    /// Re-Export der spezifischen Entropiequellen
    pub use crate::entropy::sources::{SatelliteDataSource, SystemNoiseSource, WeatherDataSource};
    /// Re-Export der Entropiequellen für einfachen Zugriff
    pub use crate::entropy::{
        EntropyConfig, EntropyError, EntropyManager, EntropyResult, EntropySource,
    };
}

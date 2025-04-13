// Benchmark-Szenarien für HekmatMind
//
// Dieses Modul definiert spezifische Testszenarien für Leistungsmessungen
// der neuronalen Komponenten in verschiedenen Konfigurationen.

use std::collections::HashMap;
use std::time::Instant;

use rand;

use super::BenchmarkScenario;

use crate::neural::neuron::Neuron;
use crate::telemetry::TelemetryRegistry;
use crate::telemetry::collector::TelemetryCollector;

/// Benchmark für einzelne Neuronen-Verarbeitung
pub struct SingleNeuronBenchmark {
    /// Zu testendes Neuron
    neuron: Neuron,
    /// Anzahl der Neuronen-Zyklen pro Iteration
    cycles_per_iteration: usize,
    /// Eingabewert für das Neuron
    input_value: f32,
}

impl SingleNeuronBenchmark {
    /// Erstellt einen neuen Neuronen-Benchmark mit Standardwerten
    pub fn new(speed: u16) -> Self {
        SingleNeuronBenchmark {
            neuron: Neuron::new(speed),
            cycles_per_iteration: 1000,
            input_value: 0.5,
        }
    }

    /// Konfiguriert die Anzahl der Zyklen pro Iteration
    pub fn with_cycles(mut self, cycles: usize) -> Self {
        self.cycles_per_iteration = cycles;
        self
    }

    /// Konfiguriert den Eingabewert für das Neuron
    pub fn with_input(mut self, input: f32) -> Self {
        self.input_value = input;
        self
    }
}

impl BenchmarkScenario for SingleNeuronBenchmark {
    fn name(&self) -> &str {
        "single_neuron_processing"
    }

    fn description(&self) -> &str {
        "Misst die Verarbeitungsgeschwindigkeit eines einzelnen Neurons"
    }

    fn setup(&mut self) {
        // Neuron zurücksetzen
        self.neuron = Neuron::new(self.neuron.speed());
    }

    fn run_iteration(&mut self) {
        // Neuronen-Zyklen ausführen
        for _ in 0..self.cycles_per_iteration {
            self.neuron.receive_input(self.input_value);
            let output = self.neuron.cycle();

            // Aktivität in Telemetrie erfassen
            if let Ok(reg) = crate::telemetry::registry() {
                let mut labels = self.telemetry_labels();
                labels.insert("neuron_speed".to_string(), self.neuron.speed().to_string());

                reg.record_gauge(
                    "neural",
                    "neuron_output",
                    output as f64, // Konvertierung zu f64 für Telemetrie
                    Some(labels),
                );
            }
        }
    }

    fn telemetry_labels(&self) -> HashMap<String, String> {
        let mut labels = HashMap::new();
        labels.insert("benchmark".to_string(), self.name().to_string());
        labels.insert("cycles".to_string(), self.cycles_per_iteration.to_string());
        labels
    }
}

/// Einfacher Netzwerk-Stub für Benchmarks
///
/// Diese Implementierung wird für Benchmarks verwendet, solange das
/// vollständige Netzwerk-Modul noch nicht implementiert ist.
pub struct Network {
    #[allow(dead_code)]
    name: String,
    neurons: Vec<Neuron>,
    connections: Vec<(usize, usize, f32)>, // (Quelle, Ziel, Stärke)
}

impl Network {
    /// Erstellt ein neues Netzwerk mit dem angegebenen Namen
    pub fn new(name: &str) -> Self {
        Network {
            name: name.to_string(),
            neurons: Vec::new(),
            connections: Vec::new(),
        }
    }

    /// Fügt ein Neuron zum Netzwerk hinzu
    pub fn add_neuron(&mut self, neuron: Neuron) {
        self.neurons.push(neuron);
    }

    /// Verbindet zwei Neuronen miteinander
    pub fn connect_neurons(&mut self, source: usize, target: usize, strength: f32) {
        if source < self.neurons.len() && target < self.neurons.len() {
            self.connections.push((source, target, strength));
        }
    }

    /// Sendet einen Eingabewert an ein Neuron
    pub fn send_input(&mut self, neuron_idx: usize, value: f32) {
        if neuron_idx < self.neurons.len() {
            self.neurons[neuron_idx].receive_input(value);
        }
    }

    /// Führt einen Verarbeitungszyklus für das gesamte Netzwerk durch
    pub fn cycle(&mut self) -> usize {
        // Alle Neuronen verarbeiten ihre Eingaben
        let mut outputs = Vec::with_capacity(self.neurons.len());

        for neuron in &mut self.neurons {
            outputs.push(neuron.cycle());
        }

        // Signale über Verbindungen weitergeben
        for (source, target, strength) in &self.connections {
            let input = outputs[*source] * strength;
            if *target < self.neurons.len() {
                self.neurons[*target].receive_input(input);
            }
        }

        // Anzahl der aktiven Neuronen zurückgeben
        outputs.iter().filter(|&&output| output > 0.0).count()
    }

    /// Gibt die Anzahl der Neuronen im Netzwerk zurück
    pub fn neuron_count(&self) -> usize {
        self.neurons.len()
    }
}

/// Dieses Szenario misst, wie effizient das Netzwerk große Mengen an Neuronen verarbeiten kann.
/// Es erstellt ein Netzwerk mit einer festgelegten Anzahl an Neuronen und führt eine bestimmte
/// Anzahl an Zyklen durch.
///
/// Gemessen wird dabei die durchschnittliche Dauer pro Zyklus, was einen Einblick in die
/// Skalierbarkeit des Netzwerks gibt.
pub struct NetworkScalabilityBenchmark<R = TelemetryRegistry> {
    /// Größe des Netzwerks (Anzahl der Neuronen)
    network_size: usize,

    /// Anzahl der Zyklen pro Iteration
    cycles_per_iteration: usize,

    /// Innere Struktur des Netzwerks (wird dynamisch erstellt)
    network: Option<Network>,

    /// Eine benutzerdefinierte Registry, die für Tests verwendet werden kann
    /// Dies ermöglicht isolierte Tests, ohne die globale Registry zu beeinflussen
    custom_registry: Option<R>,
}

impl<R> NetworkScalabilityBenchmark<R>
where
    R: TelemetryCollector + Clone + Send + Sync,
{
    /// Erstellt ein neues Benchmark-Szenario mit der angegebenen Netzwerkgröße.
    ///
    /// # Argumente
    ///
    /// * `network_size` - Die Anzahl der Neuronen im Netzwerk
    pub fn new(network_size: usize) -> Self {
        NetworkScalabilityBenchmark {
            network_size,
            cycles_per_iteration: 1000,
            network: None,
            custom_registry: None,
        }
    }

    /// Setzt die Anzahl der Zyklen pro Iteration
    ///
    /// # Argumente
    ///
    /// * `cycles` - Die Anzahl der Zyklen, die in jeder Iteration durchgeführt werden sollen
    ///
    /// # Rückgabe
    ///
    /// Eine neue Benchmark-Instanz mit der aktualisierten Zyklenanzahl
    pub fn with_cycles(mut self, cycles: usize) -> Self {
        self.cycles_per_iteration = cycles;
        self
    }

    /// Setzt eine benutzerdefinierte Telemetrie-Registry
    ///
    /// # Argumente
    ///
    /// * `registry` - Die zu verwendende Registry
    ///
    /// # Rückgabe
    ///
    /// Eine neue Benchmark-Instanz mit der benutzerdefinierten Registry
    pub fn with_registry(mut self, registry: R) -> Self {
        self.custom_registry = Some(registry);
        self
    }

    /// Gibt die benutzerdefinierte Registry zurück, falls vorhanden
    ///
    /// # Rückgabe
    ///
    /// Eine Referenz auf die benutzerdefinierte Registry, falls vorhanden
    #[cfg(test)]
    pub fn get_registry(&self) -> Option<&R> {
        self.custom_registry.as_ref()
    }

    /// Nimmt die benutzerdefinierte Registry aus dem Benchmark
    ///
    /// # Rückgabe
    ///
    /// Die benutzerdefinierte Registry, falls vorhanden
    pub fn take_registry(&mut self) -> Option<R> {
        self.custom_registry.take()
    }
}

impl<R> BenchmarkScenario for NetworkScalabilityBenchmark<R>
where
    R: TelemetryCollector + Clone + Send + Sync,
{
    fn name(&self) -> &str {
        "network_scalability"
    }

    fn description(&self) -> &str {
        "Misst die Skalierbarkeit des neuronalen Netzwerks mit unterschiedlichen Neuronenzahlen"
    }

    fn setup(&mut self) {
        // Neues Netzwerk erstellen
        let mut network = Network::new(&format!("benchmark_network_{}", self.network_size));

        // Neuronen mit verschiedenen Geschwindigkeiten hinzufügen
        for i in 0..self.network_size {
            let speed = 200_u16.saturating_add((i % 800) as u16); // Neuronen mit unterschiedlichen Geschwindigkeiten
            let neuron = Neuron::new(speed);
            network.add_neuron(neuron);
        }

        // Zufällige Verbindungen zwischen Neuronen herstellen (ca. 10% Vernetzung)
        if self.network_size > 1 {
            let connection_count = (self.network_size * self.network_size / 10).max(1);

            for _ in 0..connection_count {
                let source = rand::random::<usize>() % self.network_size;
                let mut target = rand::random::<usize>() % self.network_size;

                // Vermeidet Selbstverbindungen
                while target == source {
                    target = rand::random::<usize>() % self.network_size;
                }

                // Verbindungsstärke zwischen 0.1 und 1.0
                let strength = 0.1 + rand::random::<f32>() * 0.9;

                // Verbindung herstellen
                network.connect_neurons(source, target, strength);
            }
        }

        self.network = Some(network);
    }

    fn teardown(&mut self) {
        // Netzwerk freigeben
        self.network = None;
    }

    fn run_iteration(&mut self) {
        // Telemetrie-Labels außerhalb der Network-Verwendung erstellen
        let benchmark_name = self.name().to_string();
        let neuron_count = self.network_size;
        let cycles = self.cycles_per_iteration;

        let mut base_labels = HashMap::new();
        base_labels.insert("benchmark".to_string(), benchmark_name);
        base_labels.insert("neuron_count".to_string(), neuron_count.to_string());
        base_labels.insert("cycles".to_string(), cycles.to_string());

        // Netzwerkzyklen ausführen
        if let Some(network) = &mut self.network {
            for i in 0..self.cycles_per_iteration {
                // Zufallseingaben an 10% der Neuronen
                let input_count = (self.network_size / 10).max(1);

                for _ in 0..input_count {
                    let target = rand::random::<usize>() % self.network_size;
                    let input_value = rand::random::<f32>();

                    network.send_input(target, input_value);
                }

                // Netzwerkzyklus ausführen
                let start_time = Instant::now();
                let active_neurons = network.cycle(); // Kein time_step-Parameter
                let cycle_duration = start_time.elapsed();

                // Telemetrie aufzeichnen
                if let Some(ref mut test_registry) = self.custom_registry {
                    let mut cycle_labels = base_labels.clone();
                    cycle_labels.insert("cycle".to_string(), i.to_string());

                    // Dauer des Netzwerkzyklus in Mikrosekunden
                    test_registry.record_histogram(
                        "network",
                        "cycle_duration_us",
                        cycle_duration.as_micros() as f64,
                        Some(cycle_labels.clone()),
                    );

                    // Anzahl aktiver Neuronen
                    test_registry.record_gauge(
                        "network",
                        "active_neurons",
                        active_neurons as f64, // Konvertierung zu f64 für Telemetrie
                        Some(cycle_labels),
                    );
                } else if let Ok(reg) = crate::telemetry::registry() {
                    let mut cycle_labels = base_labels.clone();
                    cycle_labels.insert("cycle".to_string(), i.to_string());

                    // Dauer des Netzwerkzyklus in Mikrosekunden
                    reg.record_histogram(
                        "network",
                        "cycle_duration_us",
                        cycle_duration.as_micros() as f64,
                        Some(cycle_labels.clone()),
                    );

                    // Anzahl aktiver Neuronen
                    reg.record_gauge(
                        "network",
                        "active_neurons",
                        active_neurons as f64, // Konvertierung zu f64 für Telemetrie
                        Some(cycle_labels),
                    );
                }
            }
        }
    }

    fn telemetry_labels(&self) -> HashMap<String, String> {
        let mut labels = HashMap::new();
        labels.insert("benchmark".to_string(), self.name().to_string());
        labels.insert("neuron_count".to_string(), self.network_size.to_string());
        labels.insert("cycles".to_string(), self.cycles_per_iteration.to_string());
        labels
    }
}

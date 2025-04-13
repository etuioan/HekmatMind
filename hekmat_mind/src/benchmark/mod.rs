// Benchmark-Framework für HekmatMind
//
// Dieses Modul implementiert eine erweiterbare Infrastruktur für Leistungstests,
// die eng mit der Telemetrie-Architektur integriert ist.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::telemetry::registry;

/// Benchmark-Konfiguration
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// Name des Benchmarks
    pub name: String,
    /// Beschreibung des Benchmarks
    pub description: String,
    /// Anzahl der Wiederholungen
    pub iterations: usize,
    /// Aufwärmzyklus vor Beginn der Messungen
    pub warmup_iterations: usize,
    /// Zusätzliche Konfigurationsparameter
    pub parameters: HashMap<String, String>,
}

impl BenchmarkConfig {
    /// Erstellt eine neue Benchmark-Konfiguration mit Standardwerten
    pub fn new(name: &str, description: &str) -> Self {
        BenchmarkConfig {
            name: name.to_string(),
            description: description.to_string(),
            iterations: 10,
            warmup_iterations: 3,
            parameters: HashMap::new(),
        }
    }

    /// Fügt einen Konfigurationsparameter hinzu
    pub fn with_param(mut self, key: &str, value: &str) -> Self {
        self.parameters.insert(key.to_string(), value.to_string());
        self
    }

    /// Setzt die Anzahl der Wiederholungen
    pub fn with_iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }

    /// Setzt die Anzahl der Aufwärmzyklen
    pub fn with_warmup(mut self, warmup: usize) -> Self {
        self.warmup_iterations = warmup;
        self
    }
}

/// Ergebnis eines einzelnen Benchmark-Laufs
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Name des Benchmarks
    pub name: String,
    /// Beschreibung des Benchmarks
    pub description: String,
    /// Zeitpunkt des Benchmark-Starts
    pub start_time: Instant,
    /// Dauer des Benchmarks
    pub total_duration: Duration,
    /// Einzelne Iterations-Ergebnisse in Millisekunden
    pub iteration_results: Vec<f64>,
    /// Zusätzliche Metriken aus der Telemetrie
    pub metrics: HashMap<String, Vec<f64>>,
    /// Verwendete Konfiguration
    pub config: BenchmarkConfig,
}

impl BenchmarkResult {
    /// Berechnet die durchschnittliche Ausführungszeit in Millisekunden
    pub fn average_ms(&self) -> f64 {
        if self.iteration_results.is_empty() {
            return 0.0;
        }
        self.iteration_results.iter().sum::<f64>() / self.iteration_results.len() as f64
    }

    /// Berechnet die minimale Ausführungszeit in Millisekunden
    pub fn min_ms(&self) -> f64 {
        self.iteration_results
            .iter()
            .fold(f64::MAX, |a, &b| a.min(b))
    }

    /// Berechnet die maximale Ausführungszeit in Millisekunden
    pub fn max_ms(&self) -> f64 {
        self.iteration_results.iter().fold(0.0, |a, &b| a.max(b))
    }

    /// Berechnet die Standardabweichung der Ausführungszeit
    pub fn std_dev_ms(&self) -> f64 {
        if self.iteration_results.len() <= 1 {
            return 0.0;
        }

        let avg = self.average_ms();
        let variance = self
            .iteration_results
            .iter()
            .map(|&x| (x - avg).powi(2))
            .sum::<f64>()
            / (self.iteration_results.len() - 1) as f64;

        variance.sqrt()
    }
}

impl std::fmt::Display for BenchmarkResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Benchmark: {}", self.name)?;
        writeln!(f, "Beschreibung: {}", self.description)?;
        writeln!(f, "Iterationen: {}", self.iteration_results.len())?;
        writeln!(f, "Durchschnitt: {:.3} ms", self.average_ms())?;
        writeln!(f, "Min: {:.3} ms", self.min_ms())?;
        writeln!(f, "Max: {:.3} ms", self.max_ms())?;
        writeln!(f, "Std.Abw.: {:.3} ms", self.std_dev_ms())?;

        Ok(())
    }
}

/// Definition eines Benchmark-Szenarios
pub trait BenchmarkScenario: Send + Sync {
    /// Name des Szenarios
    fn name(&self) -> &str;

    /// Beschreibung des Szenarios
    fn description(&self) -> &str;

    /// Initialisierung vor dem Benchmark
    fn setup(&mut self) {}

    /// Bereinigung nach dem Benchmark
    fn teardown(&mut self) {}

    /// Ausführung eines einzelnen Benchmark-Schritts
    fn run_iteration(&mut self);

    /// Generiert Telemetrie-Labels für dieses Szenario
    fn telemetry_labels(&self) -> HashMap<String, String> {
        let mut labels = HashMap::new();
        labels.insert("benchmark".to_string(), self.name().to_string());
        labels
    }
}

/// Benchmarker für die Ausführung von Leistungstests
pub struct Benchmarker {
    /// Eindeutiger Name des Benchmarkers
    name: String,
}

impl Benchmarker {
    /// Erstellt einen neuen Benchmarker
    pub fn new(name: &str) -> Self {
        Benchmarker {
            name: name.to_string(),
        }
    }

    /// Führt ein Benchmark-Szenario mit der angegebenen Konfiguration aus
    pub fn run<T: BenchmarkScenario>(
        &self,
        scenario: &mut T,
        config: &BenchmarkConfig,
    ) -> BenchmarkResult {
        println!(
            "Starte Benchmark: {} - {}",
            scenario.name(),
            scenario.description()
        );

        // Initialisierung
        scenario.setup();

        // Telemetrie-Labels für diesen Benchmark
        let mut labels = scenario.telemetry_labels();
        labels.insert("benchmarker".to_string(), self.name.clone());

        // Aufwärmphase
        if config.warmup_iterations > 0 {
            println!("Aufwärmphase: {} Iterationen", config.warmup_iterations);
            for i in 0..config.warmup_iterations {
                println!("  Aufwärm-Iteration {}/{}", i + 1, config.warmup_iterations);
                scenario.run_iteration();
            }
        }

        // Hauptmessung
        println!("Hauptmessung: {} Iterationen", config.iterations);

        let start_time = Instant::now();
        let mut iteration_results = Vec::with_capacity(config.iterations);

        for i in 0..config.iterations {
            // Einzeliteration messen
            let iteration_start = Instant::now();

            // Iteration ausführen
            scenario.run_iteration();

            // Ergebnis speichern
            let iteration_duration = iteration_start.elapsed();
            let duration_ms = iteration_duration.as_secs_f64() * 1000.0;
            iteration_results.push(duration_ms);

            // In Telemetrie speichern
            if let Ok(reg) = registry() {
                reg.record_histogram(
                    "benchmark",
                    &format!("{}_iteration", scenario.name()),
                    duration_ms,
                    Some(labels.clone()),
                );
            }

            println!(
                "  Iteration {}/{}: {:.3} ms",
                i + 1,
                config.iterations,
                duration_ms
            );
        }

        let total_duration = start_time.elapsed();

        // Bereinigung
        scenario.teardown();

        // Ergebnis erstellen
        let result = BenchmarkResult {
            name: scenario.name().to_string(),
            description: scenario.description().to_string(),
            start_time,
            total_duration,
            iteration_results,
            metrics: HashMap::new(), // Hier könnten weitere Metriken aus der Telemetrie hinzugefügt werden
            config: config.clone(),
        };

        // Zusammenfassung ausgeben
        println!("{}", result);

        result
    }
}

pub mod scenarios;

#[cfg(test)]
mod tests;

// Netzwerk-Skalierungsbenchmark für HekmatMind
//
// Dieses Programm führt Skalierungstests für das neuronale Netzwerk
// mit verschiedenen Größen (10^2 bis 10^5 Neuronen) durch.

use hekmat_mind::benchmark::scenarios::NetworkScalabilityBenchmark;
use hekmat_mind::prelude::*;
use hekmat_mind::telemetry::TelemetryRegistry;
use hekmat_mind::telemetry::in_memory::InMemoryCollector;

fn main() {
    println!("HekmatMind Netzwerk-Skalierungstest");
    println!("===================================\n");

    // Telemetrie-Umgebung vorbereiten
    if let Ok(mut registry) = registry_mut() {
        registry.clear();

        // Telemetrie-Collector registrieren
        let collector = InMemoryCollector::new(2000);
        registry.register(Box::new(collector));

        // Skalierungstestgrößen definieren
        let network_sizes = [100, 1_000, 5_000, 10_000];

        for &size in &network_sizes {
            println!("\nNetzwerk-Skalierungstest: {} Neuronen", size);

            // Skalierungsbenchmark erstellen und konfigurieren
            let mut scenario =
                NetworkScalabilityBenchmark::<TelemetryRegistry>::new(size).with_cycles(5); // Weniger Zyklen für schnellere Tests

            // Benchmark-Konfiguration erstellen
            let config = BenchmarkConfig::new(
                &format!("network_scalability_{}", size),
                &format!("Netzwerkskalierungstest mit {} Neuronen", size),
            )
            .with_iterations(if size <= 1_000 { 3 } else { 2 })
            .with_warmup(1);

            // Benchmarker erstellen und Benchmark ausführen
            let benchmarker = Benchmarker::new(&format!("network_scalability_{}", size));
            let result = benchmarker.run(&mut scenario, &config);

            // Ergebnisse ausgeben
            println!(
                "Netzwerk mit {} Neuronen: Durchschnitt {:.3} ms, Min {:.3} ms, Max {:.3} ms",
                size,
                result.average_ms(),
                result
                    .iteration_results
                    .iter()
                    .copied()
                    .fold(f64::INFINITY, f64::min),
                result
                    .iteration_results
                    .iter()
                    .copied()
                    .fold(f64::NEG_INFINITY, f64::max)
            );
        }
    } else {
        eprintln!("Fehler: Konnte Telemetrie-Registry nicht abrufen");
    }
}

// Benchmarking-Tests für HekmatMind
//
// Diese Tests validieren die Funktionalität des Benchmark-Frameworks und
// dessen Integration mit der Telemetrie-Infrastruktur.

use std::thread;
use std::time::Duration;

use hekmat_mind::prelude::{BenchmarkConfig, BenchmarkScenario, Benchmarker};
use hekmat_mind::telemetry::{
    collector::QueryableCollector, in_memory::InMemoryCollector, registry, registry_mut,
};

/// Test-Implementation eines Benchmark-Szenarios
#[derive(Debug)]
struct TestBenchmarkScenario {
    name: String,
    description: String,
    iteration_duration_ms: u64,
}

// Implementierung des BenchmarkScenario-Traits für unsere Teststruktur
impl BenchmarkScenario for TestBenchmarkScenario {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn run_iteration(&mut self) {
        // Simuliere Arbeit durch Warten
        thread::sleep(Duration::from_millis(self.iteration_duration_ms));
    }
}

#[test]
fn test_basic_benchmark_with_telemetry() {
    // Telemetrie-Umgebung vorbereiten
    let collector = InMemoryCollector::new(100);

    // Collector in der Registry registrieren
    {
        let mut reg = registry_mut().expect("Registry-Lock fehlgeschlagen");
        reg.clear();
        reg.register(Box::new(collector));
    }

    // Benchmark-Szenario vorbereiten
    let mut scenario = TestBenchmarkScenario {
        name: "basic_test".to_string(),
        description: "Einfacher Funktionstest für das Benchmark-Framework".to_string(),
        iteration_duration_ms: 10,
    };

    // Benchmark-Konfiguration erstellen
    let config = BenchmarkConfig::new(
        "basic_test",
        "Einfacher Funktionstest für das Benchmark-Framework",
    )
    .with_iterations(3)
    .with_warmup(1);

    // Benchmark ausführen
    let benchmarker = Benchmarker::new("test_benchmarker");
    let result = benchmarker.run(&mut scenario, &config);

    // Ergebnisse validieren
    assert_eq!(result.name, "basic_test");
    assert_eq!(result.config.iterations, 3);
    assert_eq!(result.iteration_results.len(), 3);

    // Telemetriedaten überprüfen - korrekte Metrik-Benennung verwenden
    let reg = registry().expect("Registry-Lock fehlgeschlagen");
    let collectors = reg.collectors();
    let collector_ref = collectors
        .first()
        .expect("Kein Collector registriert")
        .as_any()
        .downcast_ref::<InMemoryCollector>()
        .expect("Collector ist kein InMemoryCollector");

    // Metriken mit dem korrekten Namen abfragen (Name + "_iteration")
    let benchmark_stats =
        collector_ref.query_stats("benchmark", &format!("{}_iteration", "basic_test"));
    assert!(
        benchmark_stats.is_some(),
        "Keine Benchmark-Telemetriedaten gefunden"
    );
    assert_eq!(
        benchmark_stats.expect("Stats sollten vorhanden sein").count,
        3,
        "Falsche Anzahl von Telemetriedatenpunkten"
    );
}

#[test]
fn test_performance_measurement() {
    // Telemetrie-Umgebung vorbereiten
    let collector = InMemoryCollector::new(500);

    // Collector in der Registry registrieren
    {
        let mut reg = registry_mut().expect("Registry-Lock fehlgeschlagen");
        reg.clear();
        reg.register(Box::new(collector));
    }

    // Varianten mit unterschiedlichen Zeiten testen
    let scenarios = [("fast", 5), ("medium", 25), ("slow", 50)];

    for (speed, duration_ms) in scenarios {
        let test_name = format!("performance_test_{}", speed);

        // Benchmark-Szenario vorbereiten
        let mut scenario = TestBenchmarkScenario {
            name: test_name.clone(),
            description: format!("Performance-Test mit {} ms Iteration", duration_ms),
            iteration_duration_ms: duration_ms,
        };

        // Benchmark-Konfiguration erstellen
        let config = BenchmarkConfig::new(
            &test_name,
            &format!("Performance-Messung mit {} ms Iterationsdauer", duration_ms),
        )
        .with_iterations(5)
        .with_warmup(2);

        // Benchmark ausführen
        let benchmarker = Benchmarker::new("performance_benchmarker");
        let result = benchmarker.run(&mut scenario, &config);

        // Ergebnisse validieren
        assert_eq!(result.name, test_name);
        assert_eq!(result.config.iterations, 5);
        assert_eq!(result.iteration_results.len(), 5);

        // Ergebnisse überprüfen
        println!(
            "{}: Durchschnitt {:.3} ms, Min {:.3} ms, Max {:.3} ms",
            speed,
            result.average_ms(),
            result.min_ms(),
            result.max_ms()
        );

        // Grundlegende Plausibilitätsprüfungen für die Zeitmessungen
        assert!(
            result.average_ms() >= duration_ms as f64 * 0.5,
            "Durchschnittliche Zeit zu niedrig: {} ms",
            result.average_ms()
        );
        assert!(
            result.min_ms() >= duration_ms as f64 * 0.5,
            "Minimale Zeit zu niedrig: {} ms",
            result.min_ms()
        );
    }

    // Telemetriedaten überprüfen
    let reg = registry().expect("Registry-Lock fehlgeschlagen");
    let collectors = reg.collectors();
    let collector_ref = collectors
        .first()
        .expect("Kein Collector registriert")
        .as_any()
        .downcast_ref::<InMemoryCollector>()
        .expect("Collector ist kein InMemoryCollector");

    // Überprüfen, ob für jedes Szenario Telemetriedaten vorhanden sind
    for (speed, _) in scenarios {
        let test_name = format!("performance_test_{}", speed);
        // Metriken mit dem korrekten Namen abfragen (Name + "_iteration")
        let stats = collector_ref.query_stats("benchmark", &format!("{}_iteration", test_name));
        assert!(
            stats.is_some(),
            "Keine Telemetriedaten für {} gefunden",
            test_name
        );
        assert_eq!(
            stats.expect("Stats sollten vorhanden sein").count,
            5,
            "Falsche Anzahl von Telemetriedatenpunkten für {}",
            test_name
        );
    }
}

#[test]
fn test_network_scalability() {
    // Prüfen, ob mehrere Benchmarks hintereinander ausgeführt werden können
    for size in [10, 50, 100] {
        // In einem echten Test würden wir hier ein neuronales Netz mit
        // der angegebenen Größe erstellen und dessen Leistung messen

        let test_name = format!("network_scalability_{}", size);

        // Simuliertes Netzwerk-Szenario
        let mut scenario = TestBenchmarkScenario {
            name: test_name.clone(),
            description: format!("Netzwerk-Skalierungstest mit {} Neuronen", size),
            iteration_duration_ms: size, // Komplexität steigt mit Größe
        };

        // Benchmark-Konfiguration erstellen
        let config = BenchmarkConfig::new(
            &test_name,
            &format!("Netzwerkskalierungstest mit {} Neuronen", size),
        )
        .with_iterations(2)
        .with_warmup(1);

        // Benchmark ausführen
        let benchmarker = Benchmarker::new("network_scalability_benchmarker");
        let result = benchmarker.run(&mut scenario, &config);

        // Ergebnisse validieren und ausgeben
        println!(
            "Netzwerk mit {} Neuronen: Durchschnitt {:.3} ms, Min {:.3} ms, Max {:.3} ms",
            size,
            result.average_ms(),
            result.min_ms(),
            result.max_ms()
        );

        // Grundlegende Plausibilitätsprüfungen
        assert!(
            result.average_ms() >= size as f64 * 0.5,
            "Durchschnittliche Zeit zu niedrig: {} ms",
            result.average_ms()
        );

        // Für größere Netzwerke sollten die Zeiten länger sein (Skalierung prüfen)
        if size > 10 {
            // In einem echten Test würden wir hier die Skalierungseigenschaften prüfen
            // Diese einfache Implementierung skaliert linear mit der Größe
        }
    }
}

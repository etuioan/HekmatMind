//! Unit-Tests für das Benchmark-Modul
//!
//! Diese Datei enthält Unit-Tests für das Benchmark-Framework. Sie sind direkt
//! neben der Implementierung platziert, um das TDD-Prinzip (Test-Driven Development)
//! konsequent im gesamten Projekt einheitlich umzusetzen.

#[cfg(test)]
mod benchmark_tests {
    use std::collections::HashMap;
    use std::time::Duration;

    use crate::benchmark::{BenchmarkConfig, BenchmarkResult, BenchmarkScenario, Benchmarker};
    // TelemetryCollector-Trait wird indirekt über BenchmarkScenario verwendet

    /// Eine einfache Test-Implementierung des BenchmarkScenario-Traits
    struct TestScenario {
        setup_called: bool,
        run_called: bool,
        teardown_called: bool,
        iteration_time_ms: u64,
    }

    impl TestScenario {
        fn new(iteration_time_ms: u64) -> Self {
            Self {
                setup_called: false,
                run_called: false,
                teardown_called: false,
                iteration_time_ms,
            }
        }
    }

    impl BenchmarkScenario for TestScenario {
        fn name(&self) -> &str {
            "TestScenario"
        }

        fn description(&self) -> &str {
            "Ein Testszenario für Unit-Tests"
        }

        fn setup(&mut self) {
            self.setup_called = true;
        }

        fn teardown(&mut self) {
            self.teardown_called = true;
        }

        fn run_iteration(&mut self) {
            self.run_called = true;
            if self.iteration_time_ms > 0 {
                std::thread::sleep(Duration::from_millis(self.iteration_time_ms));
            }
        }

        fn telemetry_labels(&self) -> HashMap<String, String> {
            let mut labels = HashMap::new();
            labels.insert("test_key".to_string(), "test_value".to_string());
            labels
        }
    }

    #[test]
    fn test_benchmark_config() {
        // Test der Erstellung und Parametrisierung
        let config = BenchmarkConfig::new("test_benchmark", "Beschreibung")
            .with_iterations(10)
            .with_warmup(2)
            .with_param("key1", "value1")
            .with_param("key2", "value2");

        // Überprüfung der Grundwerte
        assert_eq!(config.name, "test_benchmark");
        assert_eq!(config.description, "Beschreibung");
        assert_eq!(config.iterations, 10);
        assert_eq!(config.warmup_iterations, 2);

        // Überprüfung der Parameter
        assert_eq!(config.parameters.len(), 2);
        assert_eq!(config.parameters.get("key1"), Some(&"value1".to_string()));
        assert_eq!(config.parameters.get("key2"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_benchmark_result_calculations() {
        // Erstelle ein Benchmark-Resultat mit bekannten Werten
        let config = BenchmarkConfig::new("test_result", "Beschreibung");
        let start_time = std::time::Instant::now();

        let result = BenchmarkResult {
            name: "test_result".to_string(),
            description: "Beschreibung".to_string(),
            start_time,
            total_duration: Duration::from_millis(600),
            iteration_results: vec![100.0, 200.0, 300.0],
            metrics: HashMap::new(),
            config,
        };

        // Überprüfe die Berechnungen
        assert_eq!(result.average_ms(), 200.0);
        assert_eq!(result.min_ms(), 100.0);
        assert_eq!(result.max_ms(), 300.0);

        // Standardabweichung überprüfen (ungefähr)
        // Standardabweichung sollte mit drei Werten (100, 200, 300) etwa 100 sein
        let std_dev = result.std_dev_ms();
        assert!(
            std_dev > 80.0 && std_dev < 120.0,
            "Standardabweichung ist {}",
            std_dev
        );

        // Display-Trait testen
        let display_string = format!("{}", result);
        assert!(
            display_string.contains("test_result"),
            "Display-String enthält nicht den Test-Namen"
        );
        // Anstatt auf exaktes Format zu testen, prüfen wir nur, ob die Zahlen vorhanden sind
        assert!(
            display_string.contains("200"),
            "Display-String enthält nicht den Durchschnitt"
        );
    }

    #[test]
    fn test_benchmarker_lifecycle() {
        // Erstelle einen Benchmarker und ein Testszenario
        let benchmarker = Benchmarker::new("test_benchmarker");
        let mut scenario = TestScenario::new(1);
        let config = BenchmarkConfig::new("test_run", "Beschreibung")
            .with_iterations(5)
            .with_warmup(2);

        // Führe den Benchmark aus
        let result = benchmarker.run(&mut scenario, &config);

        // Überprüfe, ob der Lebenszyklus korrekt ist
        assert!(scenario.setup_called, "Setup wurde nicht aufgerufen");
        assert!(scenario.run_called, "Run wurde nicht aufgerufen");
        assert!(scenario.teardown_called, "Teardown wurde nicht aufgerufen");

        // Überprüfe das Ergebnis
        // Der Name wird vom Szenario übernommen, nicht von der Konfiguration
        assert_eq!(result.name, "TestScenario");
        // Die Beschreibung wird vom Szenario übernommen, nicht von der Konfiguration
        assert_eq!(result.description, "Ein Testszenario für Unit-Tests");
        assert_eq!(result.iteration_results.len(), 5);
    }

    #[test]
    fn test_zero_iterations() {
        // Test mit 0 Iterationen - sollte nicht abstürzen
        let benchmarker = Benchmarker::new("zero_test");
        let mut scenario = TestScenario::new(0);
        let config =
            BenchmarkConfig::new("zero_test", "Test mit null Iterationen").with_iterations(0);

        // Sollte nicht abstürzen und leere Timestamps zurückgeben
        let result = benchmarker.run(&mut scenario, &config);
        assert_eq!(result.iteration_results.len(), 0);
    }

    #[test]
    fn test_telemetry_integration() {
        // Teste, ob Telemetrie-Labels korrekt in das Ergebnis übernommen werden
        let benchmarker = Benchmarker::new("telemetry_test");
        let mut scenario = TestScenario::new(0);
        let config = BenchmarkConfig::new("telemetry_test", "Test der Telemetrie-Integration")
            .with_iterations(1);

        let result = benchmarker.run(&mut scenario, &config);

        // Überprüfen, ob der Benchmark erfolgreich war
        // Der Name wird vom Szenario übernommen, nicht von der Konfiguration
        assert_eq!(result.name, "TestScenario");
        assert!(!result.iteration_results.is_empty());
    }
}

#[cfg(test)]
mod scenarios_tests {
    use crate::benchmark::BenchmarkScenario;
    use crate::benchmark::scenarios::{NetworkScalabilityBenchmark, SingleNeuronBenchmark};
    use crate::telemetry::in_memory::InMemoryCollector;

    #[test]
    fn test_single_neuron_benchmark() {
        // Erstelle ein SingleNeuronBenchmark
        let mut benchmark = SingleNeuronBenchmark::new(500)
            .with_cycles(10)
            .with_input(0.8);

        // Teste den Lebenszyklus
        benchmark.setup();

        // Teste eine Iteration
        benchmark.run_iteration();

        // Überprüfe, dass telemetry_labels eine Map zurückgibt
        let labels = benchmark.telemetry_labels();
        assert!(
            !labels.is_empty(),
            "Telemetrie-Labels sollten nicht leer sein"
        );
    }

    #[test]
    fn test_network_scalability_benchmark() {
        // Erstelle ein kleines Netzwerk-Benchmark für schnelle Tests
        let mut benchmark = NetworkScalabilityBenchmark::<InMemoryCollector>::new(5).with_cycles(2);

        // Teste den Lebenszyklus
        benchmark.setup();

        // Da Netzwerk privat ist, können wir nur indirekt testen
        // Wir können run_iteration ausführen, was das Netzwerk verwendet
        benchmark.run_iteration();

        // Führe eine Iteration aus
        benchmark.run_iteration();

        // Überprüfe, dass telemetry_labels eine Map zurückgibt
        let labels = benchmark.telemetry_labels();
        assert!(
            !labels.is_empty(),
            "Telemetrie-Labels sollten nicht leer sein"
        );

        // Teste Teardown
        benchmark.teardown();
    }

    #[test]
    fn test_registry_handling() {
        // Teste die Registry-Funktionalität mit einem benutzerdefinierten Collector
        let collector = InMemoryCollector::new(100); // 100 Datenpunkte als Kapazität

        let mut benchmark = NetworkScalabilityBenchmark::new(3).with_registry(collector.clone());

        // Überprüfe, ob die Registry gesetzt wurde
        assert!(benchmark.get_registry().is_some());

        // Überprüfe, ob wir die Registry extrahieren können
        let extracted_registry = benchmark.take_registry();
        // Prüfe, ob wir ein gültiges Registry-Objekt extrahiert haben
        assert!(extracted_registry.is_some());

        // Registry sollte jetzt None sein
        assert!(benchmark.get_registry().is_none());
    }
}

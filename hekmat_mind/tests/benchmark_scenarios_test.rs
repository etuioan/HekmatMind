// Benchmark-Szenarien-Tests für HekmatMind
//
// Diese Tests validieren die Funktionalität der einzelnen Benchmark-Szenarien
// im HekmatMind-Projekt und sorgen für eine hohe Testabdeckung.

use hekmat_mind::benchmark::BenchmarkScenario;
use hekmat_mind::benchmark::scenarios::{
    Network, NetworkScalabilityBenchmark, SingleNeuronBenchmark,
};
use hekmat_mind::neural::neuron::Neuron;
use hekmat_mind::telemetry::collector::QueryableCollector;
use hekmat_mind::telemetry::in_memory::InMemoryCollector;
use hekmat_mind::telemetry::registry;
use hekmat_mind::telemetry::registry_mut;

// Hilfsfunktion zur Konfiguration der Telemetrie für Tests
fn setup_telemetry() -> InMemoryCollector {
    let collector = InMemoryCollector::new(500);

    // Collector in der Registry registrieren
    {
        let mut reg = registry_mut().expect("Registry-Lock fehlgeschlagen");
        reg.clear();
        reg.register(Box::new(collector));
    }

    // Neuen Collector für die Testauswertung erstellen
    InMemoryCollector::new(500)
}

#[test]
fn test_single_neuron_benchmark_creation() {
    // Teste die Erstellung und Konfiguration
    let benchmark = SingleNeuronBenchmark::new(300)
        .with_cycles(500)
        .with_input(0.75);

    assert_eq!(benchmark.name(), "single_neuron_processing");
    assert!(
        !benchmark.description().is_empty(),
        "Beschreibung sollte nicht leer sein"
    );
}

#[test]
fn test_single_neuron_benchmark_setup() {
    // Initialisiere Benchmark
    let mut benchmark = SingleNeuronBenchmark::new(300);

    // Führe Setup aus
    benchmark.setup();

    // Prüfe, ob alle notwendigen Komponenten initialisiert wurden
    // Diese Prüfung ist implizit, da setup() das Neuron zurücksetzt, aber keine direkten Prüfmöglichkeiten bietet
    // Wir testen daher die Funktionalität im nächsten Test
}

#[test]
fn test_single_neuron_benchmark_run_iteration() {
    // Telemetrie vorbereiten
    let _collector = setup_telemetry();

    // Benchmark initialisieren mit niedriger Zyklenanzahl für schnellere Tests
    let mut benchmark = SingleNeuronBenchmark::new(300)
        .with_cycles(5)
        .with_input(0.8);

    benchmark.setup();
    benchmark.run_iteration();

    // Prüfe Telemetriedaten
    let reg = registry().expect("Registry-Lock fehlgeschlagen");
    let collectors = reg.collectors();
    let collector_ref = collectors
        .first()
        .expect("Kein Collector registriert")
        .as_any()
        .downcast_ref::<InMemoryCollector>()
        .expect("Collector ist kein InMemoryCollector");

    // Prüfe, ob neuron_output-Metrik aufgezeichnet wurde
    let stats = collector_ref.query_stats("neural", "neuron_output");
    assert!(stats.is_some(), "Keine neuron_output-Metriken gefunden");
    assert!(
        stats.unwrap().count > 0,
        "Keine neuron_output-Datenpunkte gefunden"
    );
}

#[test]
fn test_single_neuron_benchmark_telemetry_labels() {
    let benchmark = SingleNeuronBenchmark::new(300).with_cycles(100);

    let labels = benchmark.telemetry_labels();

    // Prüfe, ob die erwarteten Labels vorhanden sind
    assert!(
        labels.contains_key("benchmark"),
        "Label 'benchmark' nicht gefunden"
    );
    assert!(
        labels.contains_key("cycles"),
        "Label 'cycles' nicht gefunden"
    );
    assert_eq!(labels.get("benchmark").unwrap(), "single_neuron_processing");
    assert_eq!(labels.get("cycles").unwrap(), "100");
}

#[test]
fn test_network_stub_creation() {
    let network = Network::new("test_network");

    // Teste Neuron-Hinzufügen
    let mut network_with_neurons = Network::new("network_with_neurons");
    network_with_neurons.add_neuron(Neuron::new(200));
    network_with_neurons.add_neuron(Neuron::new(300));

    // Stelle sicher, dass die Neuronen korrekt hinzugefügt wurden
    assert_eq!(network_with_neurons.neuron_count(), 2);
    assert!(
        network_with_neurons.neuron_count() > network.neuron_count(),
        "Netzwerk mit Neuronen sollte mehr Neuronen haben als leeres Netzwerk"
    );
}

#[test]
fn test_network_stub_input_and_cycle() {
    let mut network = Network::new("test_network");

    // Füge Neuronen hinzu
    network.add_neuron(Neuron::new(200));
    network.add_neuron(Neuron::new(300));

    // Sende Eingaben an Neuronen
    network.send_input(0, 0.8);

    // Führe einen Zyklus aus
    let active_count = network.cycle();

    // Prüfe, ob mindestens ein Neuron aktiv wurde (bei diesem Input sollte das so sein)
    assert!(
        active_count > 0,
        "Mindestens ein Neuron sollte durch den Input aktiviert werden"
    );
}

#[test]
fn test_network_scalability_benchmark_creation() {
    let benchmark = NetworkScalabilityBenchmark::<InMemoryCollector>::new(50).with_cycles(20);

    assert_eq!(benchmark.name(), "network_scalability");
    assert!(
        !benchmark.description().is_empty(),
        "Beschreibung sollte nicht leer sein"
    );
}

#[test]
fn test_network_scalability_benchmark_setup_and_teardown() {
    // Initialisiere Benchmark mit kleiner Neuronenzahl für schnellere Tests
    let mut benchmark = NetworkScalabilityBenchmark::<InMemoryCollector>::new(5).with_cycles(2);

    // Führe Setup aus
    benchmark.setup();

    // Netzwerk sollte jetzt initialisiert sein (indirekt prüfbar bei run_iteration)

    // Führe Teardown aus
    benchmark.teardown();

    // Nach Teardown sollte das Netzwerk freigegeben sein (intern getestet)
}

#[test]
fn test_network_scalability_benchmark_run_iteration() {
    // Collector für den Test erstellen
    let collector = InMemoryCollector::new(500);

    // Benchmark initialisieren mit kleiner Neuronenzahl für schnellere Tests
    // und mit eigenem Collector
    let mut benchmark = NetworkScalabilityBenchmark::<InMemoryCollector>::new(5)
        .with_cycles(3)
        .with_registry(collector.clone());

    // Benchmark ausführen
    benchmark.setup();
    benchmark.run_iteration();

    // Registry aus dem Benchmark extrahieren
    let custom_registry = benchmark
        .take_registry()
        .expect("Keine custom_registry gefunden");

    // Testen, ob die Metriken erfasst wurden
    let duration_stats = custom_registry.query_stats("network", "cycle_duration_us");
    println!("Dauer-Statistiken gefunden: {}", duration_stats.is_some());

    assert!(
        duration_stats.is_some(),
        "Keine cycle_duration_us-Metriken gefunden"
    );
    assert!(
        duration_stats.unwrap().count > 0,
        "Keine cycle_duration_us-Datenpunkte gefunden"
    );

    let active_neurons_stats = custom_registry.query_stats("network", "active_neurons");
    println!(
        "Neuronen-Statistiken gefunden: {}",
        active_neurons_stats.is_some()
    );

    assert!(
        active_neurons_stats.is_some(),
        "Keine active_neurons-Metriken gefunden"
    );
    assert!(
        active_neurons_stats.unwrap().count > 0,
        "Keine active_neurons-Datenpunkte gefunden"
    );
}

#[test]
fn test_network_scalability_benchmark_telemetry_labels() {
    let benchmark = NetworkScalabilityBenchmark::<InMemoryCollector>::new(100).with_cycles(50);

    let labels = benchmark.telemetry_labels();

    // Prüfe, ob die erwarteten Labels vorhanden sind
    assert!(
        labels.contains_key("benchmark"),
        "Label 'benchmark' nicht gefunden"
    );
    assert!(
        labels.contains_key("neuron_count"),
        "Label 'neuron_count' nicht gefunden"
    );
    assert!(
        labels.contains_key("cycles"),
        "Label 'cycles' nicht gefunden"
    );

    assert_eq!(labels.get("benchmark").unwrap(), "network_scalability");
    assert_eq!(labels.get("neuron_count").unwrap(), "100");
    assert_eq!(labels.get("cycles").unwrap(), "50");
}

#[test]
fn test_network_with_various_sizes() {
    // Teste die Erstellung von Netzwerken verschiedener Größen
    for size in [1, 5, 10] {
        let mut network = Network::new(&format!("test_network_{}", size));

        // Füge die angegebene Anzahl an Neuronen hinzu
        for i in 0..size {
            network.add_neuron(Neuron::new(i + 100));
        }

        // Verifiziere die Anzahl der Neuronen
        assert_eq!(
            network.neuron_count(),
            size as usize,
            "Netzwerk sollte genau {} Neuronen haben",
            size
        );

        // Führe einen Zyklus aus
        let active_count = network.cycle();

        // Wir können nicht genau vorhersagen, wie viele Neuronen aktiv sein werden,
        // da dies von den internen Zuständen und Schwellwerten abhängt, aber wir können
        // zumindest prüfen, ob die Funktion einen plausiblen Wert zurückgibt
        assert!(
            active_count <= size as usize,
            "Es können nicht mehr aktive Neuronen als Gesamtneuronen geben"
        );
    }
}

#[test]
fn test_network_with_complex_connectivity() {
    let mut network = Network::new("complex_network");

    // Erstelle ein Netzwerk mit 4 Neuronen
    for _ in 0..4 {
        network.add_neuron(Neuron::new(150));
    }

    // Erstelle verschiedene Verbindungsmuster
    // 1. Vollständig verbundenes Paar (0-1)
    network.connect_neurons(0, 1, 0.9);
    network.connect_neurons(1, 0, 0.9);

    // 2. Vorwärtsgerichtete Kette (2-3)
    network.connect_neurons(2, 3, 0.8);

    // 3. Verbinde erste mit zweiter Gruppe
    network.connect_neurons(0, 2, 0.5);

    // Aktiviere erstes Neuron
    network.send_input(0, 1.0);

    // Führe mehrere Zyklen aus, um Signalausbreitung zu testen
    for _ in 0..3 {
        network.cycle();
    }

    // Wir prüfen hier nicht auf spezifische Ergebnisse, da diese von der
    // internen Implementierung der Neuronen abhängen. Der Test dient hauptsächlich
    // zur Codeabdeckung und Sicherstellung, dass keine Ausnahmen auftreten.
}

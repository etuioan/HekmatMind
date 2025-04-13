use hekmat_mind::neural::neuron::model::{Neuron, NeuronState};
use std::time::Instant;

/// Funktionaler Regressionstest für das Baseline-Verhalten von Neuronen
///
/// Dieser Test validiert die grundlegenden Verhaltenscharakteristiken von Neuronen,
/// um sicherzustellen, dass keine Regressionen im Kernverhalten auftreten.
#[test]
fn test_neuron_baseline_behavior() {
    // 1. ERSTELLE ein Neuron mit festgelegten Parametern für reproduzierbare Tests
    let mut neuron = Neuron::with_params(500, 0.5, 0.01);

    // Validiere Anfangszustand
    assert_eq!(neuron.state(), NeuronState::Inactive);
    assert_eq!(neuron.threshold(), 0.5);
    assert_eq!(neuron.activation_energy(), 0.0);

    // 2. FÜHRE eine deterministische Sequenz von Aktivierungen durch
    assert!(!neuron.receive_input(0.3));
    assert_eq!(neuron.activation_energy(), 0.3);

    assert!(!neuron.receive_input(0.1));
    assert_eq!(neuron.activation_energy(), 0.4);

    assert!(neuron.receive_input(0.2)); // Überschreitet Schwellwert, wird aktiviert
    assert_eq!(neuron.state(), NeuronState::Active);

    // 3. VALIDIERE Zyklusverhalten und Refraktärphase
    let output = neuron.cycle();
    assert!(output > 0.5); // Ausgabe sollte mindestens Schwellwert sein
    assert_eq!(neuron.state(), NeuronState::Refractory);
    assert_eq!(neuron.activation_energy(), 0.0);

    // Eingabe während Refraktärphase sollte ignoriert werden
    assert!(!neuron.receive_input(1.0));
    assert_eq!(neuron.activation_energy(), 0.0);

    // Nach einem weiteren Zyklus sollte das Neuron wieder inaktiv sein
    assert_eq!(neuron.cycle(), 0.0);
    assert_eq!(neuron.state(), NeuronState::Inactive);

    // 4. TESTE Plastizität
    let original_threshold = neuron.threshold();
    neuron.adapt_threshold(true, 0.2); // Anpassung mit höherer als gewünschter Aktivität
    let new_threshold = neuron.threshold();
    assert!(
        new_threshold > original_threshold,
        "Schwellwert sollte erhöht werden, wenn Aktivität höher als Zielwert ist"
    );
}

/// Leistungsregressionstest für Neuronen
///
/// Dieser Test überwacht die Leistungscharakteristiken der Neuronen-Implementierung,
/// um frühzeitig Leistungseinbußen zu erkennen.
#[test]
fn test_neuron_performance() {
    const NUM_CYCLES: usize = 10_000;
    const MAX_ALLOWED_TIME_MS: u128 = 50; // Maximal erlaubte Zeit in Millisekunden

    let mut neuron = Neuron::new(500);

    // Zeitmessung für eine große Anzahl von Zyklen
    let start = Instant::now();

    for _ in 0..NUM_CYCLES {
        neuron.receive_input(0.1);
        neuron.cycle();
    }

    let duration = start.elapsed();
    let duration_ms = duration.as_millis();

    println!(
        "Neuron Performance: {} Zyklen in {} ms",
        NUM_CYCLES, duration_ms
    );

    // Sicherstellen, dass die Leistung nicht unter einen festgelegten Schwellwert fällt
    assert!(
        duration_ms < MAX_ALLOWED_TIME_MS,
        "Performance-Regression erkannt: {} ms überschreitet Limit von {} ms",
        duration_ms,
        MAX_ALLOWED_TIME_MS
    );
}

/// Test für Zustandswiederherstellung und Determinismus
///
/// Dieser Test validiert, dass Neuronen deterministisch arbeiten und
/// bei gleichen Eingaben die gleichen Ausgaben produzieren.
#[test]
fn test_neuron_determinism() {
    // Zwei identische Neuronen erstellen
    let mut neuron1 = Neuron::with_params(500, 0.5, 0.01);
    let mut neuron2 = Neuron::with_params(500, 0.5, 0.01);

    // Beide mit identischen Eingaben stimulieren
    let input_sequence = [0.1, 0.2, 0.1, 0.2, 0.3];

    let mut outputs1 = Vec::new();
    let mut outputs2 = Vec::new();

    for &input in &input_sequence {
        neuron1.receive_input(input);
        neuron2.receive_input(input);

        outputs1.push(neuron1.cycle());
        outputs2.push(neuron2.cycle());
    }

    // Prüfen, ob beide Neuronen identische Ausgaben produzieren
    assert_eq!(
        outputs1, outputs2,
        "Neuronen sollten bei identischen Eingaben deterministisch reagieren"
    );
}

/// Adaptiver Schwellwerttest (evolutionärer Aspekt)
///
/// Dieser Test validiert die Fähigkeit des Neurons, seinen Schwellwert
/// an verschiedene Aktivitätsmuster anzupassen.
#[test]
fn test_neuron_threshold_adaptation() {
    let mut neuron = Neuron::with_params(500, 0.5, 0.05); // Höhere Plastizitätsrate für schnellere Anpassung

    let initial_threshold = neuron.threshold();

    // 1. Hohe Aktivität simulieren (sollte Schwellwert erhöhen)
    for _ in 0..10 {
        neuron.adapt_threshold(true, 0.2); // Aktiv bei gewünschter niedrigerer Aktivität
    }

    let high_activity_threshold = neuron.threshold();
    assert!(
        high_activity_threshold > initial_threshold,
        "Schwellwert sollte bei hoher Aktivität steigen"
    );

    // 2. Niedrige Aktivität simulieren (sollte Schwellwert senken)
    for _ in 0..10 {
        neuron.adapt_threshold(false, 0.8); // Inaktiv bei gewünschter höherer Aktivität
    }

    let low_activity_threshold = neuron.threshold();
    assert!(
        low_activity_threshold < high_activity_threshold,
        "Schwellwert sollte bei niedriger Aktivität sinken"
    );
}

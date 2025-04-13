use hekmat_mind::neural::synapse::model::{Synapse, constants};
use std::time::Instant;
use uuid::Uuid;

/// Funktionaler Regressionstest für das Baseline-Verhalten von Synapsen
///
/// Dieser Test validiert die grundlegenden Verhaltenscharakteristiken von Synapsen,
/// um sicherzustellen, dass keine Regressionen im Kernverhalten auftreten.
#[test]
fn test_synapse_baseline_behavior() {
    // IDs für Testzwecke generieren
    let pre_id = Uuid::new_v4();
    let post_id = Uuid::new_v4();

    // 1. ERSTELLE eine Synapse mit festgelegten Parametern
    let mut synapse = Synapse::new(pre_id, post_id, 0.5);

    // Validiere Anfangszustand
    assert_eq!(synapse.weight(), 0.5);
    assert_eq!(synapse.delay(), constants::DEFAULT_DELAY);
    assert!(!synapse.active_state());
    assert_eq!(synapse.pre_neuron_id(), &pre_id);
    assert_eq!(synapse.post_neuron_id(), &post_id);

    // 2. TESTE Signalübertragung
    let input = 0.8;
    let output = synapse.transmit(input);

    // Überprüfe, dass Output = Input * Gewicht
    assert_eq!(output, input * synapse.weight());
    assert!(synapse.active_state());

    // 3. ÜBERPRÜFE Aktivitätsdauer
    let time_step = constants::ACTIVE_DURATION / 2.0;
    synapse.update(time_step);
    assert!(synapse.active_state()); // Sollte noch aktiv sein

    synapse.update(time_step); // Gesamtzeit = ACTIVE_DURATION
    assert!(!synapse.active_state()); // Sollte inaktiv werden

    // 4. TESTE Gewichtsanpassung durch Hebbsche Plastizität
    let original_weight = synapse.weight();

    // Fall 1: Prä- und post-synaptisches Neuron aktiv - Gewicht sollte steigen
    synapse.apply_hebbian_plasticity(true, true, 0.1);
    assert!(
        synapse.weight() > original_weight,
        "Gewicht sollte bei gemeinsamer Aktivität erhöht werden"
    );

    // Fall 2: Nur präsynaptisches Neuron aktiv - Gewicht sollte sinken
    let strengthened_weight = synapse.weight();
    synapse.apply_hebbian_plasticity(true, false, 0.1);
    assert!(
        synapse.weight() < strengthened_weight,
        "Gewicht sollte bei nicht-korrelierter Aktivität reduziert werden"
    );
}

/// Leistungsregressionstest für Synapsen
///
/// Dieser Test überwacht die Leistungscharakteristiken der Synapsen-Implementierung,
/// um frühzeitig Leistungseinbußen zu erkennen.
#[test]
fn test_synapse_performance() {
    const NUM_OPERATIONS: usize = 20_000;
    const MAX_ALLOWED_TIME_MS: u128 = 50; // Maximal erlaubte Zeit in Millisekunden

    let pre_id = Uuid::new_v4();
    let post_id = Uuid::new_v4();
    let mut synapse = Synapse::new(pre_id, post_id, 0.5);

    // Zeitmessung für eine große Anzahl von Operationen
    let start = Instant::now();

    for i in 0..NUM_OPERATIONS {
        if i % 3 == 0 {
            synapse.transmit(0.5);
        }
        synapse.update(0.001);

        if i % 10 == 0 {
            synapse.apply_hebbian_plasticity(true, i % 5 == 0, 0.01);
        }
    }

    let duration = start.elapsed();
    let duration_ms = duration.as_millis();

    println!(
        "Synapse Performance: {} Operationen in {} ms",
        NUM_OPERATIONS, duration_ms
    );

    // Sicherstellen, dass die Leistung nicht unter einen festgelegten Schwellwert fällt
    assert!(
        duration_ms < MAX_ALLOWED_TIME_MS,
        "Performance-Regression erkannt: {} ms überschreitet Limit von {} ms",
        duration_ms,
        MAX_ALLOWED_TIME_MS
    );
}

/// Test für Determinismus bei Gewichtsanpassungen
///
/// Dieser Test validiert, dass Synapsen deterministisch arbeiten und
/// bei gleichen Eingaben die gleichen Gewichtsänderungen produzieren.
#[test]
fn test_synapse_determinism() {
    let pre_id1 = Uuid::new_v4();
    let post_id1 = Uuid::new_v4();
    let pre_id2 = Uuid::new_v4();
    let post_id2 = Uuid::new_v4();

    // Zwei identische Synapsen erstellen
    let mut synapse1 = Synapse::new(pre_id1, post_id1, 0.5);
    let mut synapse2 = Synapse::new(pre_id2, post_id2, 0.5);

    // Definiere eine Sequenz von Plastizitäts-Ereignissen
    let plasticity_events = [
        (true, true),   // Beide aktiv
        (true, false),  // Nur prä aktiv
        (false, true),  // Nur post aktiv
        (true, true),   // Beide aktiv
        (false, false), // Beide inaktiv
    ];

    // Beide Synapsen mit identischen Ereignissen aktualisieren
    for &(pre_active, post_active) in &plasticity_events {
        synapse1.apply_hebbian_plasticity(pre_active, post_active, 0.1);
        synapse2.apply_hebbian_plasticity(pre_active, post_active, 0.1);
    }

    // Prüfen, ob beide Synapsen die gleiche Gewichtsänderung aufweisen
    assert_eq!(
        synapse1.weight(),
        synapse2.weight(),
        "Synapsen sollten bei identischen Eingaben deterministisch reagieren"
    );
}

/// Test für Synapsen-Konfigurierbarkeit und Extremwerte
///
/// Dieser Test validiert das Verhalten von Synapsen an den Grenzen ihrer
/// Parameterbereiche und die korrekte Begrenzung von Werten.
#[test]
fn test_synapse_boundary_values() {
    let pre_id = Uuid::new_v4();
    let post_id = Uuid::new_v4();

    // Test mit minimalem Gewicht
    let min_synapse = Synapse::new(pre_id, post_id, 0.0);
    assert_eq!(min_synapse.weight(), 0.0);

    // Test mit maximalem Gewicht
    let max_synapse = Synapse::new(pre_id, post_id, 1.0);
    assert_eq!(max_synapse.weight(), 1.0);

    // Test mit Gewicht außerhalb des gültigen Bereichs
    let over_synapse = Synapse::new(pre_id, post_id, 1.5); // Sollte auf 1.0 begrenzt werden
    assert_eq!(over_synapse.weight(), 1.0);

    let under_synapse = Synapse::new(pre_id, post_id, -0.5); // Sollte auf 0.0 begrenzt werden
    assert_eq!(under_synapse.weight(), 0.0);

    // Test mit extremen Verzögerungswerten
    let mut test_synapse = Synapse::new(pre_id, post_id, 0.5);

    test_synapse.set_delay(constants::MAX_DELAY * 2.0); // Sollte auf MAX_DELAY begrenzt werden
    assert_eq!(test_synapse.delay(), constants::MAX_DELAY);

    test_synapse.set_delay(-1.0); // Sollte auf 0.0 begrenzt werden
    assert_eq!(test_synapse.delay(), 0.0);
}

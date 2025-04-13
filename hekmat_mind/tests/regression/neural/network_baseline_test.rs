use hekmat_mind::neural::network::model::{Network, NetworkBuilder};
use hekmat_mind::neural::neuron::model::{Neuron, NeuronState};
use hekmat_mind::neural::synapse::model::Synapse;
use std::time::Instant;

/// Funktionaler Regressionstest für Baseline-Verhalten von neuronalen Netzwerken
///
/// Dieser Test validiert die grundlegenden Verhaltenscharakteristiken von neuronalen
/// Netzwerken, um sicherzustellen, dass keine Regressionen im Kernverhalten auftreten.
#[test]
fn test_network_baseline_behavior() {
    // Teste Netzwerk mit genau definierten Komponenten für deterministische Ergebnisse
    let mut network = Network::new();

    // Spezifische Neuronen mit kontrollierten Parametern
    let neuron1 = Neuron::with_params(500, 0.5, 0.01);
    let neuron2 = Neuron::with_params(500, 0.5, 0.01);

    let neuron1_id = *neuron1.id();
    let neuron2_id = *neuron2.id();

    // Neuronen zum Netzwerk hinzufügen
    network.add_neuron(neuron1);
    network.add_neuron(neuron2);

    // Teste Netzwerkgröße
    assert_eq!(network.neuron_count(), 2);
    assert_eq!(network.synapse_count(), 0);

    // Synapse zwischen Neuronen erstellen (unidirektional)
    let synapse = Synapse::new(neuron1_id, neuron2_id, 0.8);
    network.add_synapse(synapse);

    // Teste Synapsenerstellung
    assert_eq!(network.synapse_count(), 1);
    assert!(network.has_synapse_between(&neuron1_id, &neuron2_id));
    assert!(!network.has_synapse_between(&neuron2_id, &neuron1_id));

    // Teste Signalpropagation (Neuron1 aktiviert, Signal sollte zu Neuron2 fließen)
    network.stimulate_neuron(&neuron1_id, 1.0); // Aktiviere Neuron1

    // Hole Referenzen für Überprüfungszwecke
    let neuron1 = network.get_neuron(&neuron1_id).unwrap();
    assert_eq!(neuron1.state(), NeuronState::Active);

    // Führe Netzwerkzyklus durch
    network.cycle(0.001);

    // Teste ob das Signal korrekt übertragen wurde
    let neuron2 = network.get_neuron(&neuron2_id).unwrap();
    assert!(
        neuron2.activation_energy() > 0.0,
        "Signal sollte vom ersten zum zweiten Neuron übertragen werden"
    );
}

/// Leistungsregressionstest für neuronale Netzwerke
///
/// Dieser Test überwacht die Leistungscharakteristiken der Netzwerk-Implementierung,
/// um frühzeitig Leistungseinbußen zu erkennen.
#[test]
fn test_network_performance() {
    // Erstelle ein Netzwerk mit definierten Eigenschaften
    let mut network = NetworkBuilder::new()
        .with_neurons(20, 500) // 20 Neuronen mit mittlerer Geschwindigkeit
        .with_random_connections(0.3, 0.5) // 30% Verbindungswahrscheinlichkeit, Gewicht 0.5
        .build();

    const NUM_CYCLES: usize = 100;
    const MAX_ALLOWED_TIME_MS: u128 = 100;

    // Zeitmessung für eine feste Anzahl von Zyklen
    let start = Instant::now();

    for _ in 0..NUM_CYCLES {
        // Zufällige Stimulation von 30% der Neuronen
        let neuron_ids: Vec<_> = network.neurons().keys().cloned().collect();
        for neuron_id in neuron_ids.iter().take(neuron_ids.len() / 3) {
            network.stimulate_neuron(neuron_id, 0.8);
        }

        // Netzwerkzyklus ausführen
        network.cycle(0.001);

        // Plastizität anwenden
        network.apply_plasticity(0.01);
    }

    let duration = start.elapsed();
    let duration_ms = duration.as_millis();

    println!(
        "Netzwerk Performance (20 Neuronen): {} Zyklen in {} ms",
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

/// Test für Skalierbarkeit des Netzwerks (evolutionärer Aspekt)
///
/// Dieser Test validiert das Verhalten bei verschiedenen Netzwerkgrößen,
/// um sicherzustellen, dass die Implementierung mit steigenden Netzwerkgrößen skaliert.
#[test]
fn test_network_scalability() {
    // Teste verschiedene Netzwerkgrößen
    let sizes = [5, 20, 50];
    let connection_density = 0.2;

    for &size in &sizes {
        // Erstelle ein Netzwerk mit spezifischer Größe und deterministischen Verbindungen
        let mut network = NetworkBuilder::new()
            .with_neurons(size, 500)
            .with_deterministic_connections(connection_density, 0.5)
            .build();

        // Berechne die erwartete Anzahl von Verbindungen
        // (Für deterministische Verbindungen: size * (size-1) * connection_density)
        let max_connections = size * (size - 1);
        let expected_synapses = (max_connections as f32 * connection_density).round() as usize;

        // Teste ob die Anzahl der Synapsen exakt den Erwartungen entspricht
        let actual_synapses = network.synapse_count();
        assert_eq!(
            actual_synapses, expected_synapses,
            "Unerwartete Anzahl von Synapsen bei Netzwerkgröße {}: Erwartet {}, Erhalten {}",
            size, expected_synapses, actual_synapses
        );

        // Führe einen einfachen Netzwerkzyklus durch
        network.cycle(0.001);
    }
}

/// Test für Netzwerkreset und -rekonstruktion
///
/// Dieser Test validiert, dass Netzwerke vollständig zurückgesetzt werden können
/// und dass die Aktivität aller Komponenten nach dem Reset korrekt ist.
#[test]
fn test_network_reset() {
    // Erstelle ein einfaches Netzwerk
    let mut network = NetworkBuilder::new()
        .with_neurons(10, 500)
        .with_random_connections(0.4, 0.5)
        .build();

    // Aktiviere einige Neuronen
    let neuron_ids: Vec<_> = network.neurons().keys().cloned().collect();
    for neuron_id in neuron_ids.iter().take(5) {
        network.stimulate_neuron(neuron_id, 1.0);
    }

    // Netzwerkzyklus ausführen
    network.cycle(0.001);

    // Netzwerk zurücksetzen
    network.reset();

    // Validiere, dass alle Neuronen nach dem Reset inaktiv sind
    for neuron in network.neurons().values() {
        assert_eq!(
            neuron.state(),
            NeuronState::Inactive,
            "Alle Neuronen sollten nach Reset inaktiv sein"
        );
        assert_eq!(
            neuron.activation_energy(),
            0.0,
            "Alle Neuronen sollten nach Reset keine Aktivierungsenergie haben"
        );
    }
}

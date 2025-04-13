#[cfg(test)]
mod neuron_tests {
    // Wachstumsspezifische Importe wurden ins Growth-Testmodul verschoben
    use crate::neural::growth::Position;
    use crate::neural::neuron::model::{Neuron, NeuronState, constants};
    use proptest::prelude::*;
    use std::fmt::Write;

    #[test]
    fn test_neuron_creation() {
        // Teste die Erstellung mit gültigen Werten
        let neuron = Neuron::new(500);
        assert_eq!(neuron.speed(), 500);
        assert_eq!(neuron.state(), NeuronState::Inactive);
        assert_eq!(neuron.threshold(), constants::DEFAULT_THRESHOLD);
        assert_eq!(neuron.activation_energy(), 0.0);
        assert_eq!(neuron.plasticity_rate(), constants::DEFAULT_PLASTICITY_RATE);

        // Teste die Begrenzung der Geschwindigkeit
        let neuron_min = Neuron::new(0); // Sollte auf MIN_SPEED begrenzt werden
        assert_eq!(neuron_min.speed(), constants::MIN_SPEED);

        let neuron_max = Neuron::new(2000); // Sollte auf MAX_SPEED begrenzt werden
        assert_eq!(neuron_max.speed(), constants::MAX_SPEED);
    }

    #[test]
    fn test_capacity_calculation() {
        // Teste die Kapazitätsberechnung mit verschiedenen Geschwindigkeiten
        let neuron_slow = Neuron::new(100);
        assert_eq!(neuron_slow.capacity(), 100.0 * constants::CAPACITY_FACTOR);

        let neuron_fast = Neuron::new(1000);
        assert_eq!(neuron_fast.capacity(), 1000.0 * constants::CAPACITY_FACTOR);
    }

    #[test]
    fn test_custom_parameters() {
        // Teste die Erstellung mit benutzerdefinierten Parametern
        let threshold = 0.7;
        let plasticity_rate = 0.05;
        let neuron = Neuron::with_params(300, threshold, plasticity_rate);

        assert_eq!(neuron.speed(), 300);
        assert_eq!(neuron.threshold(), threshold);
        assert_eq!(neuron.plasticity_rate(), plasticity_rate);
    }

    #[test]
    fn test_activation_cycle() {
        // Teste den vollständigen Aktivierungszyklus eines Neurons
        let mut neuron = Neuron::new(500);
        let threshold = neuron.threshold();

        // Noch nicht aktiviert - unter Schwellwert
        assert!(!neuron.receive_input(threshold * 0.5));
        assert_eq!(neuron.state(), NeuronState::Inactive);
        assert_eq!(neuron.activation_energy(), threshold * 0.5);

        // Jetzt aktiviert - über Schwellwert
        assert!(neuron.receive_input(threshold * 0.6));
        assert_eq!(neuron.state(), NeuronState::Active);
        assert_eq!(neuron.activation_energy(), threshold * 1.1);

        // Zyklus ausführen - sollte Ausgabe liefern und in refraktären Zustand wechseln
        let output = neuron.cycle();
        assert_eq!(output, threshold * 1.1);
        assert_eq!(neuron.state(), NeuronState::Refractory);
        assert_eq!(neuron.activation_energy(), 0.0);

        // Weitere Eingaben im refraktären Zustand sollten ignoriert werden
        assert!(!neuron.receive_input(1.0));
        assert_eq!(neuron.activation_energy(), 0.0);

        // Nächster Zyklus sollte zurück in inaktiven Zustand führen
        assert_eq!(neuron.cycle(), 0.0);
        assert_eq!(neuron.state(), NeuronState::Inactive);
    }

    #[test]
    fn test_threshold_adaptation() {
        let mut neuron = Neuron::with_params(500, 0.5, 0.1);
        let initial_threshold = neuron.threshold();

        // Adaptation bei Aktivität (Schwellwert sollte steigen)
        neuron.adapt_threshold(true, 0.2);
        assert!(neuron.threshold() > initial_threshold);

        // Adaptation bei Inaktivität (Schwellwert sollte sinken)
        let high_threshold = neuron.threshold();
        neuron.adapt_threshold(false, 0.2);
        assert!(neuron.threshold() < high_threshold);

        // Stresstest für mehrere Adaptationen
        for _ in 0..10 {
            let old_threshold = neuron.threshold();
            neuron.adapt_threshold(false, 0.9); // Ziel ist hohe Aktivität
            assert!(neuron.threshold() <= old_threshold); // Schwellwert sollte sinken oder gleich bleiben
        }

        // Test für Grenzwerte
        neuron = Neuron::with_params(500, 0.1, 0.2);
        for _ in 0..10 {
            neuron.adapt_threshold(false, 1.0); // Unmögliches Ziel (immer aktiv)
            assert!(neuron.threshold() >= 0.0); // Schwellwert sollte nie negativ werden
        }
    }

    #[test]
    fn test_reset() {
        let mut neuron = Neuron::new(500);

        // Neuron aktivieren
        neuron.receive_input(neuron.threshold() * 2.0);
        assert_eq!(neuron.state(), NeuronState::Active);

        // Neuron zurücksetzen
        neuron.reset();
        assert_eq!(neuron.state(), NeuronState::Inactive);
        assert_eq!(neuron.activation_energy(), 0.0);
    }

    #[test]
    fn test_neuron_id() {
        // Erzeuge zwei Neuronen und überprüfe, dass sie unterschiedliche IDs haben
        let neuron1 = Neuron::new(500);
        let neuron2 = Neuron::new(500);

        let id1 = neuron1.id();
        let id2 = neuron2.id();

        // IDs sollten nicht null sein und unterschiedlich sein
        assert_ne!(id1, id2, "Neuronen sollten unterschiedliche IDs haben");

        // ID sollte beim Klonen erhalten bleiben
        let neuron_clone = neuron1.clone();
        assert_eq!(
            neuron1.id(),
            neuron_clone.id(),
            "Geklonte Neuronen sollten identische IDs haben"
        );
    }

    #[test]
    fn test_neuron_state_display() {
        // Teste die Display-Implementierung für alle Zustände
        let mut buffer = String::new();

        write!(&mut buffer, "{}", NeuronState::Inactive).unwrap();
        assert_eq!(buffer, "Inaktiv");

        buffer.clear();
        write!(&mut buffer, "{}", NeuronState::Active).unwrap();
        assert_eq!(buffer, "Aktiv");

        buffer.clear();
        write!(&mut buffer, "{}", NeuronState::Refractory).unwrap();
        assert_eq!(buffer, "Refraktär");
    }

    #[test]
    fn test_refractory_state_cycle() {
        // Ein Neuron erstellen und in refraktären Zustand versetzen
        let mut neuron = Neuron::new(500); // Geschwindigkeit 500 als Parameter übergeben

        // Zuerst aktivieren und dann einen Zyklus durchführen, um es in den refraktären Zustand zu bringen
        neuron.receive_input(1000.0); // Genug Energie um zu aktivieren
        let _output = neuron.cycle();

        // Überprüfen, dass das Neuron jetzt im refraktären Zustand ist
        assert_eq!(neuron.state(), NeuronState::Refractory);

        // Einen weiteren Zyklus durchführen während es im refraktären Zustand ist
        let refractory_output = neuron.cycle();

        // Überprüfen, dass das Neuron nach einem Zyklus im refraktären Zustand 0.0 ausgibt
        assert_eq!(refractory_output, 0.0);

        // Überprüfen, dass das Neuron nach einem Zyklus im refraktären Zustand wieder inaktiv ist
        assert_eq!(neuron.state(), NeuronState::Inactive);
    }

    /// Testet, dass die cycle-Methode korrekt im aktiven Zustand funktioniert
    #[test]
    fn test_cycle_active_state() {
        // Erstellen eines Neurons
        let mut neuron = Neuron::new(100);

        // Berechne den benötigten Input basierend auf dem Schwellenwert
        let input_needed = neuron.threshold() + 0.1; // Etwas mehr als der Schwellenwert

        // Aktiviere das Neuron durch ausreichenden Input, um den Schwellenwert zu überschreiten
        neuron.receive_input(input_needed);

        // Das Neuron sollte jetzt aktiv sein, da receive_input() es aktiviert haben sollte
        assert_eq!(neuron.state(), NeuronState::Active);

        // Führe einen Zyklus aus - dies sollte das Neuron in den Refractory-Zustand versetzen
        let output = neuron.cycle();

        // Das Ausgangssignal sollte positiv sein
        assert!(output > 0.0);

        // Nach einem Zyklus sollte das Neuron im Refractory-Zustand sein
        assert_eq!(neuron.state(), NeuronState::Refractory);

        // Die Aktivierungsenergie sollte zurückgesetzt sein
        assert_eq!(neuron.activation_energy(), 0.0);
    }

    // Die Tests für start_axon_growth und as_growth_factor wurden in das
    // separate Growth-Modul-Testmodul verschoben (src/neural/growth/tests.rs).
    // Dies verbessert die Modulorganisation und folgt dem Prinzip, dass Tests
    // bei der zu testenden Funktionalität liegen sollten.

    #[test]
    fn test_neuron_position() {
        // Test für die Standardposition (0,0,0) bei Erstellung ohne Position
        let neuron = Neuron::new(500);
        assert_eq!(*neuron.position(), Position::new(0.0, 0.0, 0.0));

        // Test für die Erstellung mit benutzerdefinierter Position
        let pos = Position::new(1.0, 2.0, 3.0);
        let neuron_with_pos = Neuron::with_position(500, pos);
        assert_eq!(*neuron_with_pos.position(), pos);

        // Test für die Erstellung mit benutzerdefinierten Parametern und Position
        let neuron_custom =
            Neuron::with_params_and_position(500, 0.7, 0.05, Position::new(4.0, 5.0, 6.0));
        assert_eq!(*neuron_custom.position(), Position::new(4.0, 5.0, 6.0));

        // Test für die Änderung der Position
        let mut neuron_mutable = Neuron::new(500);
        let new_pos = Position::new(7.0, 8.0, 9.0);
        neuron_mutable.set_position(new_pos);
        assert_eq!(*neuron_mutable.position(), new_pos);
    }

    // Property-based Tests mit proptest
    proptest! {
        #[test]
        fn proptest_speed_always_in_valid_range(speed in 0u16..3000) {
            let neuron = Neuron::new(speed);
            let actual_speed = neuron.speed();
            assert!(actual_speed >= constants::MIN_SPEED);
            assert!(actual_speed <= constants::MAX_SPEED);
        }

        #[test]
        fn proptest_capacity_proportional_to_speed(speed in constants::MIN_SPEED..=constants::MAX_SPEED) {
            let neuron = Neuron::new(speed);
            assert_eq!(neuron.capacity(), speed as f32 * constants::CAPACITY_FACTOR);
        }

        // Verbesserter Property-Test, der alle Neuronenzustände berücksichtigt
        // und Rundungsfehler vermeidet, um Hängen zu verhindern
        #[test]
        fn proptest_activation_with_various_inputs(
            // Begrenztere Eingabebereiche für stabilere Tests
            speed in constants::MIN_SPEED..=constants::MAX_SPEED,
            threshold in 0.2f32..0.9f32, // Vermeide extreme Schwellwerte
            inputs in prop::collection::vec(0.05f32..1.5f32, 1..5) // Weniger, aber bedeutsamere Eingaben
        ) {
            // Protokolliere Testparameter für Debugging
            let params_info = format!("Test mit: speed={}, threshold={:.4}, inputs={:?}",
                                        speed, threshold, inputs);

            // Neuron mit stabilen Parametern erstellen
            let mut neuron = Neuron::with_params(speed, threshold, 0.01);
            let _initial_state = neuron.state(); // Initial-Zustand für Debug-Zwecke verfügbar halten

            // Status-Tracking
            let mut total_input = 0.0;
            let mut activated = false;
            let mut cycled = false;

            // Status-Logik für jeden Zustand
            for (i, input) in inputs.iter().enumerate() {
                // Protokolliere den aktuellen Zustand vor jeder Eingabe
                let pre_state = neuron.state();

                // Nur im inaktiven Zustand Energie akkumulieren
                if pre_state == NeuronState::Inactive {
                    // Kleiner Epsilon-Wert zur Vermeidung von Fließkomma-Vergleichsproblemen
                    let epsilon = 1e-6;
                    total_input += input;

                    // Aktivierung erwarten, wenn Schwellwert überschritten wird (mit Toleranz)
                    let should_activate = total_input >= (threshold - epsilon);
                    let did_activate = neuron.receive_input(*input);

                    if should_activate != did_activate {
                        // Detaillierte Fehlerinformationen mit robusterem Vergleich
                        prop_assert!(should_activate == did_activate,
                            "Aktivierungsfehler: erwartet={}, tatsächlich={}, total={:.6}, schwelle={:.6}\n{}",
                            should_activate, did_activate, total_input, threshold, params_info);
                    }

                    if did_activate {
                        activated = true;
                    }
                } else if pre_state == NeuronState::Active {
                    // Im aktiven Zustand, führe einen Zyklus durch
                    let output = neuron.cycle();
                    prop_assert!(output > 0.0, "Aktivierter Zyklus sollte positive Ausgabe haben");
                    prop_assert_eq!(neuron.state(), NeuronState::Refractory,
                                  "Nach aktivem Zyklus sollte Neuron refractär sein");
                    cycled = true;
                } else if pre_state == NeuronState::Refractory {
                    // Im refractären Zustand, überspringe Eingabe oder führe Zyklus durch
                    if i % 2 == 0 { // Manchmal Eingabe, manchmal Zyklus
                        let _ = neuron.receive_input(*input); // Sollte ignoriert werden
                    } else {
                        neuron.cycle(); // Zurück zum inaktiven Zustand
                    }
                }
            }

            // Abschließende Überprüfungen basierend auf aktiviertem Status
            if activated {
                // Wenn aktiviert, sollte das Neuron entweder aktiv oder refractär sein (nach Zyklus)
                let final_state = neuron.state();
                prop_assert!(final_state == NeuronState::Active ||
                           (final_state == NeuronState::Refractory && cycled) ||
                           (final_state == NeuronState::Inactive && cycled),
                           "Unerwarteter Endzustand: {:?} nach Aktivierung", final_state);
            } else if total_input >= threshold {
                // Diese Bedingung ist jetzt weniger wahrscheinlich, aber für alle Fälle
                // Vermeiden wir panic! und verwenden prop_assert! für bessere Fehlerbehandlung
                prop_assert!(false,
                    "Neuron nicht aktiviert trotz ausreichender Eingabe: total={:.6}, schwelle={:.6}\n{}",
                    total_input, threshold, params_info);
            }
            // Proptest-Funktionen geben implizit einen Test-Ergebnis-Typ zurück, kein explizites Ok(())
        }
    }
}

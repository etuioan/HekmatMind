//! Tests für das Wachstumsmodul
//!
//! Diese Datei enthält Unit-Tests für die Wachstumskomponenten,
//! einschließlich AxonGrowth, GrowthFactor, etc.

#[cfg(test)]
mod axon_tests {
    use crate::neural::growth::{AxonGrowth, FactorType, GrowthFactor, Position};
    use crate::neural::neuron::Neuron;
    use proptest::prelude::*;

    #[test]
    fn test_axon_growth_creation() {
        // Position und Energie-Parameter
        let position = Position::new(1.0, 2.0, 3.0);
        let energy = 50.0;

        // Erstellen eines AxonGrowth
        let axon_growth = AxonGrowth::new(position, energy);

        // Überprüfen der Eigenschaften
        assert_eq!(axon_growth.position(), position);
        assert_eq!(axon_growth.energy(), energy);
        assert_eq!(axon_growth.length(), 0.0);
    }

    #[test]
    fn test_axon_growth_state() {
        // AxonGrowth erstellen
        let position = Position::new(0.0, 0.0, 0.0);
        let energy = 100.0;
        let growth = AxonGrowth::new(position, energy);

        // Überprüfe die Eigenschaften
        assert_eq!(growth.energy(), 100.0);
        assert_eq!(growth.position(), position);
        assert_eq!(growth.length(), 0.0);
        assert_eq!(growth.direction(), [1.0, 0.0, 0.0]); // Standardrichtung

        // Kann wachsen prüfen
        assert!(growth.can_grow());
    }

    #[test]
    fn test_growth_factor_creation() {
        // Position definieren
        let position = Position::new(1.0, 2.0, 3.0);

        // Attraktiven Faktor erstellen
        let attractive = GrowthFactor::new(
            position,
            0.75, // Stärke
            10.0, // Radius
            FactorType::Attractive,
        );

        // Eigenschaften überprüfen
        assert_eq!(attractive.position, position);
        assert_eq!(attractive.radius, 10.0);
        assert_eq!(attractive.strength, 0.75);
        assert_eq!(attractive.factor_type, FactorType::Attractive);

        // Repulsiven Faktor erstellen
        let repulsive = GrowthFactor::new(
            position,
            0.3, // Stärke
            5.0, // Radius
            FactorType::Repulsive,
        );

        // Eigenschaften überprüfen
        assert_eq!(repulsive.position, position);
        assert_eq!(repulsive.radius, 5.0);
        assert_eq!(repulsive.strength, 0.3);
        assert_eq!(repulsive.factor_type, FactorType::Repulsive);
    }

    #[test]
    fn test_neuron_start_axon_growth() {
        // Erstellen eines Neurons
        // Erstelle ein Neuron mit einer benutzerdefinierten Position
        let position = Position::new(1.0, 2.0, 3.0);
        let neuron = Neuron::with_position(100, position);

        // Teste mit Standardenergie
        let axon_growth1 = neuron.start_axon_growth(None);

        // Die Energie sollte proportional zur Kapazität sein
        assert_eq!(axon_growth1.energy(), neuron.capacity() * 0.5);
        assert_eq!(axon_growth1.position(), *neuron.position());

        // Teste mit spezifizierter Energie
        let specific_energy = 42.0;
        let axon_growth2 = neuron.start_axon_growth(Some(specific_energy));

        // Die Energie sollte dem spezifizierten Wert entsprechen
        assert_eq!(axon_growth2.energy(), specific_energy);
        assert_eq!(axon_growth2.position(), *neuron.position());
    }

    #[test]
    fn test_neuron_as_growth_factor() {
        // Erstellen eines Neurons mit angepassten Eigenschaften und Position
        let position = Position::new(1.0, 2.0, 3.0);
        let mut neuron = Neuron::with_position(100, position);

        // Aktiviere das Neuron teilweise (um eine bestimmte Aktivierungsenergie zu erreichen)
        neuron.receive_input(5.0);

        // Teste als anziehendes/exzitatorisches Wachstumsfaktor
        let attractive = neuron.as_growth_factor(true);

        // Überprüfe Eigenschaften
        assert_eq!(attractive.position, *neuron.position());
        assert_eq!(attractive.factor_type, FactorType::Attractive);

        // Überprüfe, dass der Radius korrekt basierend auf der Neuronengeschwindigkeit berechnet wird
        let expected_radius = neuron.speed() as f32 * 0.2;
        assert_eq!(attractive.radius, expected_radius);

        // Überprüfe, dass die Stärke korrekt berechnet und auf den gültigen Bereich begrenzt wird
        // Die Stärke wird vom GrowthFactor-Konstruktor auf den Bereich 0.0-1.0 begrenzt
        let raw_strength = neuron.activation_energy() / neuron.threshold();
        let expected_strength = raw_strength.clamp(0.0, 1.0);
        assert_eq!(attractive.strength, expected_strength);

        // Teste als abstoßendes/inhibitorisches Wachstumsfaktor
        let repulsive = neuron.as_growth_factor(false);

        // Überprüfe Eigenschaften
        assert_eq!(repulsive.position, *neuron.position());
        assert_eq!(repulsive.factor_type, FactorType::Repulsive);

        // Die grundlegenden Faktoren (Radius, Stärke) sollten gleich sein
        assert_eq!(repulsive.radius, attractive.radius);
        assert_eq!(repulsive.strength, attractive.strength);
    }

    // Property-based Tests mit proptest
    proptest! {
        #[test]
        fn proptest_position_properties(
            x1 in -100.0f32..100.0,
            y1 in -100.0f32..100.0,
            z1 in -100.0f32..100.0
        ) {
            let pos = Position::new(x1, y1, z1);

            // Position-Eigenschaften testen
            assert_eq!(pos.x, x1);
            assert_eq!(pos.y, y1);
            assert_eq!(pos.z, z1);

            // Ein Position-Objekt sollte korrekt erstellt werden
            let pos_copy = Position::new(x1, y1, z1);
            assert_eq!(pos, pos_copy);
        }

        #[test]
        fn proptest_growth_factor_validity(
            x in -100.0f32..100.0,
            y in -100.0f32..100.0,
            z in -100.0f32..100.0,
            radius in 0.1f32..50.0,
            strength in 0.0f32..2.0
        ) {
            let pos = Position::new(x, y, z);
            let attractive_factor = GrowthFactor::new(
                pos,
                radius,
                strength,
                FactorType::Attractive
            );

            // Stärke sollte im gültigen Bereich sein
            assert!(attractive_factor.strength >= 0.0);
            assert!(attractive_factor.strength <= 1.0);

            // Radius sollte positiv sein
            assert!(attractive_factor.radius > 0.0);

            // Position sollte erhalten bleiben
            assert_eq!(attractive_factor.position.x, x);
            assert_eq!(attractive_factor.position.y, y);
            assert_eq!(attractive_factor.position.z, z);
        }
    }
}

#[cfg(test)]
mod dendritic_tests {
    use crate::neural::growth::{
        DendriteResourceManager, DendriticSegment, DendriticTree, Position, Synapse,
    };

    use uuid::Uuid;

    #[test]
    fn test_dendritic_segment_creation() {
        let position = Position::new(1.0, 2.0, 3.0);
        let segment = DendriticSegment::new(position, 10.0, 0, None);

        assert_eq!(segment.position(), position);
        // parent_id ist ein privates Feld, nicht über Methode zugreifbar
        // Test muss über öffentliche API erfolgen
        assert!(segment.child_ids().is_empty());
    }

    #[test]
    fn test_dendritic_tree_basics() {
        // Erstelle einen Baum mit Neuron-ID und Energie
        let neuron_id = Uuid::new_v4();
        let initial_energy = 100.0;
        let tree = DendriticTree::new(neuron_id, initial_energy);

        // Überprüfe Initialwerte
        assert_eq!(tree.neuron_id(), neuron_id);
        assert_eq!(tree.energy(), initial_energy);

        // Da DendriticTree komplex ist und viele interne Implementierungsdetails hat,
        // testen wir hier nur die öffentliche API
        // In der aktuellen Implementierung wird der Baum ohne Segmente initialisiert
        assert_eq!(tree.segment_count(), 0); // Kein Segment zu Beginn
    }

    #[test]
    fn test_synapse_lifecycle() {
        // Erstelle eine Synapse
        let source_id = Uuid::new_v4();
        let post_position = Position::new(1.0, 1.0, 1.0);
        let electrotonic_distance = 0.1;
        let mut synapse = Synapse::new(source_id, post_position, electrotonic_distance);

        // Überprüfe Initialwerte
        assert_eq!(synapse.weight(), 0.1); // Standardgewicht in der aktuellen Implementierung

        // Eigenschaften prüfen
        assert_eq!(synapse.source_id(), source_id);
        assert_eq!(synapse.electrotonic_distance(), electrotonic_distance);

        // Effektive Stärke sollte geringer sein als das Gewicht aufgrund der elektrotonischen Dämpfung
        let effective = synapse.effective_strength();
        assert!(
            effective < synapse.weight(),
            "Effektive Stärke {} sollte kleiner sein als Gewicht {}",
            effective,
            synapse.weight()
        );

        // Stärken mit hohem Wert
        let original_weight = synapse.weight(); // Wird in den Assertions verwendet
        synapse.strengthen(1.0); // Maximalen Wert verwenden
        let new_weight = synapse.weight();
        // Prüfen mit besserer Fehlermeldung
        assert!(
            new_weight > original_weight,
            "Neues Gewicht {} sollte größer sein als Original {}",
            new_weight,
            original_weight
        );

        // Schwächen mit hohem Wert
        let strengthened_weight = synapse.weight();
        synapse.weaken(1.0); // Maximalen Wert verwenden
        let final_weight = synapse.weight();
        assert!(
            final_weight < strengthened_weight || final_weight <= 0.01,
            "Finales Gewicht {} sollte kleiner sein als gestärktes Gewicht {} oder Minimalwert 0.01",
            final_weight,
            strengthened_weight
        );
    }

    /// Detaillierter Debug-Test für Synapse-Lebenszyklus mit Protokollierung in Datei
    ///
    /// Dieser Test wird standardmäßig ignoriert. Er kann explizit ausgeführt werden, um
    /// Probleme mit der Synapse-Implementierung zu diagnostizieren, indem er detaillierte
    /// Protokollinformationen in die Datei `/tmp/synapse_debug.log` schreibt.
    ///
    /// Ausführen mit: `cargo test synapsen_debug_protokoll -- --ignored`
    #[test]
    #[ignore]
    fn synapsen_debug_protokoll() {
        use std::fs::File;
        use std::io::Write;

        // Erstelle Log-Datei mit Zeitstempel
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let log_pfad = format!("/tmp/synapse_debug_{}.log", timestamp);

        let mut debug_file = File::create(&log_pfad).expect("Konnte Debug-Datei nicht erstellen");

        writeln!(
            &mut debug_file,
            "=== Synapsen-Debug-Protokoll ({}), Zeitstempel: {} ===",
            timestamp, timestamp
        )
        .unwrap();

        // Erstelle eine Synapse
        let source_id = Uuid::new_v4();
        let post_position = Position::new(1.0, 1.0, 1.0);
        let electrotonic_distance = 0.1;

        writeln!(&mut debug_file, "[SCHRITT 1] Erstelle Synapse...").unwrap();
        let mut synapse = Synapse::new(source_id, post_position, electrotonic_distance);
        writeln!(&mut debug_file, "Synapse erstellt mit ID: {}", synapse.id()).unwrap();

        // Überprüfe Initialwerte
        writeln!(&mut debug_file, "\n[SCHRITT 2] Prüfe Initialwerte").unwrap();
        let initial_weight = synapse.weight();
        writeln!(&mut debug_file, "Initial weight: {}", initial_weight).unwrap();

        // Eigenschaften prüfen
        writeln!(&mut debug_file, "Source ID: {}", synapse.source_id()).unwrap();
        writeln!(
            &mut debug_file,
            "Electrotonic distance: {}",
            synapse.electrotonic_distance()
        )
        .unwrap();

        // Effektive Stärke
        writeln!(&mut debug_file, "\n[SCHRITT 3] Berechne effektive Stärke").unwrap();
        let effective = synapse.effective_strength();
        writeln!(&mut debug_file, "Effective strength: {}", effective).unwrap();
        writeln!(
            &mut debug_file,
            "Dämpfungsfaktor: {}",
            effective / synapse.weight()
        )
        .unwrap();

        // Stärken schrittweise mit verschiedenen Werten
        writeln!(
            &mut debug_file,
            "\n[SCHRITT 4] Teste Verstärkung mit verschiedenen Werten"
        )
        .unwrap();
        let _original_weight = synapse.weight(); // Für Debug-Zwecke aufbewahren

        let test_strengthen_values = [0.1, 0.5, 1.0];
        for &value in &test_strengthen_values {
            let weight_before = synapse.weight();
            synapse.strengthen(value);
            let weight_after = synapse.weight();
            writeln!(
                &mut debug_file,
                "strengthen({}) ändert Gewicht: {} -> {} (Delta: {})",
                value,
                weight_before,
                weight_after,
                weight_after - weight_before
            )
            .unwrap();
        }

        // Schwächen schrittweise mit verschiedenen Werten
        writeln!(
            &mut debug_file,
            "\n[SCHRITT 5] Teste Abschwächung mit verschiedenen Werten"
        )
        .unwrap();

        let test_weaken_values = [0.01, 0.05, 0.1, 0.5, 1.0];
        for &value in &test_weaken_values {
            let weight_before = synapse.weight();
            let state_before = format!("{:?}", synapse.state());

            writeln!(
                &mut debug_file,
                "VOR weaken({}) - Gewicht: {}, Zustand: {}",
                value, weight_before, state_before
            )
            .unwrap();

            synapse.weaken(value);

            let weight_after = synapse.weight();
            let state_after = format!("{:?}", synapse.state());

            writeln!(
                &mut debug_file,
                "NACH weaken({}) - Gewicht: {} -> {} (Delta: {}), Zustand: {} -> {}",
                value,
                weight_before,
                weight_after,
                weight_after - weight_before,
                state_before,
                state_after
            )
            .unwrap();
        }

        // Finale Überprüfung aller Eigenschaften
        writeln!(&mut debug_file, "\n[SCHRITT 6] Finale Werte").unwrap();
        writeln!(&mut debug_file, "Gewicht: {}", synapse.weight()).unwrap();
        writeln!(&mut debug_file, "Zustand: {:?}", synapse.state()).unwrap();
        writeln!(
            &mut debug_file,
            "Effektive Stärke: {}",
            synapse.effective_strength()
        )
        .unwrap();

        println!(
            "Debug-Test abgeschlossen. Protokoll gespeichert in: {}",
            log_pfad
        );
        writeln!(
            &mut debug_file,
            "\n=== Test erfolgreich abgeschlossen ===\n"
        )
        .unwrap();
    }

    #[test]
    fn test_resource_manager() {
        let resource_manager = DendriteResourceManager::new(100.0);

        // Teste Grundfunktionalität
        assert_eq!(resource_manager.available_energy(), 100.0);

        // Teste Energiehinzufügen
        // DendriteResourceManager implementiert Clone nicht
        let mut manager = DendriteResourceManager::new(100.0);
        manager.add_energy(50.0);
        assert_eq!(manager.available_energy(), 150.0);
    }
}

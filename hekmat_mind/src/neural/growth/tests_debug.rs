use crate::neural::growth::{Position, Synapse};
use uuid::Uuid;

#[test]
fn debug_synapse_lifecycle() {
    // Erstelle eine Synapse
    let source_id = Uuid::new_v4();
    let post_position = Position::new(1.0, 1.0, 1.0);
    let electrotonic_distance = 0.1;
    let mut synapse = Synapse::new(source_id, post_position, electrotonic_distance);

    // Überprüfe Initialwerte
    let initial_weight = synapse.weight();
    println!("Initial weight: {}", initial_weight);

    // Eigenschaften prüfen
    println!("Source ID: {}", synapse.source_id());
    println!("Electrotonic distance: {}", synapse.electrotonic_distance());

    // Effektive Stärke
    let effective = synapse.effective_strength();
    println!("Effective strength: {}", effective);

    // Stärken
    println!("Strengthening synapse...");
    synapse.strengthen(0.5);
    println!("New weight after strengthening: {}", synapse.weight());

    // Schwächen
    let strengthened_weight = synapse.weight();
    println!("Weakening synapse...");
    synapse.weaken(0.2);
    println!("New weight after weakening: {}", synapse.weight());

    println!("Test completed successfully");
}

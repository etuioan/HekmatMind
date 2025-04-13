use crate::neural::growth::types::Position;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Konstante Wachstumsparameter
pub mod constants {
    /// Basisgeschwindigkeit des Axonwachstums (µm/Tag)
    pub const BASE_GROWTH_RATE: f32 = 10.0;

    /// Maximaler Einfluss von Faktoren auf die Wachstumsrate
    pub const MAX_FACTOR_INFLUENCE: f32 = 5.0;

    /// Energieverbrauch pro Einheit Wachstum
    pub const ENERGY_PER_GROWTH_UNIT: f32 = 1.0;

    /// Minimale Energie für Wachstumsfähigkeit
    pub const MIN_ENERGY_THRESHOLD: f32 = 5.0;
}

/// Arten von Wachstumsfaktoren
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactorType {
    /// Anziehender chemischer Faktor
    Attractive,
    /// Abstoßender chemischer Faktor
    Repulsive,
    /// Physikalisches Hindernis
    Obstacle,
}

/// Chemischer oder physikalischer Wachstumsfaktor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthFactor {
    /// Position des Faktors
    pub position: Position,
    /// Stärke des Faktors (0.0-1.0)
    pub strength: f32,
    /// Wirkungsradius
    pub radius: f32,
    /// Art des Faktors
    pub factor_type: FactorType,
}

impl GrowthFactor {
    /// Erstellt einen neuen Wachstumsfaktor
    pub fn new(position: Position, strength: f32, radius: f32, factor_type: FactorType) -> Self {
        Self {
            position,
            // Stärke auf gültigen Bereich begrenzen
            strength: strength.clamp(0.0, 1.0),
            // Radius muss positiv sein
            radius: radius.max(0.1),
            factor_type,
        }
    }

    /// Berechnet den Einfluss auf eine Position
    pub fn influence_at(&self, position: &Position) -> f32 {
        let distance = self.position.distance_to(position);

        // Außerhalb des Radius kein Einfluss
        if distance > self.radius {
            return 0.0;
        }

        // Einfluss nimmt mit der Distanz ab
        let relative_distance = distance / self.radius;
        let base_influence = self.strength * (1.0 - relative_distance);

        match self.factor_type {
            FactorType::Attractive => base_influence,
            FactorType::Repulsive => -base_influence,
            FactorType::Obstacle => {
                if distance < self.radius * 0.5 {
                    -2.0 // Noch stärkere Abstoßung nahe am Hindernis
                } else {
                    -base_influence * 1.5 // Verstärkte Abstoßung für bessere Hindernisvermeidung
                }
            }
        }
    }
}

/// Messdaten für das Axonwachstum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthMeasurement {
    /// Zeitpunkt der Messung
    pub time: f32,
    /// Länge des Axons zum Zeitpunkt
    pub length: f32,
    /// Wachstumsrate zum Zeitpunkt
    pub growth_rate: f32,
    /// Anzahl der Verzweigungen
    pub branches: usize,
}

/// Hauptstruktur für axonales Wachstum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxonGrowth {
    /// Aktuelle Position der Wachstumsspitze
    position: Position,
    /// Ursprüngliche Position des Neurons
    origin: Position,
    /// Aktuelle Wachstumsrichtung
    direction: [f32; 3],
    /// Verfügbare Energie
    energy: f32,
    /// Zurückgelegte Segmente
    segments: Vec<Position>,
    /// Gesamtlänge des Axons
    length: f32,
    /// Messdaten für Validierung
    measurements: VecDeque<GrowthMeasurement>,
    /// Zeitverlauf (Tage)
    time: f32,
}

impl AxonGrowth {
    /// Erstellt ein neues Axonwachstumsmodell
    pub fn new(position: Position, initial_energy: f32) -> Self {
        Self {
            position,
            origin: position,
            direction: [1.0, 0.0, 0.0], // Standardrichtung
            energy: initial_energy,
            segments: vec![position],
            length: 0.0,
            measurements: VecDeque::with_capacity(100),
            time: 0.0,
        }
    }

    /// Gibt die aktuelle Position zurück
    pub fn position(&self) -> Position {
        self.position
    }

    /// Gibt die verfügbare Energie zurück
    pub fn energy(&self) -> f32 {
        self.energy
    }

    /// Gibt die Gesamtlänge des Axons zurück
    pub fn length(&self) -> f32 {
        self.length
    }

    /// Gibt die Wachstumsrichtung zurück
    pub fn direction(&self) -> [f32; 3] {
        self.direction
    }

    /// Gibt die Wachstumsmessungen zurück
    pub fn measurements(&self) -> &VecDeque<GrowthMeasurement> {
        &self.measurements
    }

    /// Prüft, ob Wachstum möglich ist
    pub fn can_grow(&self) -> bool {
        self.energy >= constants::MIN_ENERGY_THRESHOLD
    }

    /// Führt einen Wachstumsschritt durch
    ///
    /// # Arguments
    /// * `factors` - Liste von Wachstumsfaktoren
    /// * `time_step` - Zeitschritt in Tagen
    ///
    /// # Returns
    /// Tatsächliche Wachstumsstrecke in diesem Schritt
    pub fn grow(&mut self, factors: &[GrowthFactor], time_step: f32) -> f32 {
        if !self.can_grow() {
            return 0.0;
        }

        // Basiswachstumsrate
        let base_rate = constants::BASE_GROWTH_RATE;

        // Einfluss aller Faktoren berechnen
        let mut total_influence = 0.0;
        let mut direction_change = [0.0, 0.0, 0.0];

        // Zufällige kleine Ablenkung für natürlicheres Wachstum (verhindert perfekt gerade Linien)
        if self.segments.len() % 3 == 0 {
            // Jedes dritte Segment leichte Zufallsbewegung hinzufügen
            use std::f32::consts::PI;
            let noise_angle = (self.time * 7.0) % (2.0 * PI); // Deterministisches "Rauschen"
            direction_change[1] += noise_angle.sin() * 0.05;
            direction_change[2] += noise_angle.cos() * 0.05;
        }

        for factor in factors {
            let influence = factor.influence_at(&self.position);
            total_influence += influence;

            // Richtungsänderung basierend auf Faktorposition
            if influence != 0.0 {
                let dx = factor.position.x - self.position.x;
                let dy = factor.position.y - self.position.y;
                let dz = factor.position.z - self.position.z;

                let distance = (dx * dx + dy * dy + dz * dz).sqrt();
                if distance > 0.001 {
                    let normalized_influence = influence / distance;

                    // Bei Hindernissen drehen wir 90° von der Hindernisachse weg
                    if factor.factor_type == FactorType::Obstacle && distance < factor.radius {
                        // Orthogonale Richtung zum Hindernis berechnen
                        let ortho_y = dz;
                        let ortho_z = -dy;
                        direction_change[0] += -dx * normalized_influence * 0.5; // Rückwärtsbewegung reduzieren
                        direction_change[1] += ortho_y * normalized_influence.abs() * 2.0;
                        direction_change[2] += ortho_z * normalized_influence.abs() * 2.0;
                    } else {
                        direction_change[0] += dx * normalized_influence;
                        direction_change[1] += dy * normalized_influence;
                        direction_change[2] += dz * normalized_influence;
                    }
                }
            }
        }

        // Richtung anpassen (mit Trägheit)
        if total_influence.abs() > 0.0 || direction_change[1] != 0.0 || direction_change[2] != 0.0 {
            let mag = (direction_change[0] * direction_change[0]
                + direction_change[1] * direction_change[1]
                + direction_change[2] * direction_change[2])
                .sqrt();

            if mag > 0.001 {
                // Normalisieren
                direction_change[0] /= mag;
                direction_change[1] /= mag;
                direction_change[2] /= mag;

                // Neue Richtung mit Trägheit (70% alte Richtung, 30% neue Einflüsse für stärkere Anpassung)
                self.direction[0] = 0.7 * self.direction[0] + 0.3 * direction_change[0];
                self.direction[1] = 0.7 * self.direction[1] + 0.3 * direction_change[1];
                self.direction[2] = 0.7 * self.direction[2] + 0.3 * direction_change[2];

                // Renormalisieren
                let new_mag = (self.direction[0] * self.direction[0]
                    + self.direction[1] * self.direction[1]
                    + self.direction[2] * self.direction[2])
                    .sqrt();

                if new_mag > 0.001 {
                    self.direction[0] /= new_mag;
                    self.direction[1] /= new_mag;
                    self.direction[2] /= new_mag;
                }
            }
        }

        // Wachstumsrate modifizieren basierend auf Faktoren (zwischen 0.5x und 1.5x)
        let modifier = 1.0 + (total_influence / constants::MAX_FACTOR_INFLUENCE).clamp(-0.5, 0.5);
        let growth_rate = base_rate * modifier;

        // Tatsächliches Wachstum für diesen Zeitschritt
        let growth_amount = growth_rate * time_step;

        // Energieverbrauch
        let energy_cost = growth_amount * constants::ENERGY_PER_GROWTH_UNIT;

        // Prüfen, ob genug Energie vorhanden ist
        if self.energy < energy_cost {
            return 0.0;
        }

        // Energie verbrauchen
        self.energy -= energy_cost;

        // Position aktualisieren
        self.position.x += self.direction[0] * growth_amount;
        self.position.y += self.direction[1] * growth_amount;
        self.position.z += self.direction[2] * growth_amount;

        // Segment hinzufügen und Länge aktualisieren
        self.segments.push(self.position);
        self.length += growth_amount;

        // Zeit aktualisieren
        self.time += time_step;

        // Messdaten speichern (alle 0.5 Tage)
        if self.measurements.is_empty()
            || (self.time - self.measurements.back().unwrap().time) >= 0.5
        {
            self.measurements.push_back(GrowthMeasurement {
                time: self.time,
                length: self.length,
                growth_rate,
                branches: 0, // Noch keine Verzweigungen in diesem Basismodell
            });

            // Maximal 100 Messungen behalten
            if self.measurements.len() > 100 {
                self.measurements.pop_front();
            }
        }

        growth_amount
    }

    /// Fügt Energie hinzu (z.B. durch Stoffwechsel)
    pub fn add_energy(&mut self, amount: f32) {
        self.energy += amount;
    }

    /// Gibt die durchschnittliche Wachstumsrate zurück
    pub fn average_growth_rate(&self) -> f32 {
        if self.measurements.len() < 2 {
            return 0.0;
        }

        self.length / self.time
    }

    /// Exportiert Messdaten für empirische Validierung
    pub fn export_measurements(&self) -> Vec<GrowthMeasurement> {
        self.measurements.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axon_basic_growth() {
        let initial_position = Position::new(0.0, 0.0, 0.0);
        let mut axon = AxonGrowth::new(initial_position, 100.0);

        // Wachstum ohne Faktoren (in X-Richtung)
        let growth = axon.grow(&[], 1.0);

        assert!(growth > 0.0);
        assert!(axon.position().x > 0.0);
        assert_eq!(axon.position().y, 0.0);
        assert_eq!(axon.position().z, 0.0);
        assert!(axon.energy() < 100.0);
    }

    #[test]
    fn test_growth_factor_influence() {
        let initial_position = Position::new(0.0, 0.0, 0.0);
        let mut axon = AxonGrowth::new(initial_position, 100.0);

        // Attraktiver Faktor in Y-Richtung
        let attractive = GrowthFactor::new(
            Position::new(0.0, 10.0, 0.0),
            0.8,
            15.0,
            FactorType::Attractive,
        );

        // Wachstum mit attraktivem Faktor
        for _ in 0..5 {
            axon.grow(&[attractive.clone()], 1.0);
        }

        // Richtung sollte Y-Komponente haben
        assert!(axon.position().y > 0.0);
    }

    #[test]
    fn test_obstacle_avoidance() {
        let initial_position = Position::new(0.0, 0.0, 0.0);
        let mut axon = AxonGrowth::new(initial_position, 50.0);

        // Ein Hindernis direkt im Weg mit geringer Distanz und kleinem Zeitschritt
        let obstacle = GrowthFactor::new(
            Position::new(1.0, 0.0, 0.0), // Nah genug, um Einfluss zu haben
            1.0,
            2.0,
            FactorType::Obstacle,
        );

        // Sehr kleinen Zeitschritt verwenden, um das Hindernis nicht zu überspringen
        for i in 0..20 {
            let _growth = axon.grow(&[obstacle.clone()], 0.1);

            // Wenn wir genug gewachsen sind, sollten wir vom Pfad abweichen
            if i > 5 && axon.position().x > 0.5 {
                println!(
                    "DEBUG: Axon Position in Iteration {}: ({}, {}, {})",
                    i,
                    axon.position().x,
                    axon.position().y,
                    axon.position().z
                );
                // Wenn wir nahe genug am Hindernis sind, sollte eine Abweichung messbar sein
                if axon.position().distance_to(&obstacle.position) < 1.5 {
                    break;
                }
            }
        }

        println!(
            "DEBUG: Finale Axon Position: ({}, {}, {})",
            axon.position().x,
            axon.position().y,
            axon.position().z
        );

        // Abweichung von der X-Achse messen
        let deviation = axon.position().y.abs() + axon.position().z.abs();

        assert!(
            deviation > 0.01,
            "Axon hat keine signifikante Ausweichbewegung gemacht: Position = ({},{},{}), Abweichung = {}",
            axon.position().x,
            axon.position().y,
            axon.position().z,
            deviation
        );
    }

    #[test]
    fn test_direct_obstacle_influence() {
        // Position direkt vor einem Hindernis
        let pos = Position::new(1.0, 0.0, 0.0);
        let obstacle =
            GrowthFactor::new(Position::new(2.0, 0.0, 0.0), 1.0, 2.0, FactorType::Obstacle);

        // Einfluss testen
        let influence = obstacle.influence_at(&pos);
        assert!(influence < 0.0, "Hindernis sollte negativen Einfluss haben");
        assert!(
            influence < -0.5,
            "Hindernis sollte starken negativen Einfluss haben, war: {}",
            influence
        );
    }

    #[test]
    fn test_energy_depletion() {
        let initial_position = Position::new(0.0, 0.0, 0.0);
        let mut axon = AxonGrowth::new(initial_position, 20.0);

        // Wachstum bis Energie erschöpft ist
        let mut total_growth = 0.0;
        let mut steps = 0;

        loop {
            let growth = axon.grow(&[], 1.0);
            if growth == 0.0 {
                break;
            }

            total_growth += growth;
            steps += 1;

            // Sicherheitsabbruch
            if steps > 100 {
                panic!("Test did not complete as expected");
            }
        }

        assert!(axon.energy() < constants::MIN_ENERGY_THRESHOLD);
        assert!(total_growth > 0.0);
        assert!(!axon.can_grow());
    }

    #[test]
    fn test_measurements_recording() {
        let initial_position = Position::new(0.0, 0.0, 0.0);
        let mut axon = AxonGrowth::new(initial_position, 100.0);

        // Mehrere Wachstumsschritte
        for _ in 0..10 {
            axon.grow(&[], 0.5);
        }

        let measurements = axon.measurements();

        assert!(!measurements.is_empty());
        assert_eq!(measurements[0].time, 0.5); // Erste Messung nach 0.5 Tagen
        assert!(measurements.back().unwrap().length > 0.0);

        // Durchschnittliche Wachstumsrate sollte positiv sein
        assert!(axon.average_growth_rate() > 0.0);
    }

    #[test]
    fn test_growth_rate_modulation() {
        let initial_position = Position::new(0.0, 0.0, 0.0);

        // Test mit attraktivem Faktor
        let mut axon_attracted = AxonGrowth::new(initial_position, 100.0);
        let attractive = GrowthFactor::new(
            Position::new(10.0, 0.0, 0.0),
            1.0,
            15.0,
            FactorType::Attractive,
        );

        // Test mit abstoßendem Faktor
        let mut axon_repelled = AxonGrowth::new(initial_position, 100.0);
        let repulsive = GrowthFactor::new(
            Position::new(10.0, 0.0, 0.0),
            1.0,
            15.0,
            FactorType::Repulsive,
        );

        // Beide wachsen lassen
        for _ in 0..5 {
            axon_attracted.grow(&[attractive.clone()], 1.0);
            axon_repelled.grow(&[repulsive.clone()], 1.0);
        }

        // Angezogenes Axon sollte schneller wachsen
        assert!(axon_attracted.length() > axon_repelled.length());
    }
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Konstanten für Synapsen-Parameter
pub mod constants {
    /// Standardverzögerung für synaptische Übertragung in Sekunden
    pub const DEFAULT_DELAY: f32 = 0.001; // 1ms

    /// Maximale synaptische Verzögerung in Sekunden
    pub const MAX_DELAY: f32 = 0.020; // 20ms

    /// Dauer des aktiven Zustands einer Synapse nach Übertragung in Sekunden
    pub const ACTIVE_DURATION: f32 = 0.005; // 5ms
}

/// Repräsentiert eine synaptische Verbindung zwischen zwei Neuronen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Synapse {
    /// ID des präsynaptischen Neurons
    pre_neuron_id: Uuid,

    /// ID des postsynaptischen Neurons
    post_neuron_id: Uuid,

    /// Synaptisches Gewicht (0.0 bis 1.0)
    weight: f32,

    /// Verzögerung der Signalübertragung in Sekunden
    delay: f32,

    /// Gibt an, ob die Synapse gerade aktiv ist
    active: bool,

    /// Verbleibende Zeit im aktiven Zustand
    active_time_remaining: f32,
}

impl Synapse {
    /// Erstellt eine neue Synapse zwischen zwei Neuronen
    ///
    /// # Arguments
    ///
    /// * `pre_neuron_id` - ID des präsynaptischen Neurons
    /// * `post_neuron_id` - ID des postsynaptischen Neurons
    /// * `weight` - Anfangsgewicht der Synapse (0.0-1.0)
    pub fn new(pre_neuron_id: Uuid, post_neuron_id: Uuid, weight: f32) -> Self {
        Self {
            pre_neuron_id,
            post_neuron_id,
            weight: weight.clamp(0.0, 1.0),
            delay: constants::DEFAULT_DELAY,
            active: false,
            active_time_remaining: 0.0,
        }
    }

    /// Gibt die ID des präsynaptischen Neurons zurück
    pub fn pre_neuron_id(&self) -> &Uuid {
        &self.pre_neuron_id
    }

    /// Gibt die ID des postsynaptischen Neurons zurück
    pub fn post_neuron_id(&self) -> &Uuid {
        &self.post_neuron_id
    }

    /// Gibt das aktuelle Gewicht der Synapse zurück
    pub fn weight(&self) -> f32 {
        self.weight
    }

    /// Gibt die Verzögerung der Synapse zurück
    pub fn delay(&self) -> f32 {
        self.delay
    }

    /// Gibt an, ob die Synapse aktiv ist
    pub fn active_state(&self) -> bool {
        self.active
    }

    /// Überträgt ein Signal durch die Synapse
    ///
    /// # Arguments
    ///
    /// * `input` - Das eingehende Signal vom präsynaptischen Neuron
    ///
    /// # Returns
    ///
    /// Das gewichtete Signal, das zum postsynaptischen Neuron gesendet wird
    pub fn transmit(&mut self, input: f32) -> f32 {
        self.active = true;
        self.active_time_remaining = constants::ACTIVE_DURATION;
        input * self.weight
    }

    /// Aktualisiert den Zustand der Synapse
    ///
    /// # Arguments
    ///
    /// * `time_step` - Zeitschritt in Sekunden
    pub fn update(&mut self, time_step: f32) {
        if self.active {
            self.active_time_remaining -= time_step;
            if self.active_time_remaining <= 0.0 {
                self.active = false;
                self.active_time_remaining = 0.0;
            }
        }
    }

    /// Wendet Hebbsches Lernen auf die Synapse an
    ///
    /// # Arguments
    ///
    /// * `pre_active` - Gibt an, ob das präsynaptische Neuron aktiv ist
    /// * `post_active` - Gibt an, ob das postsynaptische Neuron aktiv ist
    /// * `plasticity_rate` - Lernrate für die Gewichtsanpassung
    pub fn apply_hebbian_plasticity(
        &mut self,
        pre_active: bool,
        post_active: bool,
        plasticity_rate: f32,
    ) {
        // Hebbsches Lernen: "Neurons that fire together, wire together"
        if pre_active && post_active {
            // Verstärkung bei gemeinsamer Aktivität
            self.weight += plasticity_rate * (1.0 - self.weight);
        } else if pre_active && !post_active {
            // Abschwächung bei präsynaptischer, aber nicht postsynaptischer Aktivität
            self.weight -= plasticity_rate * self.weight;
        }

        // Gewicht auf gültigen Bereich beschränken
        self.weight = self.weight.clamp(0.0, 1.0);
    }

    /// Setzt das Gewicht der Synapse direkt
    ///
    /// # Arguments
    ///
    /// * `new_weight` - Neues Gewicht (wird auf 0.0-1.0 begrenzt)
    pub fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight.clamp(0.0, 1.0);
    }

    /// Setzt die Verzögerung der Synapse
    ///
    /// # Arguments
    ///
    /// * `new_delay` - Neue Verzögerung in Sekunden (wird auf MAX_DELAY begrenzt)
    pub fn set_delay(&mut self, new_delay: f32) {
        self.delay = new_delay.clamp(0.0, constants::MAX_DELAY);
    }
}

/// Builder für Synapsen zur flexiblen Erstellung
pub struct SynapseBuilder {
    pre_neuron_id: Option<Uuid>,
    post_neuron_id: Option<Uuid>,
    weight: f32,
    delay: f32,
}

impl SynapseBuilder {
    /// Erstellt einen neuen SynapseBuilder mit Standardwerten
    pub fn new() -> Self {
        Self {
            pre_neuron_id: None,
            post_neuron_id: None,
            weight: 0.5, // Standardgewicht
            delay: constants::DEFAULT_DELAY,
        }
    }

    /// Setzt die ID des präsynaptischen Neurons
    pub fn with_pre_neuron_id(mut self, id: Uuid) -> Self {
        self.pre_neuron_id = Some(id);
        self
    }

    /// Setzt die ID des postsynaptischen Neurons
    pub fn with_post_neuron_id(mut self, id: Uuid) -> Self {
        self.post_neuron_id = Some(id);
        self
    }

    /// Setzt das anfängliche Gewicht
    pub fn with_weight(mut self, weight: f32) -> Self {
        self.weight = weight;
        self
    }

    /// Setzt die Verzögerung
    pub fn with_delay(mut self, delay: f32) -> Self {
        self.delay = delay;
        self
    }

    /// Erstellt die Synapse
    pub fn build(self) -> Synapse {
        // Sicherstellen, dass die erforderlichen Felder gesetzt sind
        let pre_id = self
            .pre_neuron_id
            .expect("Präsynaptische Neuron-ID muss gesetzt sein");
        let post_id = self
            .post_neuron_id
            .expect("Postsynaptische Neuron-ID muss gesetzt sein");

        let mut synapse = Synapse::new(pre_id, post_id, self.weight);
        synapse.set_delay(self.delay);

        synapse
    }
}

impl Default for SynapseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

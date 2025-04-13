use crate::neural::growth::{AxonGrowth, GrowthFactor, Position};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Die Konstanten für die Neuronen-Parameter
pub mod constants {
    /// Minimale Neuron-Geschwindigkeit
    pub const MIN_SPEED: u16 = 1;
    /// Maximale Neuron-Geschwindigkeit
    pub const MAX_SPEED: u16 = 1000;
    /// Standardwert für den Aktivierungsschwellwert
    pub const DEFAULT_THRESHOLD: f32 = 0.5;
    /// Faktor für die Berechnung der Kapazität aus der Geschwindigkeit
    pub const CAPACITY_FACTOR: f32 = 1.5;
    /// Standardwert für die Plastizitätsrate
    pub const DEFAULT_PLASTICITY_RATE: f32 = 0.01;
}

/// Zustand eines Neurons (inaktiv, aktiviert, refraktär)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeuronState {
    /// Inaktiv: Neuron kann Eingaben empfangen und aktiviert werden
    Inactive,
    /// Aktiviert: Neuron hat den Schwellwert überschritten und sendet Signale
    Active,
    /// Refraktär: Neuron ist in Erholungsphase und kann nicht aktiviert werden
    Refractory,
}

impl Default for NeuronState {
    fn default() -> Self {
        Self::Inactive
    }
}

impl fmt::Display for NeuronState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NeuronState::Inactive => write!(f, "Inaktiv"),
            NeuronState::Active => write!(f, "Aktiv"),
            NeuronState::Refractory => write!(f, "Refraktär"),
        }
    }
}

/// Grundlegende Neuronen-Implementierung
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neuron {
    /// Eindeutige ID des Neurons
    id: Uuid,
    /// Geschwindigkeit (1-1000) beeinflusst die Verarbeitungsgeschwindigkeit
    speed: u16,
    /// Aktuelle Aktivierungsenergie des Neurons
    activation_energy: f32,
    /// Schwellwert für die Aktivierung
    threshold: f32,
    /// Aktueller Zustand des Neurons
    state: NeuronState,
    /// Plastizitätsrate: Geschwindigkeit der Anpassung des Schwellwerts
    plasticity_rate: f32,
    /// Position des Neurons im 3D-Raum
    position: Position,
}

impl Neuron {
    /// Erstellt ein neues Neuron mit der angegebenen Geschwindigkeit
    ///
    /// # Arguments
    ///
    /// * `speed` - Die Geschwindigkeit des Neurons (1-1000)
    ///
    /// # Returns
    ///
    /// Ein neues Neuron mit den Standardwerten für die anderen Parameter
    pub fn new(speed: u16) -> Self {
        let speed = speed.clamp(constants::MIN_SPEED, constants::MAX_SPEED);

        Self {
            id: Uuid::new_v4(),
            speed,
            activation_energy: 0.0,
            threshold: constants::DEFAULT_THRESHOLD,
            state: NeuronState::default(),
            plasticity_rate: constants::DEFAULT_PLASTICITY_RATE,
            position: Position::new(0.0, 0.0, 0.0), // Standardposition im Ursprung
        }
    }

    /// Erstellt ein neues Neuron mit benutzerdefinierten Parametern
    ///
    /// # Arguments
    ///
    /// * `speed` - Die Geschwindigkeit des Neurons (1-1000)
    /// * `threshold` - Der Aktivierungsschwellwert
    /// * `plasticity_rate` - Die Plastizitätsrate für Anpassungen
    ///
    /// # Returns
    ///
    /// Ein neues Neuron mit den angegebenen Parametern
    pub fn with_params(speed: u16, threshold: f32, plasticity_rate: f32) -> Self {
        let speed = speed.clamp(constants::MIN_SPEED, constants::MAX_SPEED);

        Self {
            id: Uuid::new_v4(),
            speed,
            activation_energy: 0.0,
            threshold,
            state: NeuronState::default(),
            plasticity_rate,
            position: Position::new(0.0, 0.0, 0.0), // Standardposition im Ursprung
        }
    }

    /// Berechnet die Informationskapazität des Neurons basierend auf seiner Geschwindigkeit
    ///
    /// # Returns
    ///
    /// Die berechnete Kapazität als f32-Wert
    pub fn capacity(&self) -> f32 {
        self.speed as f32 * constants::CAPACITY_FACTOR
    }

    /// Gibt die eindeutige ID des Neurons zurück
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Gibt die Geschwindigkeit des Neurons zurück
    pub fn speed(&self) -> u16 {
        self.speed
    }

    /// Gibt den aktuellen Aktivierungsschwellwert zurück
    pub fn threshold(&self) -> f32 {
        self.threshold
    }

    /// Gibt den aktuellen Zustand des Neurons zurück
    pub fn state(&self) -> NeuronState {
        self.state
    }

    /// Gibt die aktuelle Aktivierungsenergie zurück
    pub fn activation_energy(&self) -> f32 {
        self.activation_energy
    }

    /// Gibt die Plastizitätsrate des Neurons zurück
    pub fn plasticity_rate(&self) -> f32 {
        self.plasticity_rate
    }

    /// Gibt die Position des Neurons zurück
    pub fn position(&self) -> &Position {
        &self.position
    }

    /// Setzt die Position des Neurons
    pub fn set_position(&mut self, new_position: Position) {
        self.position = new_position;
    }

    /// Erstellt ein neues Neuron mit der angegebenen Geschwindigkeit und Position
    ///
    /// # Arguments
    ///
    /// * `speed` - Die Geschwindigkeit des Neurons (1-1000)
    /// * `position` - Die 3D-Position des Neurons
    ///
    /// # Returns
    ///
    /// Ein neues Neuron mit der angegebenen Position und Standardwerten für andere Parameter
    pub fn with_position(speed: u16, position: Position) -> Self {
        let mut neuron = Self::new(speed);
        neuron.position = position;
        neuron
    }

    /// Erstellt ein neues Neuron mit benutzerdefinierten Parametern und Position
    ///
    /// # Arguments
    ///
    /// * `speed` - Die Geschwindigkeit des Neurons (1-1000)
    /// * `threshold` - Der Aktivierungsschwellwert
    /// * `plasticity_rate` - Die Plastizitätsrate für Anpassungen
    /// * `position` - Die 3D-Position des Neurons
    ///
    /// # Returns
    ///
    /// Ein neues Neuron mit den angegebenen Parametern und Position
    pub fn with_params_and_position(
        speed: u16,
        threshold: f32,
        plasticity_rate: f32,
        position: Position,
    ) -> Self {
        let mut neuron = Self::with_params(speed, threshold, plasticity_rate);
        neuron.position = position;
        neuron
    }

    /// Empfängt ein Eingabesignal und aktualisiert die Aktivierungsenergie
    ///
    /// # Arguments
    ///
    /// * `input` - Der Eingabewert, der zur Aktivierungsenergie addiert wird
    ///
    /// # Returns
    ///
    /// `true`, wenn das Neuron aktiviert wurde, andernfalls `false`
    pub fn receive_input(&mut self, input: f32) -> bool {
        // Ignoriere Eingaben, wenn das Neuron im refraktären Zustand ist
        if self.state == NeuronState::Refractory {
            return false;
        }

        self.activation_energy += input;

        // Prüfen, ob der Schwellwert überschritten wurde
        if self.state == NeuronState::Inactive && self.activation_energy >= self.threshold {
            self.state = NeuronState::Active;
            return true;
        }

        false
    }

    /// Führt einen Aktivierungszyklus des Neurons durch
    ///
    /// # Returns
    ///
    /// Der Ausgabewert des Neurons, wenn es aktiviert ist, sonst 0.0
    pub fn cycle(&mut self) -> f32 {
        match self.state {
            NeuronState::Inactive => 0.0,
            NeuronState::Active => {
                // Ausgabewert berechnen basierend auf Aktivierungsenergie
                let output = self.activation_energy;

                // Neuron in refraktären Zustand versetzen
                self.state = NeuronState::Refractory;

                // Aktivierungsenergie zurücksetzen
                self.activation_energy = 0.0;

                output
            }
            NeuronState::Refractory => {
                // Erholungsphase - zurück zum inaktiven Zustand
                self.state = NeuronState::Inactive;
                0.0
            }
        }
    }

    /// Passt den Schwellwert basierend auf der Häufigkeit der Aktivierung an
    ///
    /// # Arguments
    ///
    /// * `was_active` - Ob das Neuron im letzten Zyklus aktiv war
    /// * `target_activity` - Die gewünschte Aktivitätsrate (0.0-1.0)
    pub fn adapt_threshold(&mut self, was_active: bool, target_activity: f32) {
        let activity_error = if was_active { 1.0 } else { 0.0 } - target_activity;

        // Homeöstatisches Prinzip: Erhöhe Schwellwert bei zu hoher Aktivität,
        // verringere ihn bei zu niedriger Aktivität
        self.threshold += self.plasticity_rate * activity_error;

        // Stelle sicher, dass der Schwellwert nicht negativ wird
        if self.threshold < 0.0 {
            self.threshold = 0.0;
        }
    }

    /// Setzt die Parameter des Neurons zurück
    pub fn reset(&mut self) {
        self.activation_energy = 0.0;
        self.state = NeuronState::Inactive;
    }

    /// Startet das Axonwachstum für dieses Neuron
    ///
    /// # Arguments
    ///
    /// * `initial_energy` - Optionale Anfangsenergie für das Wachstum (Standard: basierend auf Kapazität)
    ///
    /// # Returns
    ///
    /// Eine neue AxonGrowth-Instanz für dieses Neuron
    pub fn start_axon_growth(&self, initial_energy: Option<f32>) -> AxonGrowth {
        let energy = initial_energy.unwrap_or_else(|| self.capacity() * 0.5);
        AxonGrowth::new(self.position, energy)
    }

    /// Berechnet den Einfluss dieses Neurons als Wachstumsfaktor
    ///
    /// # Arguments
    ///
    /// * `is_excitatory` - Ob das Neuron erregend (attraktiv) oder hemmend (repulsiv) ist
    ///
    /// # Returns
    ///
    /// Ein GrowthFactor, der den Einfluss dieses Neurons repräsentiert
    pub fn as_growth_factor(&self, is_excitatory: bool) -> GrowthFactor {
        let factor_type = if is_excitatory {
            crate::neural::growth::FactorType::Attractive
        } else {
            crate::neural::growth::FactorType::Repulsive
        };

        // Einflussradius und Stärke basierend auf Neuronen-Eigenschaften
        let radius = self.speed() as f32 * 0.2;
        let strength = self.activation_energy() / self.threshold();

        GrowthFactor::new(self.position, strength, radius, factor_type)
    }
}

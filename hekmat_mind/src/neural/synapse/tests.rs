#[cfg(test)]
mod synapse_tests {
    use crate::neural::synapse::model::{Synapse, SynapseBuilder};
    use uuid::Uuid;

    /// Testet die Erstellung einer Synapse zwischen zwei Neuronen
    #[test]
    fn test_synapse_creation() {
        let pre_neuron_id = Uuid::new_v4();
        let post_neuron_id = Uuid::new_v4();
        let synapse = Synapse::new(pre_neuron_id, post_neuron_id, 0.5);

        assert_eq!(*synapse.pre_neuron_id(), pre_neuron_id);
        assert_eq!(*synapse.post_neuron_id(), post_neuron_id);
        assert_eq!(synapse.weight(), 0.5);
        assert!(!synapse.active_state());
    }

    /// Testet die Signalübertragung durch eine Synapse
    #[test]
    fn test_signal_transmission() {
        let pre_id = Uuid::new_v4();
        let post_id = Uuid::new_v4();
        let mut synapse = Synapse::new(pre_id, post_id, 0.5);

        let input_signal = 0.8;
        let output = synapse.transmit(input_signal);

        assert_eq!(output, input_signal * 0.5);
        assert!(synapse.active_state());

        synapse.update(0.1);

        assert!(!synapse.active_state());
    }

    /// Testet die synaptische Plastizität (Hebbsches Lernen)
    #[test]
    fn test_hebbian_plasticity() {
        let pre_id = Uuid::new_v4();
        let post_id = Uuid::new_v4();
        let mut synapse = Synapse::new(pre_id, post_id, 0.5);

        let pre_active = true;
        let post_active = true;
        let plasticity_rate = 0.1;

        let old_weight = synapse.weight();
        synapse.apply_hebbian_plasticity(pre_active, post_active, plasticity_rate);

        assert!(synapse.weight() > old_weight);

        let mut synapse2 = Synapse::new(pre_id, post_id, 0.5);
        let old_weight2 = synapse2.weight();
        synapse2.apply_hebbian_plasticity(true, false, plasticity_rate);

        assert!(synapse2.weight() < old_weight2);
    }

    /// Testet die Gewichtsbegrenzung
    #[test]
    fn test_weight_bounds() {
        let pre_id = Uuid::new_v4();
        let post_id = Uuid::new_v4();

        let mut synapse_high = Synapse::new(pre_id, post_id, 0.9);
        for _ in 0..10 {
            synapse_high.apply_hebbian_plasticity(true, true, 0.2);
        }
        assert!(synapse_high.weight() <= 1.0);

        let mut synapse_low = Synapse::new(pre_id, post_id, 0.1);
        for _ in 0..10 {
            synapse_low.apply_hebbian_plasticity(true, false, 0.2);
        }
        assert!(synapse_low.weight() >= 0.0);
    }

    /// Testet die SynapseBuilder-Implementierung
    #[test]
    fn test_synapse_builder() {
        let pre_id = Uuid::new_v4();
        let post_id = Uuid::new_v4();

        let synapse = SynapseBuilder::new()
            .with_pre_neuron_id(pre_id)
            .with_post_neuron_id(post_id)
            .with_weight(0.7)
            .with_delay(0.002)
            .build();

        assert_eq!(*synapse.pre_neuron_id(), pre_id);
        assert_eq!(*synapse.post_neuron_id(), post_id);
        assert_eq!(synapse.weight(), 0.7);
        assert!(synapse.delay() > 0.0);
    }
}

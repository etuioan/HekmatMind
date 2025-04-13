//! Benchmarks für Synapsen-Komponenten
//!
//! Diese Benchmarks messen die Leistung der synaptischen Verbindungen unter verschiedenen Bedingungen.

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use hekmat_mind::SynapseBuilder;
use uuid::Uuid;

/// Benchmark für die synaptische Signalübertragung
fn bench_synapse_transmission(c: &mut Criterion) {
    let mut group = c.benchmark_group("Synapse Transmission");

    // Verschiedene Gewichte testen
    for weight in [0.1, 0.5, 0.9].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(weight), weight, |b, &weight| {
            let pre_id = Uuid::new_v4();
            let post_id = Uuid::new_v4();
            let mut synapse = SynapseBuilder::new()
                .with_pre_neuron_id(pre_id)
                .with_post_neuron_id(post_id)
                .with_weight(weight)
                .build();

            b.iter(|| {
                black_box(synapse.transmit(black_box(1.0)));
            });
        });
    }

    group.finish();
}

/// Benchmark für die Hebbsche Plastizität
fn bench_synapse_plasticity(c: &mut Criterion) {
    let mut group = c.benchmark_group("Synapse Plasticity");

    // Verschiedene Plastizitätsraten testen
    for &plasticity_rate in [0.001, 0.01, 0.1].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(plasticity_rate),
            &plasticity_rate,
            |b, &plasticity_rate| {
                let pre_id = Uuid::new_v4();
                let post_id = Uuid::new_v4();
                let mut synapse = SynapseBuilder::new()
                    .with_pre_neuron_id(pre_id)
                    .with_post_neuron_id(post_id)
                    .with_weight(0.5)
                    .build();

                b.iter(|| {
                    synapse.apply_hebbian_plasticity(
                        black_box(true),
                        black_box(true),
                        black_box(plasticity_rate),
                    );
                    black_box(());
                });
            },
        );
    }

    group.finish();
}

/// Benchmark für die Zustandsaktualisierung der Synapse
fn bench_synapse_update(c: &mut Criterion) {
    let mut group = c.benchmark_group("Synapse Update");

    // Verschiedene Zeitschritte testen
    for &time_step in [0.001, 0.005, 0.010].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(time_step),
            &time_step,
            |b, &time_step| {
                let pre_id = Uuid::new_v4();
                let post_id = Uuid::new_v4();
                let mut synapse = SynapseBuilder::new()
                    .with_pre_neuron_id(pre_id)
                    .with_post_neuron_id(post_id)
                    .with_weight(0.5)
                    .build();

                // Synapse vor dem Benchmark aktivieren
                synapse.transmit(1.0);

                b.iter(|| {
                    synapse.update(black_box(time_step));
                    black_box(());
                });
            },
        );
    }

    group.finish();
}

/// Benchmark für die Skalierbarkeit mit vielen Synapsen
fn bench_synapse_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("Synapse Scaling");

    // Verschiedene Anzahlen von Synapsen testen
    for &count in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            // Erstelle die Synapsen vor der Benchmark-Schleife
            let mut synapses = Vec::with_capacity(count as usize);

            for _ in 0..count {
                let pre_id = Uuid::new_v4();
                let post_id = Uuid::new_v4();
                let synapse = SynapseBuilder::new()
                    .with_pre_neuron_id(pre_id)
                    .with_post_neuron_id(post_id)
                    .with_weight(0.5)
                    .build();

                synapses.push(synapse);
            }

            b.iter(|| {
                // Nur die Signalübertragung messen
                let mut total_output = 0.0;
                for synapse in &mut synapses {
                    total_output += synapse.transmit(black_box(1.0));
                }
                black_box(total_output);
            });
        });
    }

    group.finish();
}

criterion_group!(
    synapse_benchmark,
    bench_synapse_transmission,
    bench_synapse_plasticity,
    bench_synapse_update,
    bench_synapse_scaling
);
criterion_main!(synapse_benchmark);

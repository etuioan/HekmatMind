use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use hekmat_mind::{Neuron, neuron_constants as constants};

// Das gemeinsame Benchmark-Modul importieren
#[path = "../common.rs"]
mod common;
use common::document_benchmark;

fn bench_neuron_activation(_c: &mut Criterion) {
    // Criterion mit Standard-Einstellungen
    let mut criterion = Criterion::default();
    let mut group = criterion.benchmark_group("Neuron_Activation");

    // Dokumentiere den Benchmark
    let _ = document_benchmark(
        "Neuron_Activation",
        "Neuron",
        "Dieser Benchmark misst, wie schnell ein Neuron auf Eingangssignale reagiert. \
        Er vergleicht Neuronen mit verschiedenen Geschwindigkeiten (100, 500, 1000) \
        und misst, wie lange die Verarbeitung von zwei Eingangssignalen und ein \
        Durchlauf des neuronalen Zyklus dauert.",
        "Die Werte zeigen die Zeit in Nanosekunden, die ein Neuron für einen \
        kompletten Verarbeitungszyklus benötigt. Niedrigere Werte bedeuten \
        schnellere Verarbeitung. Die Parameter (100, 500, 1000) repräsentieren \
        die Neuronen-Geschwindigkeit, wobei höhere Werte schnellere Reaktionszeiten \
        ermöglichen sollten.",
    );

    for speed in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(speed), speed, |b, &speed| {
            let mut neuron = Neuron::new(speed);
            let threshold = neuron.threshold();

            b.iter(|| {
                neuron.reset();
                neuron.receive_input(black_box(threshold * 0.6));
                neuron.receive_input(black_box(threshold * 0.5));
                black_box(neuron.cycle()); // Stell sicher, dass das Ergebnis verwendet wird
            });
        });
    }

    group.finish();
}

fn bench_neuron_plasticity(_c: &mut Criterion) {
    // Criterion mit Standard-Einstellungen
    let mut criterion = Criterion::default();
    let mut group = criterion.benchmark_group("Neuron_Plasticity");

    // Dokumentiere den Benchmark
    let _ = document_benchmark(
        "Neuron_Plasticity",
        "Neuron",
        "Dieser Benchmark misst die Anpassungsfähigkeit (Plastizität) eines Neurons. \
        Er testet, wie schnell ein Neuron seine Schwellenwerte anpassen kann, \
        wobei verschiedene Plastizitätsraten (0.001, 0.01, 0.1) verwendet werden. \
        Eine höhere Plastizitätsrate bedeutet schnellere Anpassung an neue Bedingungen.",
        "Die Werte zeigen die Zeit in Nanosekunden, die für 100 aufeinanderfolgende \
        Schwellenwertanpassungen benötigt wird. Die Parameter (0.001, 0.01, 0.1) \
        sind die Plastizitätsraten - höhere Werte sollten zu schnelleren Anpassungen \
        führen, könnten aber instabiler sein.",
    );

    for &plasticity_rate in [0.001, 0.01, 0.1].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(plasticity_rate),
            &plasticity_rate,
            |b, &plasticity_rate| {
                let mut neuron = Neuron::with_params(500, 0.5, plasticity_rate);

                b.iter(|| {
                    for _ in 0..100 {
                        neuron.adapt_threshold(true, black_box(0.2));
                        black_box(());
                    }
                });
            },
        );
    }

    group.finish();
}

fn bench_neuron_speed_capacity(_c: &mut Criterion) {
    // Criterion mit Standard-Einstellungen
    let mut criterion = Criterion::default();
    let mut group = criterion.benchmark_group("Neuron_Speed_Capacity");

    // Dokumentiere den Benchmark
    let _ = document_benchmark(
        "Neuron_Speed_Capacity",
        "Neuron",
        "Dieser Benchmark misst die Effizienz der Kapazitätsberechnung eines Neurons. \
        Er berechnet die Kapazität (Informationsverarbeitungsfähigkeit) für Neuronen mit \
        unterschiedlichen Geschwindigkeiten und summiert die Ergebnisse. \
        Dies testet, wie schnell das System die Kapazität vieler Neuronen berechnen kann.",
        "Die Werte zeigen die Gesamtzeit in Nanosekunden, die benötigt wird, um die \
        Kapazität aller Neuronen im Geschwindigkeitsbereich zu berechnen und zu summieren. \
        Ein niedrigerer Wert bedeutet, dass das System neuronale Eigenschaften \
        effizienter berechnen kann.",
    );

    // Erstelle einen Vektor mit allen möglichen Geschwindigkeiten
    let speeds: Vec<u16> = (constants::MIN_SPEED..=constants::MAX_SPEED)
        .step_by(100)
        .collect();

    group.bench_function("capacity_calculation", |b| {
        b.iter(|| {
            speeds
                .iter()
                .map(|&speed| {
                    let neuron = Neuron::new(speed);
                    neuron.capacity()
                })
                .sum::<f32>()
        });
    });

    group.finish();
}

// Definiere die Benchmark-Gruppe und führe sie aus
criterion_group!(
    neuron_benchmark,
    bench_neuron_activation,
    bench_neuron_plasticity,
    bench_neuron_speed_capacity
);

// Führe den Benchmark aus
criterion_main!(neuron_benchmark);

// Nach der Ausführung könnte man so die Ergebnisse zusammenfassen:
// Uncomment if you want to manually capture and print results
/*
fn print_results() {
    let results = vec![
        ("Neuron_Activation 100".to_string(), 150.0),
        ("Neuron_Activation 500".to_string(), 120.0),
        ("Neuron_Activation 1000".to_string(), 100.0),
        ("Neuron_Plasticity 0.001".to_string(), 200.0),
        ("Neuron_Plasticity 0.01".to_string(), 190.0),
        ("Neuron_Plasticity 0.1".to_string(), 180.0),
        ("Neuron_Speed_Capacity".to_string(), 300.0),
    ];
    print_benchmark_summary("Neuron Benchmark", &results);
}
*/

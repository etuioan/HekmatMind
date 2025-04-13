//! Benchmarks für den EventBroker
//!
//! Diese Benchmarks messen die Leistung des EventBrokers unter verschiedenen Bedingungen.

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use hekmat_mind::EventBroker;
use std::sync::Arc;

// Das gemeinsame Benchmark-Modul importieren
#[path = "common.rs"]
mod common;
use common::document_benchmark;

// Testeereignisse für Benchmarks
#[derive(Debug, Clone)]
struct TestEvent {
    #[allow(dead_code)]
    data: Vec<u8>,
}

impl TestEvent {
    // Erstellt ein Testereignis mit einer bestimmten Größe (in Bytes)
    fn with_size(size: usize) -> Self {
        TestEvent {
            data: vec![42; size],
        }
    }
}

// Benchmark für das Veröffentlichen von Ereignissen
fn bench_publish(_c: &mut Criterion) {
    // Verschiedene Ereignisgrößen testen
    let sizes = vec![8, 64, 512, 1024, 4096];

    // Criterion mit Standard-Einstellungen
    let mut criterion = Criterion::default();
    let mut group = criterion.benchmark_group("EventBroker_Publish");

    // Dokumentiere den Benchmark
    let _ = document_benchmark(
        "EventBroker_Publish",
        "EventBroker",
        "Dieser Benchmark misst die Effizienz des EventBrokers beim Veröffentlichen von Ereignissen \
        verschiedener Größen. Er testet, wie schnell der EventBroker Ereignisse mit Dateigrößen \
        von 8 bis 4096 Bytes verarbeiten kann.",
        "Die Werte zeigen die Zeit in Nanosekunden, die zum Veröffentlichen eines Ereignisses benötigt wird. \
        Kleinere Werte bedeuten schnellere Verarbeitung. Die Parameter (8, 64, 512, 1024, 4096) \
        stellen die Größe des Ereignisses in Bytes dar – größere Ereignisse erfordern \
        in der Regel mehr Verarbeitungszeit.",
    );

    for size in sizes {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let broker = EventBroker::new();
            let event = TestEvent::with_size(size);

            // Ein Subscriber, der nichts tut
            broker.subscribe(|_: Arc<TestEvent>| {});

            b.iter(|| {
                broker.publish(black_box(event.clone()));
            });
        });
    }

    group.finish();
}

// Benchmark für die Subscriber-Anzahl
fn bench_subscriber_count(c: &mut Criterion) {
    // Verschiedene Anzahlen von Subscribern testen
    let subscriber_counts = vec![1, 10, 100, 1000];

    let mut group = c.benchmark_group("EventBroker_SubscriberCount");

    for &count in &subscriber_counts {
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            let broker = EventBroker::new();
            let event = TestEvent::with_size(8);

            // Dummy-Subscriber registrieren, der nichts tut
            for _ in 0..count {
                broker.subscribe(|_: Arc<TestEvent>| {
                    // Leere Funktion für bessere Benchmark-Isolation
                    black_box(());
                });
            }

            b.iter(|| {
                // Nur die Publish-Operation messen
                broker.publish(black_box(event.clone()));
            });
        });
    }

    group.finish();
}

// Benchmark für verschiedene Ereignistypen
fn bench_event_types(c: &mut Criterion) {
    #[derive(Debug, Clone)]
    struct EventType1 {
        #[allow(dead_code)]
        data: u64,
    }

    #[derive(Debug, Clone)]
    struct EventType2 {
        #[allow(dead_code)]
        data: u64,
    }

    #[derive(Debug, Clone)]
    struct EventType3 {
        #[allow(dead_code)]
        data: u64,
    }

    let mut group = c.benchmark_group("EventBroker_EventTypes");

    // Einzelner Ereignistyp
    group.bench_function("Single_EventType", |b| {
        let broker = EventBroker::new();
        let event1 = EventType1 { data: 42 };

        broker.subscribe(|_: Arc<EventType1>| {});

        b.iter(|| {
            broker.publish(black_box(event1.clone()));
        });
    });

    // Mehrere Ereignistypen
    group.bench_function("Multiple_EventTypes", |b| {
        let broker = EventBroker::new();
        let event1 = EventType1 { data: 42 };
        let event2 = EventType2 { data: 21 };
        let event3 = EventType3 { data: 84 };

        broker.subscribe(|_: Arc<EventType1>| {});
        broker.subscribe(|_: Arc<EventType2>| {});
        broker.subscribe(|_: Arc<EventType3>| {});

        b.iter(|| {
            broker.publish(black_box(event1.clone()));
            broker.publish(black_box(event2.clone()));
            broker.publish(black_box(event3.clone()));
        });
    });

    group.finish();
}

criterion_group!(
    event_broker_benchmark,
    bench_publish,
    bench_subscriber_count,
    bench_event_types
);

criterion_main!(event_broker_benchmark);

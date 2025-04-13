// Tests für die TelemetryCollector Traits
//
// Diese Tests überprüfen die Funktionalität der Telemetrie-Collector-Traits
// und stellen sicher, dass sie sich wie erwartet verhalten.

use hekmat_mind::telemetry::collector::{
    ExportFormat, MetricStats, QueryableCollector, TelemetryCollector,
};
use hekmat_mind::telemetry::in_memory::InMemoryCollector;
use std::collections::HashMap;
use std::time::Duration;

/// Test für die MetricStats Struktur
#[test]
fn test_metric_stats_creation() {
    // Erstelle MetricStats mit Testwerten
    let stats = MetricStats {
        min: 1.0,
        max: 10.0,
        avg: 5.5,
        median: 5.0,
        p95: 9.5,
        p99: 9.9,
        count: 100,
    };

    // Überprüfe, ob die Werte korrekt gesetzt wurden
    assert_eq!(stats.min, 1.0);
    assert_eq!(stats.max, 10.0);
    assert_eq!(stats.avg, 5.5);
    assert_eq!(stats.median, 5.0);
    assert_eq!(stats.p95, 9.5);
    assert_eq!(stats.p99, 9.9);
    assert_eq!(stats.count, 100);

    // Teste den Debug-Trait für MetricStats
    let debug_output = format!("{:?}", stats);
    assert!(debug_output.contains("min: 1.0"));
    assert!(debug_output.contains("max: 10.0"));
    assert!(debug_output.contains("count: 100"));
}

/// Test für das ExportFormat Enum
#[test]
fn test_export_format() {
    // Teste die verschiedenen Formate
    let json_format = ExportFormat::Json;
    let csv_format = ExportFormat::Csv;
    let prometheus_format = ExportFormat::Prometheus;

    // Überprüfe Debug-Ausgabe
    assert_eq!(format!("{:?}", json_format), "Json");
    assert_eq!(format!("{:?}", csv_format), "Csv");
    assert_eq!(format!("{:?}", prometheus_format), "Prometheus");

    // Teste die Clone-Implementierung
    let cloned_json = json_format;
    assert_eq!(format!("{:?}", cloned_json), "Json");

    // Teste die Copy-Implementierung
    let copied_csv = csv_format;
    assert_eq!(format!("{:?}", copied_csv), "Csv");
}

/// Test für TelemetryCollector und QueryableCollector Traits mit InMemoryCollector
#[test]
fn test_telemetry_collector_implementation() {
    // Erstelle einen InMemoryCollector, der beide Traits implementiert
    let mut collector = InMemoryCollector::new(100);

    // Initialisiere den Collector (optional, wird normalerweise automatisch gemacht)
    collector.initialize();

    // Teste record_counter
    let counter_labels = Some(HashMap::from([(
        "test_key".to_string(),
        "test_value".to_string(),
    )]));
    collector.record_counter("test_component", "test_counter", 42, counter_labels);

    // Teste record_gauge
    collector.record_gauge("test_component", "test_gauge", 42.5, None);

    // Teste record_histogram
    collector.record_histogram("test_component", "test_histogram", 100.0, None);

    // Teste record_event
    collector.record_event(
        "test_component",
        "test_event",
        Duration::from_millis(50),
        None,
    );

    // Teste QueryableCollector Funktionalität
    let metrics = collector.query_metrics("test_component");
    assert!(!metrics.is_empty(), "Metrics sollten nicht leer sein");

    // Überprüfe, ob wir die aufgezeichneten Metriken finden können
    assert!(
        metrics.contains_key("test_counter"),
        "Counter-Metrik sollte aufgezeichnet sein"
    );
    assert!(
        metrics.contains_key("test_gauge"),
        "Gauge-Metrik sollte aufgezeichnet sein"
    );
    assert!(
        metrics.contains_key("test_histogram"),
        "Histogram-Metrik sollte aufgezeichnet sein"
    );
    assert!(
        metrics.contains_key("test_event"),
        "Event-Metrik sollte aufgezeichnet sein"
    );

    // Teste query_stats
    let stats = collector.query_stats("test_component", "test_gauge");
    assert!(stats.is_some(), "Sollte Statistiken für test_gauge finden");

    let stats_unwrapped = stats.unwrap();
    assert_eq!(stats_unwrapped.min, 42.5);
    assert_eq!(stats_unwrapped.max, 42.5);
    assert_eq!(stats_unwrapped.count, 1);

    // Teste as_any (wichtig für den Downcast in realen Anwendungen)
    let any_ref = collector.as_any();
    let downcast_result = any_ref.downcast_ref::<InMemoryCollector>();
    assert!(
        downcast_result.is_some(),
        "Downcast sollte erfolgreich sein"
    );

    // Teste Freigabe von Ressourcen
    collector.shutdown();
}

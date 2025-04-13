use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use hekmat_mind::prelude::TelemetryCollector;
use hekmat_mind::telemetry::TelemetryRegistry;
use hekmat_mind::telemetry::collector::QueryableCollector;
use hekmat_mind::telemetry::in_memory::InMemoryCollector;

/// Testet die Grenzfälle des InMemoryCollector, insbesondere das Verhalten
/// bei Erreichen der maximalen Datenpunktanzahl
#[test]
fn test_memory_collector_max_points_capacity() {
    // Collector mit kleiner Kapazität erstellen
    let collector = InMemoryCollector::new(3);
    let component = "test_component";
    let metric = "capacity_test";

    // Mehr Punkte hinzufügen als die Kapazität erlaubt
    for i in 0..5 {
        collector.record_gauge(component, metric, i as f64, None);
    }

    // Überprüfen, dass nur die letzten 3 Punkte gespeichert wurden
    let metrics = collector.query_metrics(component);
    assert!(metrics.contains_key(metric));

    let points = metrics.get(metric).unwrap();
    assert_eq!(points.len(), 3);

    // Überprüfen, dass die ältesten Punkte entfernt wurden
    assert_eq!(points[0].value, 2.0);
    assert_eq!(points[1].value, 3.0);
    assert_eq!(points[2].value, 4.0);
}

/// Testet die Statistikberechnung mit verschiedenen Datensätzen
#[test]
fn test_metric_stats_calculation() {
    let collector = InMemoryCollector::new(1000);
    let component = "stats_component";
    let metric = "stats_metric";

    // Leeren Datensatz testen
    let empty_stats = collector.query_stats(component, metric);
    assert!(empty_stats.is_none());

    // Einzelnen Datenpunkt testen
    collector.record_gauge(component, metric, 10.0, None);

    let single_stats = collector.query_stats(component, metric).unwrap();
    assert_eq!(single_stats.min, 10.0);
    assert_eq!(single_stats.max, 10.0);
    assert_eq!(single_stats.avg, 10.0);
    assert_eq!(single_stats.median, 10.0);
    assert_eq!(single_stats.p95, 10.0);
    assert_eq!(single_stats.p99, 10.0);
    assert_eq!(single_stats.count, 1);

    // Mehrere Datenpunkte in ungeordneter Reihenfolge testen
    let component2 = "multi_stats";
    let metric2 = "random_values";

    // Unsortierte Werte einfügen
    let values = vec![5.0, 2.0, 9.0, 1.0, 8.0, 3.0, 7.0, 4.0, 6.0, 10.0];
    for value in values {
        collector.record_gauge(component2, metric2, value, None);
    }

    let stats = collector.query_stats(component2, metric2).unwrap();

    // Überprüfen der statistischen Berechnungen
    assert_eq!(stats.min, 1.0);
    assert_eq!(stats.max, 10.0);
    assert_eq!(stats.avg, 5.5); // (1+2+3+4+5+6+7+8+9+10)/10 = 5.5
    assert_eq!(stats.median, 6.0); // Median bei 10 Elementen ist bei index=count/2=5 (nach Sortierung), also 6.0
    assert_eq!(stats.p95, 10.0); // 95% von 10 Elementen = Index 9, Wert = 10.0
    assert_eq!(stats.p99, 10.0); // 99% von 10 Elementen = Index 9, Wert = 10.0
    assert_eq!(stats.count, 10);
}

/// Testet die TelemetryRegistry-Funktionen mit verschiedenen Metriken
/// (umgestellt auf lokalen Registry-Ansatz für bessere Testdeterminismus)
#[test]
fn test_registry_global_functions_extensive() {
    use hekmat_mind::telemetry::TelemetryRegistry;

    // Erstellen einer lokalen Registry-Instanz
    let mut registry = TelemetryRegistry::new();

    // Erstellen eines neuen Collectors
    let collector = InMemoryCollector::new(100);

    // Eindeutigen Komponenten-Namen für diesen Test verwenden
    let component = "extensive_test_isolated_component";

    // Referenz auf Collector behalten für spätere Überprüfungen
    let collector_ref = Arc::new(collector.clone());

    // Collector zur lokalen Registry hinzufügen
    registry.register(Box::new(collector));

    // Sicherstellen, dass die Registry einen Collector hat
    assert_eq!(
        registry.collectors().len(),
        1,
        "Die Registry sollte genau einen Collector haben"
    );

    // Counter-Metrik
    registry.record_counter(component, "test_counter", 42, None);

    // Counter mit Labels
    let mut counter_labels = HashMap::new();
    counter_labels.insert("type".to_string(), "test".to_string());
    registry.record_counter(component, "labeled_counter", 100, Some(counter_labels));

    // Gauge-Metrik
    registry.record_gauge(component, "test_gauge", std::f64::consts::PI, None);

    // Histogram-Metrik
    registry.record_histogram(component, "test_histogram", 100.0, None);

    // Event-Metrik
    registry.record_event(component, "test_event", Duration::from_millis(150), None);

    // Überprüfen der Metriken
    let metrics = collector_ref.query_metrics(component);

    // Debug-Ausgabe
    println!(
        "Gefundene Metriken für {}: {:?}",
        component,
        metrics.keys().collect::<Vec<_>>()
    );

    // Überprüfen der aufgezeichneten Metriken
    assert!(
        metrics.contains_key("test_counter"),
        "Counter 'test_counter' nicht gefunden"
    );
    assert!(
        metrics.contains_key("labeled_counter"),
        "Counter 'labeled_counter' nicht gefunden"
    );
    assert!(
        metrics.contains_key("test_gauge"),
        "Gauge 'test_gauge' nicht gefunden"
    );
    assert!(
        metrics.contains_key("test_histogram"),
        "Histogram 'test_histogram' nicht gefunden"
    );
    assert!(
        metrics.contains_key("test_event"),
        "Event 'test_event' nicht gefunden"
    );
}

/// Testet die TelemetryRegistry-Funktionen mit erweiterten Metriken
/// (umgestellt auf lokalen Registry-Ansatz für bessere Testdeterminismus)
#[test]
fn test_registry_global_functions_extensive_2() {
    use hekmat_mind::telemetry::TelemetryRegistry;

    // Erstellen einer lokalen Registry-Instanz
    let mut registry = TelemetryRegistry::new();

    // Erstellen eines neuen Collectors
    let collector = InMemoryCollector::new(100);

    // Eindeutigen Komponenten-Namen für diesen Test verwenden
    let component = "extensive_2_isolated_component";

    // Referenz auf Collector behalten für spätere Überprüfungen
    let collector_ref = Arc::new(collector.clone());

    // Collector zur lokalen Registry hinzufügen
    registry.register(Box::new(collector));

    // Sicherstellen, dass die Registry einen Collector hat
    assert_eq!(
        registry.collectors().len(),
        1,
        "Die Registry sollte genau einen Collector haben"
    );

    // Counter-Metrik
    registry.record_counter(component, "global_counter", 42, None);

    // Gauge-Metrik mit Labels
    let mut gauge_labels = HashMap::new();
    gauge_labels.insert("unit".to_string(), "celsius".to_string());
    registry.record_gauge(component, "labeled_gauge", 36.5, Some(gauge_labels));

    // Histogram-Metrik
    registry.record_histogram(component, "response_time", 250.0, None);

    // Event mit Dauer und Labels
    let mut event_labels = HashMap::new();
    event_labels.insert("event_type".to_string(), "user_login".to_string());
    event_labels.insert("status".to_string(), "success".to_string());
    registry.record_event(
        component,
        "labeled_event",
        Duration::from_secs(1),
        Some(event_labels),
    );

    // Überprüfen, dass die Metriken über den lokalen Collector erfasst wurden
    let metrics = collector_ref.query_metrics(component);

    // Debug-Ausgabe der Metriken
    println!(
        "Gefundene Metriken für {}: {:?}",
        component,
        metrics.keys().collect::<Vec<_>>()
    );

    // Counter prüfen
    assert!(
        metrics.contains_key("global_counter"),
        "Counter 'global_counter' wurde nicht gefunden"
    );
    let counter_points = &metrics["global_counter"];
    assert!(
        !counter_points.is_empty(),
        "Keine Datenpunkte für 'global_counter' gefunden"
    );
    assert_eq!(counter_points[0].value, 42.0);

    // Gauge mit Labels prüfen
    assert!(
        metrics.contains_key("labeled_gauge"),
        "Gauge 'labeled_gauge' wurde nicht gefunden"
    );
    let gauge_points = &metrics["labeled_gauge"];
    assert!(
        !gauge_points.is_empty(),
        "Keine Datenpunkte für 'labeled_gauge' gefunden"
    );
    assert_eq!(gauge_points[0].value, 36.5);
    assert_eq!(gauge_points[0].labels.get("unit").unwrap(), "celsius");

    // Histogram prüfen
    assert!(
        metrics.contains_key("response_time"),
        "Histogram 'response_time' wurde nicht gefunden"
    );

    // Event prüfen
    assert!(
        metrics.contains_key("labeled_event"),
        "Event 'labeled_event' wurde nicht gefunden"
    );

    // Statistiken abrufen und überprüfen
    if let Some(stats) = collector_ref.query_stats(component, "global_counter") {
        assert_eq!(stats.count, 1);
    } else {
        panic!("Keine Statistik für global_counter gefunden!");
    }
}

/// Testet die Registry-Funktionen mit lokaler Registry-Instanz für bessere Isolierung
#[test]
fn test_registry_global_functions_safe() {
    use hekmat_mind::telemetry::TelemetryRegistry;

    // Erstellen einer lokalen Registry-Instanz
    let mut registry = TelemetryRegistry::new();

    // Erstellen eines neuen Collectors
    let collector = InMemoryCollector::new(100);

    // Eindeutigen Komponenten-Namen für diesen Test verwenden
    let component = "safe_test_isolated_component";

    // Referenz auf Collector behalten für spätere Überprüfungen
    let collector_ref = Arc::new(collector.clone());

    // Collector zur lokalen Registry hinzufügen
    registry.register(Box::new(collector));

    // Vergewissern, dass wir einen Collector haben
    assert_eq!(
        registry.collectors().len(),
        1,
        "Die Registry sollte genau einen Collector haben"
    );

    // Verschiedene Metrik-Typen aufzeichnen
    registry.record_counter(component, "test_counter_1", 10, None);
    registry.record_counter(component, "test_counter_2", 20, None);
    registry.record_gauge(component, "test_gauge", 30.5, None);
    registry.record_histogram(component, "test_histogram", 40.0, None);
    registry.record_event(component, "test_event", Duration::from_millis(50), None);

    // Mehrere Werte für den gleichen Counter aufzeichnen
    registry.record_counter(component, "multi_counter", 1, None);
    registry.record_counter(component, "multi_counter", 2, None);
    registry.record_counter(component, "multi_counter", 3, None);

    // Metrik mit Labels
    let mut labels = HashMap::new();
    labels.insert("priority".to_string(), "high".to_string());
    registry.record_counter(component, "labeled_counter", 100, Some(labels));

    // Metriken überprüfen
    let metrics = collector_ref.query_metrics(component);

    // Debug-Ausgabe
    println!(
        "Gefundene Metriken für {}: {:?}",
        component,
        metrics.keys().collect::<Vec<_>>()
    );

    // Überprüfen, dass alle Metrik-Typen korrekt aufgezeichnet wurden
    assert!(
        metrics.contains_key("test_counter_1"),
        "Counter 'test_counter_1' nicht gefunden"
    );
    assert!(
        metrics.contains_key("test_counter_2"),
        "Counter 'test_counter_2' nicht gefunden"
    );
    assert!(
        metrics.contains_key("test_gauge"),
        "Gauge 'test_gauge' nicht gefunden"
    );
    assert!(
        metrics.contains_key("test_histogram"),
        "Histogram 'test_histogram' nicht gefunden"
    );
    assert!(
        metrics.contains_key("test_event"),
        "Event 'test_event' nicht gefunden"
    );
    assert!(
        metrics.contains_key("multi_counter"),
        "Counter 'multi_counter' nicht gefunden"
    );
    assert!(
        metrics.contains_key("labeled_counter"),
        "Counter 'labeled_counter' nicht gefunden"
    );

    // Statistiken für multi_counter überprüfen
    if let Some(stats) = collector_ref.query_stats(component, "multi_counter") {
        assert_eq!(stats.count, 3, "Multi-Counter sollte 3 Werte haben");
        assert_eq!(
            stats.avg, 2.0,
            "Durchschnitt der Multi-Counter-Werte sollte 2.0 sein"
        );
        assert_eq!(
            stats.min, 1.0,
            "Minimalwert der Multi-Counter-Werte sollte 1.0 sein"
        );
        assert_eq!(
            stats.max, 3.0,
            "Maximalwert der Multi-Counter-Werte sollte 3.0 sein"
        );
    } else {
        panic!("Keine Statistik für multi_counter gefunden!");
    }

    // Werte für gelabelten Counter prüfen
    let labeled_points = &metrics["labeled_counter"];
    assert!(
        !labeled_points.is_empty(),
        "Keine Datenpunkte für 'labeled_counter' gefunden"
    );
    assert_eq!(labeled_points[0].value, 100.0);
    assert_eq!(labeled_points[0].labels.get("priority").unwrap(), "high");
}

/// Testet die leeren Metriken in verschiedenen Situationen
#[test]
fn test_empty_metrics_edge_cases() {
    let collector = InMemoryCollector::new(1000);

    // Abfrage für nicht existierende Komponenten und Metriken
    let metrics = collector.query_metrics("nonexistent");
    assert_eq!(metrics.len(), 0);

    let stats = collector.query_stats("nonexistent", "metric");
    assert!(stats.is_none());

    // Leere Komponente erstellen, aber keine Metriken hinzufügen
    collector.record_counter("empty_component", "empty_metric", 0, None);

    // Neue Registry mit diesem Collector erstellen
    let mut registry = TelemetryRegistry::new();
    registry.register(Box::new(collector));

    // Leere Registry testen
    let empty_registry = TelemetryRegistry::new();

    // Statistiken von beiden Registry-Instanzen abfragen
    let registry_stats = registry.collectors().first().and_then(|c| {
        if let Some(queryable) = c.as_any().downcast_ref::<InMemoryCollector>() {
            queryable.query_stats("empty_component", "empty_metric")
        } else {
            None
        }
    });
    assert!(registry_stats.is_some());

    // Überprüfen, dass die leere Registry keine Statistiken hat
    assert_eq!(empty_registry.collectors().len(), 0);
}

/// Test für die Kapazitätsbegrenzung des InMemoryCollector
#[test]
fn test_in_memory_collector_capacity() {
    // Testen, dass die new-Methode mit Kapazitätslimit korrekt funktioniert
    let collector = InMemoryCollector::new(5);

    // Kapazität ausnutzen
    for i in 0..5 {
        collector.record_counter("capacity_test", "counter", i as u64, None);
    }

    // Überprüfen, dass alle 5 Punkte gespeichert wurden
    let metrics = collector.query_metrics("capacity_test");
    let points = metrics.get("counter").unwrap();
    assert_eq!(points.len(), 5);

    // Einen weiteren Punkt hinzufügen, der den ältesten verdrängen sollte
    collector.record_counter("capacity_test", "counter", 100, None);

    // Überprüfen, dass weiterhin 5 Punkte vorhanden sind und der älteste entfernt wurde
    let metrics = collector.query_metrics("capacity_test");
    let points = metrics.get("counter").unwrap();
    assert_eq!(points.len(), 5);
    assert_eq!(points[0].value, 1.0); // Der Punkt mit Wert 0 wurde entfernt
}

/// Testet erweiterte Edge Cases des InMemoryCollector insbesondere bei ungültigen Werten
/// und verschiedenen Datentypen
#[test]
fn test_in_memory_collector_extended_edge_cases() {
    let collector = InMemoryCollector::new(10);
    let component = "edge_test";

    // Extremwerte für verschiedene Metriktypen testen
    // Sehr große Werte
    collector.record_counter(component, "big_counter", u64::MAX, None);
    collector.record_gauge(component, "big_gauge", f64::MAX, None);

    // Sehr kleine Werte
    collector.record_gauge(component, "tiny_gauge", f64::MIN_POSITIVE, None);

    // Negative Werte für Gauge
    collector.record_gauge(component, "negative_gauge", -100.0, None);

    // NaN und Infinity testen
    collector.record_gauge(component, "nan_gauge", f64::NAN, None);
    collector.record_gauge(component, "inf_gauge", f64::INFINITY, None);

    // Events mit verschiedenen Zeitdauern
    collector.record_event(component, "quick_event", Duration::from_nanos(1), None);
    collector.record_event(component, "long_event", Duration::from_secs(3600), None);

    // Überprüfen, dass alle Metriken gespeichert wurden
    let metrics = collector.query_metrics(component);
    assert!(metrics.contains_key("big_counter"));
    assert!(metrics.contains_key("big_gauge"));
    assert!(metrics.contains_key("tiny_gauge"));
    assert!(metrics.contains_key("negative_gauge"));
    assert!(metrics.contains_key("nan_gauge"));
    assert!(metrics.contains_key("inf_gauge"));
    assert!(metrics.contains_key("quick_event"));
    assert!(metrics.contains_key("long_event"));

    // Statistiken für spezielle Werte überprüfen
    if let Some(stats) = collector.query_stats(component, "negative_gauge") {
        assert!(stats.min < 0.0);
        assert_eq!(stats.count, 1);
    } else {
        panic!("Keine Statistik für negative_gauge gefunden!");
    }

    // Überprüfen der Behandlung von NaN-Werten
    // Da NaN sich nicht mit sich selbst vergleichen lässt, können spezielle Behandlungen nötig sein
    let nan_metrics = metrics.get("nan_gauge").unwrap();
    assert_eq!(nan_metrics.len(), 1);

    // Überprüfen der Behandlung von Infinity-Werten
    let inf_metrics = metrics.get("inf_gauge").unwrap();
    assert_eq!(inf_metrics.len(), 1);
    assert!(inf_metrics[0].value.is_infinite());
}

/// Testet die Statistikberechnungsfunktionen des InMemoryCollector im Detail
#[test]
fn test_in_memory_collector_statistics_calculation() {
    let collector = InMemoryCollector::new(20);
    let component = "stats_test";
    let metric_name = "test_percentiles";

    // Eine Reihe von Werten als Histogramm aufzeichnen
    // Verwenden einer nicht sortierten Reihenfolge, um die Sortierung zu testen
    let values = [
        10.0, 30.0, 20.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0, 5.0,
    ];

    for &value in &values {
        collector.record_histogram(component, metric_name, value, None);
    }

    // Statistiken abrufen und überprüfen
    let stats = collector.query_stats(component, metric_name).unwrap();

    // Grundlegende Statistiken prüfen
    assert_eq!(stats.count, values.len());
    assert_eq!(stats.min, 5.0); // Der kleinste Wert
    assert_eq!(stats.max, 100.0); // Der größte Wert

    // Das berechnete Mittel (Average) überprüfen
    let expected_avg = values.iter().sum::<f64>() / values.len() as f64;
    assert!((stats.avg - expected_avg).abs() < 0.001);

    // Median prüfen (sollte bei 50.0 liegen für diese Daten)
    assert!((stats.median - 50.0).abs() < 0.001);

    // Die Perzentile überprüfen
    // Bei 11 Werten (Index 0-10) ist p95 der Wert an Index 10 (11*0.95=10.45 -> 10), also 100.0
    // p99 ist auch der Wert an Index 10 (11*0.99=10.89 -> 10), also 100.0
    assert_eq!(stats.p95, 100.0);
    assert_eq!(stats.p99, 100.0);

    // Test mit nur einem einzigen Wert
    let single_component = "single_value";
    let single_metric = "single_test";
    collector.record_histogram(single_component, single_metric, 42.0, None);

    let single_stats = collector
        .query_stats(single_component, single_metric)
        .unwrap();
    assert_eq!(single_stats.count, 1);
    assert_eq!(single_stats.min, 42.0);
    assert_eq!(single_stats.max, 42.0);
    assert_eq!(single_stats.avg, 42.0);
    assert_eq!(single_stats.median, 42.0);
    assert_eq!(single_stats.p95, 42.0);
    assert_eq!(single_stats.p99, 42.0);

    // Test mit leerer Metrik (sollte None zurückgeben)
    let empty_component = "empty";
    let empty_metric = "no_data";
    assert!(
        collector
            .query_stats(empty_component, empty_metric)
            .is_none()
    );
}

/// Testet die TelemetryRegistry-Funktionen mit einer lokalen Instanz (ohne globale Funktionen)
#[test]
fn test_registry_local_instance() {
    use hekmat_mind::telemetry::TelemetryRegistry;
    use std::time::Duration;

    // Erstellen einer lokalen Registry-Instanz
    let mut registry = TelemetryRegistry::new();

    // Erstellen eines neuen Collectors
    let collector = InMemoryCollector::new(100);

    // Referenz auf Collector behalten für spätere Überprüfungen
    let collector_ref = Arc::new(collector.clone());

    // Collector zur Registry hinzufügen
    registry.register(Box::new(collector));

    // Sicherstellen, dass die Registry einen Collector hat
    assert_eq!(registry.collectors().len(), 1);

    // Komponente definieren, für die Metriken erfasst werden
    let component = "local_test_component";

    // Ein paar Metriken erfassen

    // Counter-Metrik
    registry.record_counter(component, "local_counter", 42, None);

    // Gauge-Metrik
    registry.record_gauge(component, "local_gauge", std::f64::consts::PI, None);

    // Histogram-Metrik
    registry.record_histogram(component, "local_histogram", std::f64::consts::E, None);

    // Event mit Dauer und Labels
    let mut event_labels = HashMap::new();
    event_labels.insert("local_event_type".to_string(), "test".to_string());
    registry.record_event(
        component,
        "local_event",
        Duration::from_millis(500),
        Some(event_labels),
    );

    // Einen weiteren Collector hinzufügen
    let temp_collector = InMemoryCollector::new(10);
    registry.register(Box::new(temp_collector));

    // Prüfen, dass jetzt zwei Collectors vorhanden sind
    assert_eq!(registry.collectors().len(), 2);

    // Einen weiteren Counter aufzeichnen
    registry.record_counter(component, "local_counter", 58, None);

    // Registry leeren
    registry.clear();

    // Prüfen, dass keine Collectors mehr vorhanden sind
    assert_eq!(registry.collectors().len(), 0);

    // Überprüfen, dass die Metriken zuvor korrekt aufgezeichnet wurden
    let metrics = collector_ref.query_metrics(component);

    // Counter prüfen
    assert!(metrics.contains_key("local_counter"));
    let counter_points = &metrics["local_counter"];
    assert_eq!(counter_points.len(), 2);
    assert_eq!(counter_points[0].value, 42.0);
    assert_eq!(counter_points[1].value, 58.0);

    // Gauge prüfen
    assert!(metrics.contains_key("local_gauge"));
    let gauge_points = &metrics["local_gauge"];
    assert_eq!(gauge_points.len(), 1);
    assert_eq!(gauge_points[0].value, std::f64::consts::PI);

    // Histogram prüfen
    assert!(metrics.contains_key("local_histogram"));
    let histogram_points = &metrics["local_histogram"];
    assert_eq!(histogram_points.len(), 1);
    assert_eq!(histogram_points[0].value, std::f64::consts::E);

    // Event prüfen
    assert!(metrics.contains_key("local_event"));
    let event_points = &metrics["local_event"];
    assert_eq!(event_points.len(), 1);
    assert_eq!(event_points[0].value, 500.0); // Millisekunden als float

    // Prüfe, dass Labels existieren und korrekt sind
    assert!(!event_points[0].labels.is_empty());
    assert_eq!(
        event_points[0].labels.get("local_event_type").unwrap(),
        "test"
    );
}

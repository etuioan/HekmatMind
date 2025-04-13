// Tests für die TestRegistry
use std::time::Duration;

use crate::test_registry::TestRegistry;
use hekmat_mind::telemetry::collector::QueryableCollector;
use hekmat_mind::telemetry::in_memory::InMemoryCollector;

mod test_registry; // Import des test_registry-Moduls

#[test]
fn test_basic_test_registry_functionality() {
    println!("TEST START: test_basic_test_registry_functionality");

    // Neue TestRegistry erstellen
    let mut test_registry = TestRegistry::new();
    assert_eq!(
        test_registry.collectors().len(),
        0,
        "Frische TestRegistry sollte leer sein"
    );

    // Einen Collector registrieren
    let collector = Box::new(InMemoryCollector::new(100));
    test_registry.register(collector);
    assert_eq!(
        test_registry.collectors().len(),
        1,
        "TestRegistry sollte einen Collector haben"
    );

    // Metriken aufzeichnen
    test_registry.record_counter("test_component", "test_counter", 42, None);
    test_registry.record_gauge("test_component", "test_gauge", 84.5, None);
    test_registry.record_histogram("test_component", "test_histogram", 100.0, None);
    test_registry.record_event(
        "test_component",
        "test_event",
        Duration::from_millis(123),
        None,
    );

    // TestRegistry löschen
    test_registry.clear();
    assert_eq!(
        test_registry.collectors().len(),
        0,
        "Gelöschte TestRegistry sollte leer sein"
    );

    println!("TEST ENDE: test_basic_test_registry_functionality");
}

#[test]
fn test_test_registry_isolation() {
    println!("TEST START: test_test_registry_isolation");

    // Zwei unabhängige TestRegistry-Instanzen erstellen
    let mut registry1 = TestRegistry::new();
    let mut registry2 = TestRegistry::new();

    // Collectors für beide Registries erstellen und registrieren
    let collector1 = Box::new(InMemoryCollector::new(100));
    let collector2 = Box::new(InMemoryCollector::new(100));

    registry1.register(collector1);
    registry2.register(collector2);

    // In registry1 Metriken aufzeichnen
    registry1.record_counter("test_component", "isolation_counter", 10, None);

    // Zweiten InMemoryCollector aus registry2 extrahieren und prüfen
    let collectors2 = registry2.collectors();
    let collector_ref2 = collectors2[0]
        .as_any()
        .downcast_ref::<InMemoryCollector>()
        .expect("Collector ist kein InMemoryCollector");

    // Überprüfen, dass registry2 die Metrik von registry1 nicht enthält
    let metrics = collector_ref2.query_stats("test_component", "isolation_counter");
    assert!(
        metrics.is_none(),
        "Die zweite Registry sollte keine Metriken der ersten Registry enthalten"
    );

    println!("TEST ENDE: test_test_registry_isolation");
}

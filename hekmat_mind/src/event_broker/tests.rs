//! Unit-Tests für den EventBroker
//!
//! Diese Datei enthält die Unit-Tests für die EventBroker-Komponente,
//! um die Modularität und Teststruktur des Projekts zu vereinheitlichen.

#[cfg(test)]
mod event_broker_tests {
    use crate::event_broker::EventBroker;
    use std::sync::Arc;

    #[test]
    fn test_subscribe_and_publish() {
        let broker = EventBroker::new();
        let mut received = false;

        // Für einen String-Event anmelden
        broker.subscribe(|event: Arc<String>| {
            assert_eq!(*event, "test");
            received = true;
        });

        // Event veröffentlichen
        broker.publish(String::from("test"));

        // Überprüfen, dass das Event verarbeitet wurde
        assert!(received);
    }

    #[test]
    fn test_multiple_subscribers() {
        let broker = EventBroker::new();
        let mut count = 0;

        // Mehrere Abonnenten für denselben Ereignistyp registrieren
        broker.subscribe(|_: Arc<i32>| {
            count += 1;
        });

        broker.subscribe(|_: Arc<i32>| {
            count += 1;
        });

        // Ereignis veröffentlichen
        broker.publish(42);

        // Überprüfen, dass beide Abonnenten das Ereignis erhalten haben
        assert_eq!(count, 2);
    }

    #[test]
    fn test_type_safety() {
        let broker = EventBroker::new();
        let mut int_received = false;
        let mut string_received = false;

        // Für verschiedene Ereignistypen anmelden
        broker.subscribe(|_: Arc<i32>| {
            int_received = true;
        });

        broker.subscribe(|_: Arc<String>| {
            string_received = true;
        });

        // Nur ein i32-Ereignis veröffentlichen
        broker.publish(42);

        // Nur der i32-Abonnent sollte das Ereignis empfangen haben
        assert!(int_received);
        assert!(!string_received);

        // Zurücksetzen
        int_received = false;

        // Nur ein String-Ereignis veröffentlichen
        broker.publish(String::from("test"));

        // Nur der String-Abonnent sollte das Ereignis empfangen haben
        assert!(!int_received);
        assert!(string_received);
    }

    #[test]
    fn test_subscriber_count() {
        let broker = EventBroker::new();

        // Keine Abonnenten am Anfang
        assert_eq!(broker.subscriber_count(), 0);

        // Für einen i32-Event anmelden
        broker.subscribe(|_: Arc<i32>| {});
        assert_eq!(broker.subscriber_count(), 1);

        // Für einen String-Event anmelden
        broker.subscribe(|_: Arc<String>| {});
        assert_eq!(broker.subscriber_count(), 2);

        // Für einen anderen i32-Event anmelden
        broker.subscribe(|_: Arc<i32>| {});
        assert_eq!(broker.subscriber_count(), 3);
    }

    #[test]
    fn test_clear_subscribers() {
        let broker = EventBroker::new();

        // Einige Abonnenten hinzufügen
        broker.subscribe(|_: Arc<i32>| {});
        broker.subscribe(|_: Arc<String>| {});
        assert_eq!(broker.subscriber_count(), 2);

        // Alle Abonnenten löschen
        broker.clear_subscribers();
        assert_eq!(broker.subscriber_count(), 0);

        // Veröffentlichung sollte keine Fehler verursachen
        broker.publish(42);
        broker.publish(String::from("test"));
    }
}

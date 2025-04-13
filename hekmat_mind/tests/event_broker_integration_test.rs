//! Integrationstest für den EventBroker
//!
//! Dieser Test prüft, ob der EventBroker korrekt zwischen verschiedenen Modulen kommunizieren kann.

use hekmat_mind::EventBroker;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Testeereignisse
#[derive(Debug, Clone)]
struct NeuralEvent {
    neuron_id: usize,
    activation: f64,
}

#[derive(Debug, Clone)]
struct SystemEvent {
    message: String,
}

// Simuliert ein Modul im System
struct TestModule {
    name: String,
    received_events: Arc<Mutex<Vec<String>>>,
}

impl TestModule {
    fn new(name: &str) -> Self {
        TestModule {
            name: name.to_string(),
            received_events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn register_with_broker(&self, broker: &EventBroker) {
        let name = self.name.clone();
        let events = Arc::clone(&self.received_events);

        // Registriere für NeuralEvent
        broker.subscribe(move |event: Arc<NeuralEvent>| {
            let msg = format!(
                "{} received NeuralEvent: neuron_id={}, activation={}",
                name, event.neuron_id, event.activation
            );
            events.lock().unwrap().push(msg);
        });

        let name = self.name.clone();
        let events = Arc::clone(&self.received_events);

        // Registriere für SystemEvent
        broker.subscribe(move |event: Arc<SystemEvent>| {
            let msg = format!("{} received SystemEvent: message={}", name, event.message);
            events.lock().unwrap().push(msg);
        });
    }

    fn received_event_count(&self) -> usize {
        self.received_events.lock().unwrap().len()
    }

    fn get_received_events(&self) -> Vec<String> {
        self.received_events.lock().unwrap().clone()
    }
}

#[test]
fn test_cross_module_communication() {
    // EventBroker erstellen
    let broker = EventBroker::new();

    // Module erstellen
    let neural_module = TestModule::new("NeuralModule");
    let system_module = TestModule::new("SystemModule");

    // Module mit dem Broker verbinden
    neural_module.register_with_broker(&broker);
    system_module.register_with_broker(&broker);

    // Ereignisse veröffentlichen
    broker.publish(NeuralEvent {
        neuron_id: 42,
        activation: 0.75,
    });

    broker.publish(SystemEvent {
        message: "System startup complete".to_string(),
    });

    // Kurz warten, um sicherzustellen, dass alle Ereignisse verarbeitet wurden
    thread::sleep(Duration::from_millis(10));

    // Überprüfen, ob beide Module die Ereignisse empfangen haben
    assert_eq!(neural_module.received_event_count(), 2);
    assert_eq!(system_module.received_event_count(), 2);

    // Prüfen, ob der Inhalt korrekt ist
    let neural_events = neural_module.get_received_events();
    let system_events = system_module.get_received_events();

    // Beide Module sollten das NeuralEvent empfangen haben
    assert!(neural_events.iter().any(|e| e.contains("neuron_id=42")));
    assert!(system_events.iter().any(|e| e.contains("neuron_id=42")));

    // Beide Module sollten das SystemEvent empfangen haben
    assert!(
        neural_events
            .iter()
            .any(|e| e.contains("System startup complete"))
    );
    assert!(
        system_events
            .iter()
            .any(|e| e.contains("System startup complete"))
    );
}

#[test]
fn test_multithreaded_event_processing() {
    // EventBroker erstellen
    let broker = Arc::new(EventBroker::new());

    // Module erstellen
    let module1 = Arc::new(TestModule::new("Module1"));
    let module2 = Arc::new(TestModule::new("Module2"));

    // Module mit dem Broker verbinden
    module1.register_with_broker(&broker);
    module2.register_with_broker(&broker);

    // Anzahl der zu sendenden Ereignisse
    let event_count = 100;

    // Thread 1: Sendet NeuralEvents
    let broker_clone = Arc::clone(&broker);
    let thread1 = thread::spawn(move || {
        for i in 0..event_count {
            broker_clone.publish(NeuralEvent {
                neuron_id: i,
                activation: i as f64 / 100.0,
            });

            // Kleine Verzögerung, um Thread-Wechsel zu ermöglichen
            if i % 10 == 0 {
                thread::sleep(Duration::from_micros(1));
            }
        }
    });

    // Thread 2: Sendet SystemEvents
    let broker_clone = Arc::clone(&broker);
    let thread2 = thread::spawn(move || {
        for i in 0..event_count {
            broker_clone.publish(SystemEvent {
                message: format!("System message {}", i),
            });

            // Kleine Verzögerung, um Thread-Wechsel zu ermöglichen
            if i % 10 == 0 {
                thread::sleep(Duration::from_micros(1));
            }
        }
    });

    // Auf Beendigung der Threads warten
    thread1.join().unwrap();
    thread2.join().unwrap();

    // Kurz warten, um sicherzustellen, dass alle Ereignisse verarbeitet wurden
    thread::sleep(Duration::from_millis(20));

    // Überprüfen, ob alle Ereignisse empfangen wurden
    // Jedes Modul sollte 2*event_count Ereignisse empfangen haben (beide Ereignistypen)
    assert_eq!(module1.received_event_count(), 2 * event_count);
    assert_eq!(module2.received_event_count(), 2 * event_count);

    // Prüfen, ob alle NeuralEvents empfangen wurden
    for i in 0..event_count {
        let neuron_id_str = format!("neuron_id={}", i);
        assert!(
            module1
                .get_received_events()
                .iter()
                .any(|e| e.contains(&neuron_id_str))
        );
        assert!(
            module2
                .get_received_events()
                .iter()
                .any(|e| e.contains(&neuron_id_str))
        );
    }

    // Prüfen, ob alle SystemEvents empfangen wurden
    for i in 0..event_count {
        let message_str = format!("System message {}", i);
        assert!(
            module1
                .get_received_events()
                .iter()
                .any(|e| e.contains(&message_str))
        );
        assert!(
            module2
                .get_received_events()
                .iter()
                .any(|e| e.contains(&message_str))
        );
    }
}

//! # EventBroker-Modul
//!
//! Dieses Modul implementiert einen typsicheren, Thread-sicheren
//! Event Broker für die Kommunikation zwischen Systemkomponenten.
//!
//! ## Überblick
//!
//! Der EventBroker ist ein zentrales Element im HekmatMind-System und ermöglicht
//! die lose Kopplung zwischen verschiedenen Komponenten durch einen
//! Publish-Subscribe-Mechanismus. Er sorgt dafür, dass Komponenten miteinander
//! kommunizieren können, ohne direkte Abhängigkeiten untereinander zu haben.
//!
//! ## Funktionsweise
//!
//! - Komponenten können sich für bestimmte Ereignistypen registrieren (subscribe)
//! - Andere Komponenten können Ereignisse veröffentlichen (publish)
//! - Der EventBroker leitet Ereignisse an die registrierten Subscriber weiter
//! - Typsicherheit wird durch Rusts Typsystem gewährleistet
//! - Thread-Sicherheit wird durch `RwLock` implementiert
//!
//! ## Beispiel-Verwendung
//!
//! ```
//! use hekmat_mind::EventBroker;
//! use std::sync::Arc;
//!
//! // Ein einfacher Ereignistyp
//! #[derive(Debug)]
//! struct NeuronEvent { id: usize, activation: f64 }
//!
//! // EventBroker erstellen
//! let broker = EventBroker::new();
//!
//! // Für ein Ereignis registrieren
//! broker.subscribe(|event: Arc<NeuronEvent>| {
//!     println!("Neuron {} wurde mit Stärke {} aktiviert",
//!              event.id, event.activation);
//! });
//!
//! // Ein Ereignis veröffentlichen
//! broker.publish(NeuronEvent { id: 42, activation: 0.8 });
//! ```

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Typ-Alias für die Funktion, die ein Ereignis verarbeitet.
///
/// Diese Funktion nimmt ein typloses Ereignis entgegen (`Arc<dyn Any + Send + Sync>`),
/// das später auf den konkreten Typ gedowncastet wird. Die Funktion muss
/// Thread-sicher sein (`Send + Sync`) und kann zwischen Threads verschoben werden.
type SubscriberFn = Box<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync>;

/// Der EventBroker dient als zentraler Kommunikationsmechanismus
/// zwischen verschiedenen Komponenten des HekmatMind-Systems.
///
/// Er ermöglicht typsichere, asynchrone Kommunikation zwischen Modulen,
/// ohne dass diese direkt voneinander abhängig sein müssen.
///
/// # Implementierungsdetails
///
/// Intern verwendet der EventBroker eine Hashmap, die Typinformationen auf
/// eine Liste von Subscriber-Funktionen abbildet. Wenn ein Ereignis veröffentlicht wird,
/// werden alle für diesen Typ registrierten Funktionen aufgerufen.
///
/// Der EventBroker ist Thread-sicher durch den Einsatz von `RwLock`. Mehrere Threads können
/// gleichzeitig lesen (Ereignisse veröffentlichen), aber Schreibzugriffe (Hinzufügen/Entfernen
/// von Subscribern) sind exklusiv.
#[derive(Default)]
pub struct EventBroker {
    /// Speichert die Subscriber-Funktionen, indiziert nach Event-Typ.
    ///
    /// - Schlüssel: `TypeId` des Ereignistyps
    /// - Wert: Liste von Funktionen, die bei Ereignissen dieses Typs aufgerufen werden
    ///
    /// `RwLock` gewährleistet die Thread-Sicherheit, sodass der EventBroker
    /// sicher zwischen Threads geteilt werden kann.
    subscribers: RwLock<HashMap<TypeId, Vec<SubscriberFn>>>,
}

impl EventBroker {
    /// Erstellt eine neue EventBroker-Instanz.
    ///
    /// Diese Methode initialisiert einen leeren EventBroker ohne registrierte Subscriber.
    ///
    /// # Beispiel
    ///
    /// ```
    /// use hekmat_mind::EventBroker;
    ///
    /// let broker = EventBroker::new();
    /// ```
    pub fn new() -> Self {
        EventBroker {
            subscribers: RwLock::new(HashMap::new()),
        }
    }

    /// Registriert einen Subscriber für einen bestimmten Ereignistyp.
    ///
    /// Der Callback wird aufgerufen, wenn ein Ereignis des spezifizierten Typs
    /// veröffentlicht wird. Der Typ wird automatisch aus der Signatur des
    /// Callbacks ermittelt.
    ///
    /// # Typparameter
    ///
    /// - `T`: Der Typ des Ereignisses, für das der Subscriber registriert wird
    /// - `F`: Der Typ der Callback-Funktion
    ///
    /// # Parameter
    ///
    /// - `callback`: Die Funktion, die aufgerufen wird, wenn ein Ereignis vom Typ `T` veröffentlicht wird.
    ///   Die Funktion erhält eine Arc-Referenz auf das Ereignis.
    ///
    /// # Beispiel
    /// ```
    /// use hekmat_mind::EventBroker;
    /// use std::sync::Arc;
    ///
    /// let broker = EventBroker::new();
    ///
    /// // Einen Subscriber für String-Ereignisse registrieren
    /// broker.subscribe(|event: Arc<String>| {
    ///     println!("Received event: {}", event);
    /// });
    /// ```
    pub fn subscribe<T, F>(&self, callback: F)
    where
        T: 'static + Any + Send + Sync,
        F: Fn(Arc<T>) + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<T>();

        // Erstellt einen Wrapper, der das typenlose Ereignis auf den konkreten Typ castet
        let callback_wrapper = Box::new(move |event: Arc<dyn Any + Send + Sync>| {
            if let Ok(event) = event.downcast::<T>() {
                callback(event);
            }
        });

        // Schreibzugriff auf die Subscriber-Map
        let mut subscribers = self.subscribers.write().unwrap();

        // Fügt den Callback zur Liste für diesen Typ hinzu
        subscribers
            .entry(type_id)
            .or_default()
            .push(callback_wrapper);
    }

    /// Veröffentlicht ein Ereignis an alle registrierten Subscriber.
    ///
    /// Diese Methode verteilt das Ereignis an alle Subscriber, die für den
    /// entsprechenden Ereignistyp registriert sind.
    ///
    /// # Typparameter
    ///
    /// - `T`: Der Typ des zu veröffentlichenden Ereignisses
    ///
    /// # Parameter
    ///
    /// - `event`: Das Ereignis, das veröffentlicht werden soll
    ///
    /// # Beispiel
    /// ```
    /// use hekmat_mind::EventBroker;
    ///
    /// let broker = EventBroker::new();
    /// broker.publish(String::from("Hello, World!"));
    /// ```
    pub fn publish<T>(&self, event: T)
    where
        T: 'static + Any + Send + Sync,
    {
        // Ereignis in Arc einpacken für Thread-sicheres Teilen
        let event = Arc::new(event);
        let type_id = (*event).type_id();

        // Lesezugriff auf die Subscriber-Map
        let subscribers = self.subscribers.read().unwrap();

        // Alle Subscriber für diesen Typ benachrichtigen
        if let Some(callbacks) = subscribers.get(&type_id) {
            let event = event as Arc<dyn Any + Send + Sync>;
            for callback in callbacks {
                callback(Arc::clone(&event));
            }
        }
    }

    /// Entfernt alle Subscriber für einen bestimmten Ereignistyp.
    ///
    /// Diese Methode löscht alle Callback-Funktionen, die für den
    /// angegebenen Ereignistyp registriert sind.
    ///
    /// # Typparameter
    ///
    /// - `T`: Der Ereignistyp, für den alle Subscriber entfernt werden sollen
    pub fn clear_subscribers<T>(&self)
    where
        T: 'static + Any + Send + Sync,
    {
        let type_id = TypeId::of::<T>();
        let mut subscribers = self.subscribers.write().unwrap();
        subscribers.remove(&type_id);
    }

    /// Gibt die Anzahl der registrierten Subscriber für einen bestimmten Ereignistyp zurück.
    ///
    /// # Typparameter
    ///
    /// - `T`: Der Ereignistyp, für den die Anzahl der Subscriber zurückgegeben werden soll
    ///
    /// # Rückgabewert
    ///
    /// Die Anzahl der registrierten Subscriber für den Typ `T`
    pub fn subscriber_count<T>(&self) -> usize
    where
        T: 'static + Any + Send + Sync,
    {
        let type_id = TypeId::of::<T>();
        let subscribers = self.subscribers.read().unwrap();

        subscribers
            .get(&type_id)
            .map_or(0, |callbacks| callbacks.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Teststruktur für Ereignisse in den Unit-Tests.
    #[derive(Debug, Clone)]
    struct TestEvent {
        /// Eindeutige ID des Ereignisses
        id: usize,
        /// Nachrichteninhalt des Ereignisses
        message: String,
    }

    /// Zweiter Ereignistyp für Tests der Typsicherheit.
    #[derive(Debug, Clone)]
    struct OtherEvent {
        /// Numerischer Wert des Ereignisses
        _value: f64,
    }

    /// Test für die grundlegende Funktionalität von subscribe und publish.
    ///
    /// Dieser Test prüft, ob:
    /// - Ein Subscriber korrekt registriert wird
    /// - Ein veröffentlichtes Ereignis den Subscriber erreicht
    /// - Die Ereignisdaten korrekt übermittelt werden
    #[test]
    fn test_subscribe_and_publish() {
        let broker = EventBroker::new();
        let counter = Arc::new(AtomicUsize::new(0));

        // Subscriber registrieren
        let counter_clone = Arc::clone(&counter);
        broker.subscribe(move |event: Arc<TestEvent>| {
            assert_eq!(event.id, 42);
            assert_eq!(event.message, "Test");
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Event veröffentlichen
        broker.publish(TestEvent {
            id: 42,
            message: "Test".to_string(),
        });

        // Prüfen, ob der Subscriber das Event erhalten hat
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_multiple_subscribers() {
        let broker = EventBroker::new();
        let counter = Arc::new(AtomicUsize::new(0));

        // Zwei Subscriber registrieren
        for _ in 0..2 {
            let counter_clone = Arc::clone(&counter);
            broker.subscribe(move |_: Arc<TestEvent>| {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            });
        }

        // Event veröffentlichen
        broker.publish(TestEvent {
            id: 1,
            message: "Test".to_string(),
        });

        // Prüfen, ob beide Subscriber das Event erhalten haben
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_type_safety() {
        let broker = EventBroker::new();
        let test_counter = Arc::new(AtomicUsize::new(0));
        let other_counter = Arc::new(AtomicUsize::new(0));

        // Subscriber für TestEvent registrieren
        let test_counter_clone = Arc::clone(&test_counter);
        broker.subscribe(move |_: Arc<TestEvent>| {
            test_counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Subscriber für OtherEvent registrieren
        let other_counter_clone = Arc::clone(&other_counter);
        broker.subscribe(move |_: Arc<OtherEvent>| {
            other_counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        // TestEvent veröffentlichen
        broker.publish(TestEvent {
            id: 1,
            message: "Test".to_string(),
        });

        // Prüfen, dass nur TestEvent-Subscriber das Event erhalten hat
        assert_eq!(test_counter.load(Ordering::SeqCst), 1);
        assert_eq!(other_counter.load(Ordering::SeqCst), 0);

        // OtherEvent veröffentlichen
        broker.publish(OtherEvent {
            _value: std::f64::consts::PI,
        });

        // Prüfen, dass nur OtherEvent-Subscriber das Event erhalten hat
        assert_eq!(test_counter.load(Ordering::SeqCst), 1);
        assert_eq!(other_counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_clear_subscribers() {
        let broker = EventBroker::new();
        let counter = Arc::new(AtomicUsize::new(0));

        // Subscriber registrieren
        let counter_clone = Arc::clone(&counter);
        broker.subscribe(move |_: Arc<TestEvent>| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Event veröffentlichen
        broker.publish(TestEvent {
            id: 1,
            message: "Test".to_string(),
        });

        // Prüfen, dass der Subscriber das Event erhalten hat
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        // Subscriber entfernen
        broker.clear_subscribers::<TestEvent>();

        // Event erneut veröffentlichen
        broker.publish(TestEvent {
            id: 2,
            message: "Test again".to_string(),
        });

        // Prüfen, dass der Subscriber das Event nicht erhalten hat
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_subscriber_count() {
        let broker = EventBroker::new();

        // Prüfen, dass initial keine Subscriber vorhanden sind
        assert_eq!(broker.subscriber_count::<TestEvent>(), 0);

        // Zwei Subscriber registrieren
        broker.subscribe(|_: Arc<TestEvent>| {});
        broker.subscribe(|_: Arc<TestEvent>| {});

        // Prüfen, dass zwei Subscriber registriert sind
        assert_eq!(broker.subscriber_count::<TestEvent>(), 2);

        // Subscriber für einen anderen Typ registrieren
        broker.subscribe(|_: Arc<OtherEvent>| {});

        // Prüfen, dass die Anzahl korrekt ist
        assert_eq!(broker.subscriber_count::<TestEvent>(), 2);
        assert_eq!(broker.subscriber_count::<OtherEvent>(), 1);

        // Subscriber entfernen
        broker.clear_subscribers::<TestEvent>();

        // Prüfen, dass keine Subscriber mehr vorhanden sind
        assert_eq!(broker.subscriber_count::<TestEvent>(), 0);
        assert_eq!(broker.subscriber_count::<OtherEvent>(), 1);
    }
}

// Telemetrie-Modul für HekmatMind
//
// Dieses Modul stellt die Grundlage für eine erweiterbare Telemetrie-Infrastruktur
// bereit, die sowohl für Laufzeitüberwachung als auch für Leistungstests genutzt wird.

use std::collections::HashMap;
use std::fmt;
use std::sync::RwLock;
use std::time::{Duration, Instant};

pub mod collector;
pub mod in_memory;

#[cfg(test)]
mod in_memory_tests;
#[cfg(test)]
mod tests;

/// Repräsentiert einen Metrik-Typ im Telemetrie-System
#[derive(Debug, Clone, PartialEq)]
pub enum MetricType {
    /// Zähler-Metrik (kumulativ, nur steigend)
    Counter,
    /// Messwert-Metrik (kann steigen oder fallen)
    Gauge,
    /// Histogramm-Metrik (für Verteilungen und Percentile)
    Histogram,
    /// Ereignis-Metrik (für zeitbasierte Ereignisse)
    Event,
}

impl fmt::Display for MetricType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MetricType::Counter => write!(f, "counter"),
            MetricType::Gauge => write!(f, "gauge"),
            MetricType::Histogram => write!(f, "histogram"),
            MetricType::Event => write!(f, "event"),
        }
    }
}

/// Ein einzelner Metrikpunkt mit Zeitstempel
#[derive(Debug, Clone)]
pub struct MetricPoint {
    /// Zeitstempel der Metrik
    pub timestamp: Instant,
    /// Metrik-Typ
    pub metric_type: MetricType,
    /// Metrik-Wert
    pub value: f64,
    /// Zusätzliche Metrik-Labels
    pub labels: HashMap<String, String>,
}

/// Zentrales Telemetrie-Register für alle Collector-Instanzen
pub struct TelemetryRegistry {
    collectors: Vec<Box<dyn collector::TelemetryCollector>>,
}

/// Default-Implementierung für TelemetryRegistry
impl Default for TelemetryRegistry {
    /// Erzeugt eine neue, leere Registry
    fn default() -> Self {
        Self::new()
    }
}

// Manuelle Clone-Implementierung, da Box<dyn TelemetryCollector> nicht automatisch klonbar ist
impl Clone for TelemetryRegistry {
    fn clone(&self) -> Self {
        // Eine vereinfachte Implementierung, die eine neue Registry zurückgibt
        // Warnung: Diese Implementierung klont nicht die tatsächlichen Collectors

        TelemetryRegistry::new()
    }
}

impl TelemetryRegistry {
    pub fn new() -> Self {
        TelemetryRegistry {
            collectors: Vec::new(),
        }
    }

    /// Registriert einen neuen Telemetrie-Collector
    pub fn register(&mut self, collector: Box<dyn collector::TelemetryCollector>) {
        self.collectors.push(collector);
    }

    /// Gibt eine Referenz auf alle registrierten Collectors zurück
    pub fn collectors(&self) -> &Vec<Box<dyn collector::TelemetryCollector>> {
        &self.collectors
    }

    /// Entfernt alle registrierten Collectors
    pub fn clear(&mut self) {
        self.collectors.clear();
    }

    /// Zeichnet einen Zähler-Metrikwert auf
    pub fn record_counter(
        &self,
        component: &str,
        name: &str,
        value: u64,
        labels: Option<HashMap<String, String>>,
    ) {
        for collector in &self.collectors {
            collector.record_counter(component, name, value, labels.clone());
        }
    }

    /// Zeichnet einen Messwert auf
    pub fn record_gauge(
        &self,
        component: &str,
        name: &str,
        value: f64,
        labels: Option<HashMap<String, String>>,
    ) {
        for collector in &self.collectors {
            collector.record_gauge(component, name, value, labels.clone());
        }
    }

    /// Zeichnet einen Histogramm-Wert auf
    pub fn record_histogram(
        &self,
        component: &str,
        name: &str,
        value: f64,
        labels: Option<HashMap<String, String>>,
    ) {
        for collector in &self.collectors {
            collector.record_histogram(component, name, value, labels.clone());
        }
    }

    /// Zeichnet ein Ereignis mit Dauer auf
    pub fn record_event(
        &self,
        component: &str,
        name: &str,
        duration: Duration,
        labels: Option<HashMap<String, String>>,
    ) {
        for collector in &self.collectors {
            collector.record_event(component, name, duration, labels.clone());
        }
    }
}

// Implementierung des TelemetryCollector-Traits für TelemetryRegistry
impl collector::TelemetryCollector for TelemetryRegistry {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn record_counter(
        &self,
        component: &str,
        name: &str,
        value: u64,
        labels: Option<HashMap<String, String>>,
    ) {
        // Leite den Aufruf an alle registrierten Collectors weiter
        for collector in &self.collectors {
            collector.record_counter(component, name, value, labels.clone());
        }
    }

    fn record_gauge(
        &self,
        component: &str,
        name: &str,
        value: f64,
        labels: Option<HashMap<String, String>>,
    ) {
        // Leite den Aufruf an alle registrierten Collectors weiter
        for collector in &self.collectors {
            collector.record_gauge(component, name, value, labels.clone());
        }
    }

    fn record_histogram(
        &self,
        component: &str,
        name: &str,
        value: f64,
        labels: Option<HashMap<String, String>>,
    ) {
        // Leite den Aufruf an alle registrierten Collectors weiter
        for collector in &self.collectors {
            collector.record_histogram(component, name, value, labels.clone());
        }
    }

    fn record_event(
        &self,
        component: &str,
        name: &str,
        duration: Duration,
        labels: Option<HashMap<String, String>>,
    ) {
        // Leite den Aufruf an alle registrierten Collectors weiter
        for collector in &self.collectors {
            collector.record_event(component, name, duration, labels.clone());
        }
    }
}

/// Globale Telemetrie-Instanz (Singleton)
static REGISTRY: once_cell::sync::Lazy<RwLock<TelemetryRegistry>> =
    once_cell::sync::Lazy::new(|| RwLock::new(TelemetryRegistry::new()));

/// Zugriff auf die globale Telemetrie-Registry
pub fn registry() -> std::sync::LockResult<std::sync::RwLockReadGuard<'static, TelemetryRegistry>> {
    REGISTRY.read()
}

/// Zugriff auf die globale Telemetrie-Registry für Änderungen
pub fn registry_mut()
-> std::sync::LockResult<std::sync::RwLockWriteGuard<'static, TelemetryRegistry>> {
    REGISTRY.write()
}

// Telemetrie-Collector-Traits für HekmatMind
//
// Diese Traits definieren die Plugin-Schnittstelle für Telemetrie-Implementierungen
// und ermöglichen eine modulare Erweiterung der Telemetrie-Infrastruktur.

use std::any::Any;
use std::collections::HashMap;
use std::time::Duration;

/// Haupttrait für Telemetrie-Collector
///
/// Dieser Trait definiert die Kernschnittstelle, die alle Telemetrie-Implementierungen
/// bereitstellen müssen. Er unterstützt verschiedene Metriktypen und ist die Basis
/// für das Plugin-System der Telemetrie-Architektur.
pub trait TelemetryCollector: Send + Sync {
    /// Zeichnet einen Zähler-Metrikwert auf
    fn record_counter(
        &self,
        component: &str,
        name: &str,
        value: u64,
        labels: Option<HashMap<String, String>>,
    );

    /// Zeichnet einen Messwert auf
    fn record_gauge(
        &self,
        component: &str,
        name: &str,
        value: f64,
        labels: Option<HashMap<String, String>>,
    );

    /// Zeichnet einen Histogramm-Wert auf
    fn record_histogram(
        &self,
        component: &str,
        name: &str,
        value: f64,
        labels: Option<HashMap<String, String>>,
    );

    /// Zeichnet ein Ereignis mit Dauer auf
    fn record_event(
        &self,
        component: &str,
        name: &str,
        duration: Duration,
        labels: Option<HashMap<String, String>>,
    );

    /// Optionaler Hook für Collector-Initialisierung
    fn initialize(&mut self) {}

    /// Optionaler Hook für Collector-Bereinigung
    fn shutdown(&mut self) {}

    /// Ermöglicht Downcasting für Typüberprüfung in Tests und speziellen Anwendungsfällen
    fn as_any(&self) -> &dyn Any;
}

/// Trait für Collector mit Abfragefunktionalität
///
/// Dieser Trait erweitert TelemetryCollector um die Fähigkeit,
/// gesammelte Metriken abzufragen. Dies ist nützlich für Implementierungen,
/// die für Benchmarking und Analyse verwendet werden.
pub trait QueryableCollector: TelemetryCollector {
    /// Fragt Metriken für eine bestimmte Komponente ab
    fn query_metrics(&self, component: &str)
    -> HashMap<String, Vec<crate::telemetry::MetricPoint>>;

    /// Fragt aggregierte Statistiken für eine bestimmte Metrik ab
    fn query_stats(&self, component: &str, metric: &str) -> Option<MetricStats>;
}

/// Aggregierte Statistiken für eine Metrik
#[derive(Debug, Clone)]
pub struct MetricStats {
    /// Minimalwert
    pub min: f64,
    /// Maximalwert
    pub max: f64,
    /// Durchschnittswert
    pub avg: f64,
    /// Medianwert
    pub median: f64,
    /// 95-Perzentil
    pub p95: f64,
    /// 99-Perzentil
    pub p99: f64,
    /// Anzahl der Messpunkte
    pub count: usize,
}

/// Trait für Collector mit Exportfunktionalität
///
/// Dieser Trait ermöglicht das Exportieren von Telemetriedaten in verschiedene
/// Formate oder externe Systeme.
pub trait ExportableCollector: TelemetryCollector {
    /// Exportiert Metriken in ein bestimmtes Format
    fn export(&self, format: ExportFormat) -> Result<String, ExportError>;
}

/// Unterstützte Exportformate
#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    /// JSON-Format
    Json,
    /// CSV-Format
    Csv,
    /// Prometheus-kompatibles Format
    Prometheus,
}

/// Fehler beim Exportieren von Metriken
#[derive(Debug)]
pub enum ExportError {
    /// Format wird nicht unterstützt
    UnsupportedFormat,
    /// Fehler bei der Serialisierung
    SerializationError(String),
    /// Andere Fehler
    Other(String),
}

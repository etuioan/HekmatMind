// In-Memory-Telemetrie-Collector für HekmatMind
//
// Diese Implementierung speichert alle Telemetriedaten im Arbeitsspeicher
// und ist besonders für Tests und Entwicklung geeignet.

use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use uuid::Uuid;

use super::MetricPoint;
use super::MetricType;
use super::collector::{MetricStats, QueryableCollector, TelemetryCollector};

/// Type-Alias für die Component-Metrik-Datenstruktur
type ComponentMetricMap = HashMap<String, HashMap<String, Vec<MetricPoint>>>;

/// In-Memory-Collector für Telemetriedaten
///
/// Speichert alle Telemetriedaten im Arbeitsspeicher und bietet
/// umfangreiche Abfragefunktionen. Besonders nützlich für Tests,
/// Entwicklung und Leistungsdiagnose.
#[derive(Clone)]
pub struct InMemoryCollector {
    /// Eindeutige ID dieses Collectors
    id: Uuid,
    /// Maximale Anzahl von Datenpunkten pro Metrik
    max_data_points: usize,
    /// Gespeicherte Metrikdaten pro Komponente und Metrikname
    data: Arc<RwLock<ComponentMetricMap>>,
}

impl InMemoryCollector {
    /// Erstellt einen neuen In-Memory-Collector mit gegebener Kapazität
    pub fn new(max_data_points: usize) -> Self {
        InMemoryCollector {
            id: Uuid::new_v4(),
            max_data_points,
            data: Arc::new(RwLock::new(ComponentMetricMap::new())),
        }
    }

    /// Gibt die eindeutige ID dieses Collectors zurück
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Generiert einen Schlüssel für die interne Datenspeicherung
    fn get_component_key(&self, component: &str) -> String {
        component.to_lowercase()
    }

    /// Fügt einen Metrikpunkt zum Speicher hinzu
    fn add_metric_point(
        &self,
        component: &str,
        name: &str,
        metric_type: MetricType,
        value: f64,
        labels: Option<HashMap<String, String>>,
    ) {
        let key = self.get_component_key(component);
        let labels = labels.unwrap_or_default();

        let point = MetricPoint {
            timestamp: Instant::now(),
            metric_type,
            value,
            labels,
        };

        if let Ok(mut data_guard) = self.data.write() {
            let component_map = data_guard.entry(key.clone()).or_insert_with(HashMap::new);
            let metric_points = component_map
                .entry(name.to_string())
                .or_insert_with(Vec::new);

            // Begrenze die Anzahl gespeicherter Punkte
            if metric_points.len() >= self.max_data_points {
                metric_points.remove(0);
            }

            metric_points.push(point);
        }
    }
}

impl TelemetryCollector for InMemoryCollector {
    fn record_counter(
        &self,
        component: &str,
        name: &str,
        value: u64,
        labels: Option<HashMap<String, String>>,
    ) {
        self.add_metric_point(component, name, MetricType::Counter, value as f64, labels);
    }

    fn record_gauge(
        &self,
        component: &str,
        name: &str,
        value: f64,
        labels: Option<HashMap<String, String>>,
    ) {
        self.add_metric_point(component, name, MetricType::Gauge, value, labels);
    }

    fn record_histogram(
        &self,
        component: &str,
        name: &str,
        value: f64,
        labels: Option<HashMap<String, String>>,
    ) {
        self.add_metric_point(component, name, MetricType::Histogram, value, labels);
    }

    fn record_event(
        &self,
        component: &str,
        name: &str,
        duration: Duration,
        labels: Option<HashMap<String, String>>,
    ) {
        let ms_duration = duration.as_secs_f64() * 1000.0;
        self.add_metric_point(component, name, MetricType::Event, ms_duration, labels);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl QueryableCollector for InMemoryCollector {
    fn query_metrics(&self, component: &str) -> HashMap<String, Vec<MetricPoint>> {
        let key = self.get_component_key(component);

        if let Ok(data_guard) = self.data.read() {
            if let Some(component_data) = data_guard.get(&key) {
                // Klonen der Daten für die Rückgabe
                let mut result = HashMap::new();
                for (metric_name, points) in component_data {
                    result.insert(metric_name.clone(), points.clone());
                }
                return result;
            }
        }

        HashMap::new()
    }

    fn query_stats(&self, component: &str, metric: &str) -> Option<MetricStats> {
        let component_key = self.get_component_key(component);

        if let Ok(data_guard) = self.data.read() {
            if let Some(component_data) = data_guard.get(&component_key) {
                if let Some(points) = component_data.get(metric) {
                    if points.is_empty() {
                        return None;
                    }

                    // Extrahiere die Werte
                    let mut values: Vec<f64> = points.iter().map(|p| p.value).collect();
                    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

                    let count = values.len();
                    let min = *values.first().unwrap_or(&0.0);
                    let max = *values.last().unwrap_or(&0.0);
                    let sum: f64 = values.iter().sum();
                    let avg = if count > 0 { sum / count as f64 } else { 0.0 };

                    // Berechne Perzentile
                    let median_idx = count / 2;
                    let median = if count > 0 { values[median_idx] } else { 0.0 };

                    let p95_idx = (count as f64 * 0.95) as usize;
                    let p95 = if p95_idx < count {
                        values[p95_idx]
                    } else {
                        max
                    };

                    let p99_idx = (count as f64 * 0.99) as usize;
                    let p99 = if p99_idx < count {
                        values[p99_idx]
                    } else {
                        max
                    };

                    return Some(MetricStats {
                        min,
                        max,
                        avg,
                        median,
                        p95,
                        p99,
                        count,
                    });
                }
            }
        }

        None
    }
}

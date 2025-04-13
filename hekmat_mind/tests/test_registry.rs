// Test-Registry für isolierte Tests
// Diese Datei bietet eine isolierte Test-Registry zur Verbesserung der Testunabhängigkeit

use hekmat_mind::telemetry::TelemetryRegistry;
use hekmat_mind::telemetry::collector::TelemetryCollector;

/// TestRegistry - Eine isolierte Registry für Tests
///
/// Diese Struktur bietet eine Alternative zur globalen Registry und ermöglicht
/// jedem Test eine eigene, isolierte Telemetrie-Umgebung.
#[derive(Clone)]
pub struct TestRegistry {
    registry: TelemetryRegistry,
}

impl Default for TestRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl TestRegistry {
    /// Erzeugt eine neue, leere Test-Registry
    pub fn new() -> Self {
        TestRegistry {
            registry: TelemetryRegistry::new(),
        }
    }

    /// Registriert einen neuen Collector
    pub fn register(&mut self, collector: Box<dyn TelemetryCollector>) {
        self.registry.register(collector);
    }

    /// Gibt eine Referenz auf alle registrierten Collectors zurück
    pub fn collectors(&self) -> &Vec<Box<dyn TelemetryCollector>> {
        self.registry.collectors()
    }

    /// Alias für collectors() für Kompatibilität mit Tests
    #[allow(dead_code)]
    pub fn get_collectors(&self) -> &Vec<Box<dyn TelemetryCollector>> {
        self.collectors()
    }

    /// Löscht alle registrierten Collectors
    pub fn clear(&mut self) {
        self.registry.clear();
    }

    /// Zeichnet einen Zähler-Metrikwert auf
    #[allow(dead_code)]
    pub fn record_counter(
        &self,
        component: &str,
        name: &str,
        value: u64,
        labels: Option<std::collections::HashMap<String, String>>,
    ) {
        self.registry.record_counter(component, name, value, labels);
    }

    /// Zeichnet einen Messwert auf
    #[allow(dead_code)]
    pub fn record_gauge(
        &self,
        component: &str,
        name: &str,
        value: f64,
        labels: Option<std::collections::HashMap<String, String>>,
    ) {
        self.registry.record_gauge(component, name, value, labels);
    }

    /// Zeichnet einen Histogramm-Wert auf
    #[allow(dead_code)]
    pub fn record_histogram(
        &self,
        component: &str,
        name: &str,
        value: f64,
        labels: Option<std::collections::HashMap<String, String>>,
    ) {
        self.registry
            .record_histogram(component, name, value, labels);
    }

    /// Zeichnet ein Ereignis mit Dauer auf
    #[allow(dead_code)]
    pub fn record_event(
        &self,
        component: &str,
        name: &str,
        duration: std::time::Duration,
        labels: Option<std::collections::HashMap<String, String>>,
    ) {
        self.registry
            .record_event(component, name, duration, labels);
    }
}

// Implementierung des TelemetryCollector-Traits für die TestRegistry
impl TelemetryCollector for TestRegistry {
    fn record_counter(
        &self,
        component: &str,
        name: &str,
        value: u64,
        labels: Option<std::collections::HashMap<String, String>>,
    ) {
        self.registry.record_counter(component, name, value, labels);
    }

    fn record_gauge(
        &self,
        component: &str,
        name: &str,
        value: f64,
        labels: Option<std::collections::HashMap<String, String>>,
    ) {
        // Delegiere an die innere Registry
        self.registry.record_gauge(component, name, value, labels);
    }

    fn record_histogram(
        &self,
        component: &str,
        name: &str,
        value: f64,
        labels: Option<std::collections::HashMap<String, String>>,
    ) {
        self.registry
            .record_histogram(component, name, value, labels);
    }

    fn record_event(
        &self,
        component: &str,
        name: &str,
        duration: std::time::Duration,
        labels: Option<std::collections::HashMap<String, String>>,
    ) {
        self.registry
            .record_event(component, name, duration, labels);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// Markiere TestRegistry als Send + Sync, da die innere TelemetryRegistry auch Send + Sync ist
unsafe impl Send for TestRegistry {}
unsafe impl Sync for TestRegistry {}

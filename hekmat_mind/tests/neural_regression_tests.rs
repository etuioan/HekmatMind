//! Integration-Tests für die Regressionsvalidierung der neuronalen Komponenten
//!
//! Diese Datei dient als Einstiegspunkt für die Ausführung aller neuronalen
//! Regressionstests als Integrationstests.

// Importiere die Tests als Module
#[path = "regression/neural/neuron_baseline_test.rs"]
mod neuron_baseline_tests;

#[path = "regression/neural/synapse_baseline_test.rs"]
mod synapse_baseline_tests;

#[path = "regression/neural/network_baseline_test.rs"]
mod network_baseline_tests;

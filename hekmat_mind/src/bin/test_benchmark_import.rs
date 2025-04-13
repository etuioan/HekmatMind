// Test für Benchmark-Import
//
// Dieses kleine Programm testet nur, ob das Benchmark-Modul korrekt
// importiert werden kann.

fn main() {
    println!("Test für Benchmark-Import");

    // Versuchen, direkt aus dem Benchmark-Modul zu importieren
    // Dies wird fehlschlagen, wenn das Modul nicht korrekt exportiert wird
    let config =
        hekmat_mind::benchmark::BenchmarkConfig::new("test_import", "Test für direkten Import");

    println!("Benchmark-Config: {:?}", config);
}

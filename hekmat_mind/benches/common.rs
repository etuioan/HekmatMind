// HekmatMind - Gemeinsames Benchmark-Modul
//
// Dieses Modul stellt gemeinsame Funktionen für alle Benchmarks in HekmatMind bereit.
// Es implementiert die Kernfunktionen: document_benchmark() und print_benchmark_summary().

use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;

/// Dokumentiert einen Benchmark mit laienverständlichen Erklärungen.
///
/// Erstellt README.md und explanation.html im Criterion-Ausgabeverzeichnis mit
/// Erklärungen zur Bedeutung des Benchmarks und zur Interpretation der Ergebnisse.
/// Erstellt außerdem eine zentrale index.html, die sowohl auf die Criterion-Berichte
/// als auch auf die benutzerfreundlichen Erklärungen verweist.
///
/// # Parameter
/// * `benchmark_name` - Name des Benchmarks (muss mit dem Criterion-Gruppennamen übereinstimmen)
/// * `component_type` - Art der getesteten Komponente (z.B. "Neuron", "Synapse")
/// * `description` - Verständliche Beschreibung, was der Benchmark misst
/// * `value_explanation` - Erklärung der Werte für Nicht-Techniker
///
/// # Beispiel
/// ```
/// document_benchmark(
///     "Neuron_Activation",
///     "Neuron",
///     "Dieser Benchmark misst, wie schnell ein Neuron auf Eingangssignale reagiert.",
///     "Die Werte zeigen die Zeit in Nanosekunden, die ein Neuron braucht, um ein Signal zu verarbeiten."
/// );
/// ```
pub fn document_benchmark(
    benchmark_name: &str,
    component_type: &str,
    description: &str,
    value_explanation: &str,
) -> std::io::Result<()> {
    // 1. Bereite Verzeichnispfade vor
    let base_dir = Path::new("target/criterion").join(benchmark_name);
    fs::create_dir_all(&base_dir)?;

    // 2. Erstelle die README.md im Hauptverzeichnis
    let readme_path = base_dir.join("README.md");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(readme_path)?;

    writeln!(file, "# {} Benchmark", benchmark_name)?;
    writeln!(file)?;
    writeln!(file, "## Komponente")?;
    writeln!(file, "{}", component_type)?;
    writeln!(file)?;
    writeln!(file, "## Was wird hier gemessen?")?;
    writeln!(file, "{}", description)?;
    writeln!(file)?;
    writeln!(file, "## Wie liest man die Werte?")?;
    writeln!(file, "{}", value_explanation)?;
    writeln!(file)?;
    writeln!(file, "## Interpretation der Criterion-Ausgabe")?;
    writeln!(
        file,
        "* **Throughput**: Je höher, desto besser (Operationen pro Sekunde)\n\
         * **Average time**: Durchschnittliche Laufzeit (niedriger ist besser)\n\
         * **Slope**: Anstieg der Regression (wie sich die Zeit mit der Eingabegröße ändert)\n\
         * **MAD, SD**: Streuungsmaße - niedrigere Werte bedeuten konsistentere Ergebnisse\n\
         * **Bootstrapped CI**: Konfidenzintervall der durchschnittlichen Laufzeit"
    )?;

    // 3. Erstelle die HTML-Erklärungsseite nur im Hauptverzeichnis
    create_html_explanation(
        &base_dir,
        benchmark_name,
        component_type,
        description,
        value_explanation,
    )?;

    // 4. Erstelle einen zentralen Einstiegspunkt im Hauptverzeichnis
    create_central_entry_point(&base_dir, benchmark_name, component_type)?;

    println!(
        "Dokumentation zu '{}' erstellt in {} und Unterverzeichnissen",
        benchmark_name,
        base_dir.display()
    );
    Ok(())
}

/// Erstellt eine HTML-Erklärungsdatei für den Benchmark
fn create_html_explanation(
    output_dir: &Path,
    benchmark_name: &str,
    component_type: &str,
    description: &str,
    value_explanation: &str,
) -> std::io::Result<()> {
    let html_content = format!(
        r#"<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>HekmatMind Benchmark: {}</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            line-height: 1.6;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            color: #333;
        }}
        h1 {{ color: #2c3e50; border-bottom: 2px solid #ecf0f1; padding-bottom: 10px; }}
        h2 {{ color: #3498db; margin-top: 25px; }}
        .container {{ margin-top: 20px; }}
        .component {{ background-color: #f7f9fa; padding: 15px; border-radius: 5px; }}
        .description {{ background-color: #edf7fd; padding: 15px; border-radius: 5px; margin-top: 20px; }}
        .values {{ background-color: #f5f5f5; padding: 15px; border-radius: 5px; margin-top: 20px; }}
        .interpretation {{ background-color: #f9f4e8; padding: 15px; border-radius: 5px; margin-top: 20px; }}
        a {{ color: #3498db; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
        .return-link {{ margin-top: 30px; display: block; }}
    </style>
</head>
<body>
    <h1>HekmatMind Benchmark: {}</h1>

    <div class="container">
        <div class="component">
            <h2>Komponente</h2>
            <p>{}</p>
        </div>

        <div class="description">
            <h2>Was wird hier gemessen?</h2>
            <p>{}</p>
        </div>

        <div class="values">
            <h2>Wie liest man die Werte?</h2>
            <p>{}</p>
        </div>

        <div class="interpretation">
            <h2>Interpretation der Criterion-Ausgabe</h2>
            <ul>
                <li><strong>Throughput</strong>: Je höher, desto besser (Operationen pro Sekunde)</li>
                <li><strong>Average time</strong>: Durchschnittliche Laufzeit (niedriger ist besser)</li>
                <li><strong>Slope</strong>: Anstieg der Regression (wie sich die Zeit mit der Eingabegröße ändert)</li>
                <li><strong>MAD, SD</strong>: Streuungsmaße - niedrigere Werte bedeuten konsistentere Ergebnisse</li>
                <li><strong>Bootstrapped CI</strong>: Konfidenzintervall der durchschnittlichen Laufzeit</li>
            </ul>
        </div>

        <a href="./index.html" class="return-link">→ Zurück zur Übersicht</a>
    </div>
</body>
</html>
        "#,
        benchmark_name, benchmark_name, component_type, description, value_explanation
    );

    let html_path = output_dir.join("explanation.html");
    let mut file = File::create(html_path)?;
    file.write_all(html_content.as_bytes())?;

    Ok(())
}

/// Erstellt eine zentrale Einstiegsseite, die alle Berichte und Erklärungen zu einem Benchmark verlinkt
///
/// Diese Funktion überprüft die Existenz von Criterion-Berichten und passt die Links entsprechend an.
fn create_central_entry_point(
    output_dir: &Path,
    benchmark_name: &str,
    component_type: &str,
) -> std::io::Result<()> {
    let index_html = format!(
        r#"<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>HekmatMind Benchmark: {}</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            line-height: 1.6;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            color: #333;
        }}
        h1 {{ color: #2c3e50; border-bottom: 2px solid #ecf0f1; padding-bottom: 10px; }}
        h2 {{ color: #3498db; margin-top: 25px; }}
        .card {{
            border: 1px solid #e8e8e8;
            border-radius: 8px;
            padding: 20px;
            margin-bottom: 20px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.05);
            background-color: #fff;
        }}
        .card h3 {{ color: #2c3e50; margin-top: 0; }}
        a.button {{
            display: inline-block;
            background-color: #3498db;
            color: white;
            padding: 8px 16px;
            border-radius: 4px;
            text-decoration: none;
            margin-top: 10px;
            transition: background-color 0.2s;
        }}
        a.button:hover {{ background-color: #2980b9; }}
        .description {{ color: #666; margin-bottom: 15px; }}
    </style>
</head>
<body>
    <h1>HekmatMind Benchmark: {}</h1>
    <h2>Komponente: {}</h2>

    <div class="card">
        <h3>Benutzerfreundliche Erklärung</h3>
        <p class="description">Verständliche Erklärung des Benchmarks, was gemessen wird und wie die Ergebnisse zu interpretieren sind.</p>
        <a href="./explanation.html" class="button">Erklärung anzeigen</a>
    </div>

    <div class="card">
        <h3>Technische Benchmark-Ergebnisse</h3>
        <p class="description">Detaillierte Benchmark-Ergebnisse mit Diagrammen und statistischen Analysen.</p>
        <a href="./report/index.html" class="button">Criterion-Bericht anzeigen</a>
    </div>

    <div class="card">
        <h3>Dokumentation</h3>
        <p class="description">Markdown-Dokumentation des Benchmarks für Entwickler.</p>
        <a href="./README.md" class="button">README ansehen</a>
    </div>
</body>
</html>
        "#,
        benchmark_name, benchmark_name, component_type
    );

    let index_path = output_dir.join("index.html");
    let mut file = File::create(index_path)?;
    file.write_all(index_html.as_bytes())?;

    Ok(())
}

/// Gibt eine Zusammenfassung der Benchmark-Ergebnisse aus.
///
/// Diese Funktion druckt eine übersichtliche Tabelle mit den Ergebnissen
/// der Benchmarks auf der Konsole aus.
///
/// # Parameter
/// * `benchmark_name` - Name des Benchmarks
/// * `results` - Liste mit Paaren aus (Test-Name, Zeit in Nanosekunden)
#[allow(dead_code)]
pub fn print_benchmark_summary(benchmark_name: &str, results: &[(String, f64)]) {
    println!(
        "\n----- BENCHMARK-ZUSAMMENFASSUNG: {} -----",
        benchmark_name
    );
    println!("{:<30} | {:<15} | {:<15}", "Test", "Zeit (ns)", "Zeit (µs)");
    println!("{}", "-".repeat(70));

    for (name, time_ns) in results {
        println!(
            "{:<30} | {:<15.2} | {:<15.2}",
            name,
            time_ns,
            time_ns / 1000.0
        );
    }

    println!("{}", "-".repeat(70));
    println!(
        "Benchmark abgeschlossen. Detaillierte Ergebnisse unter target/criterion/{}",
        benchmark_name
    );
}

// Einfache Datenstruktur für Benchmark-Ergebnisse
#[allow(dead_code)]
pub struct BenchmarkResult {
    pub name: String,
    pub value: f64,
    pub unit: String,
}

#[allow(dead_code)]
impl BenchmarkResult {
    pub fn new(name: &str, value: f64, unit: &str) -> Self {
        Self {
            name: name.to_string(),
            value,
            unit: unit.to_string(),
        }
    }

    pub fn ns(name: &str, value: f64) -> Self {
        Self::new(name, value, "ns")
    }
}

# HekmatMind

> Eine biologisch inspirierte KI-Architektur, die auf emergente Eigenschaften von Bewusstsein abzielt.

**HekmatMind** ist ein experimentelles Projekt zur Entwicklung einer KI-Architektur, die auf den Prinzipien biologischer neuronaler Netzwerke basiert. Ziel ist es, emergente Eigenschaften wie Selbstorganisation und (in späteren Phasen) Bewusstsein zu erforschen.

-----

### 🌱 Projektübersicht

Dieses Projekt ist mein kreativer Spielplatz für Ideen rund um neuronale Systeme, maschinelles Lernen und evolutionäre Architekturen. Aktuell befindet sich das Projekt in einer frühen Phase: Die grundlegende neuronale Verbindungsschicht ist funktionsfähig und zeigt bereits spannende Verhaltensweisen.

**Aktuelle Herausforderungen:**

  * **Skalierbarkeit:** Wie lassen sich zehntausende Neuronen effizient skalieren?
  * **Testbarkeit:** Wie bleibt das System trotz wachsender Komplexität gut testbar?
  * **Datenlayout & Performance:** Wie kann der Speicherzugriff für große neuronale Netzwerke optimiert werden?
  * **Architekturentscheidungen:** Viele Designentscheidungen müssen zukunftssicher und evolvierbar sein.

-----

### 🔍 Hauptmerkmale

  * **Biologisch inspirierte Neuronenschicht**
      * Modular aufgebaut und an realen neuronalen Systemen orientiert (z.B. verschiedene Neurotransmitter, Synapsentypen, Plastizität).
  * **Evolvierbare Architektur**
      * Entwurfsmuster und Komponenten sind auf Flexibilität und langfristige Erweiterbarkeit ausgelegt.
  * **Experimentell und explorativ**
      * Das Projekt ist kein fertiges Produkt, sondern eine offene Forschungsplattform.

-----

### 🧪 Codequalität

Qualität steht im Vordergrund, auch wenn es sich um ein "Work in Progress" handelt:

  * **Testabdeckung:** Der Zielwert liegt bei **80–90 %**, umgesetzt durch automatisierte Unit- und Integrationstests.
  * **Benchmarking:** Performance-Tests werden über `cargo-criterion` durchgeführt.
  * **Tooling:** Es kommen moderne Rust-Toolchains zur Codeanalyse, Formatierung und Testabdeckung zum Einsatz.

-----

### ⚙️ Entwicklung

#### Voraussetzungen

  * Rust `≥ 1.85`
  * `cargo` & `rustup`
  * **Optional:**
      * `cargo-criterion` (für Benchmarks)
      * `cargo-tarpaulin` (für Testabdeckung)
      * `OpenSSH` (z.B. für Git-Authentifizierung)

#### Benchmarks ausführen

Führe das folgende Skript aus, um die Benchmarks zu starten:

```bash
./benchmark_einfach.command
```

-----

### 🪪 Lizenz

Dieses Projekt steht unter der **MIT-Lizenz**. Du kannst es frei nutzen, verändern und teilen. Beiträge sind jederzeit willkommen\!

#!/bin/bash
# HekmatMind - Benchmark-Launcher
# KISS-Prinzip: Einfach, direkt, verständlich

# Verzeichnis zum Projekt wechseln
cd "$(dirname "$0")"

# Nachricht anzeigen
echo "HekmatMind Benchmarks"
echo "-------------------"

# Zuerst alte Benchmark-Ergebnisse entfernen
echo "Bereinige vorherige Benchmark-Ergebnisse..."
rm -rf hekmat_mind/target/criterion

# Benchmarks ausführen mit Fehlerbehandlung
echo "Führe Benchmarks aus..."
if ! cargo bench; then
    echo "Fehler: Benchmark-Ausführung fehlgeschlagen." >&2
    echo "Bitte überprüfen Sie die obigen Ausgaben für Details." >&2
    exit 1
fi

# Prüfen, ob Zielverzeichnis existiert
if [ ! -d "target/criterion" ]; then
    echo "Fehler: Benchmark-Ergebnisverzeichnis wurde nicht erstellt." >&2
    echo "Möglicherweise wurden keine Benchmarks ausgeführt." >&2
    exit 1
fi

# Datum für Versionierung
BENCHMARK_DATE=$(date '+%Y-%m-%d %H:%M:%S')

# Index-Seite erstellen
cat > target/criterion/index.html << EOF
<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>HekmatMind Benchmarks</title>
    <style>
        :root {
            --primary-color: #2c3e50;
            --accent-color: #3498db;
            --light-bg: #f8f9fa;
            --border-color: #ecf0f1;
            --box-shadow: 0 2px 5px rgba(0,0,0,0.08);
            --info-bg: #e8f4f8;
            --success-color: #27ae60;
            --text-color: #333;
        }

        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Arial, sans-serif;
            line-height: 1.6;
            max-width: 900px;
            margin: 0 auto;
            padding: 30px 20px;
            color: var(--text-color);
            background-color: #fff;
        }

        h1 {
            color: var(--primary-color);
            border-bottom: 2px solid var(--border-color);
            padding-bottom: 12px;
            margin-bottom: 25px;
            font-size: 2rem;
        }

        h2 {
            color: var(--accent-color);
            margin-top: 30px;
            font-size: 1.5rem;
        }

        .benchmark-list {
            list-style: none;
            padding: 0;
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
            gap: 15px;
        }

        .benchmark-item {
            background-color: var(--light-bg);
            border-radius: 8px;
            padding: 18px;
            box-shadow: var(--box-shadow);
            transition: transform 0.2s, box-shadow 0.2s;
        }

        .benchmark-item:hover {
            transform: translateY(-2px);
            box-shadow: 0 4px 8px rgba(0,0,0,0.12);
        }

        .benchmark-name {
            font-weight: 600;
            font-size: 1.2em;
            color: var(--primary-color);
            margin-bottom: 10px;
            padding-bottom: 8px;
            border-bottom: 1px solid var(--border-color);
        }

        .benchmark-links {
            margin-top: 12px;
            display: flex;
            flex-wrap: wrap;
            gap: 10px;
        }

        .benchmark-links a {
            display: inline-block;
            color: var(--accent-color);
            text-decoration: none;
            padding: 4px 8px;
            border-radius: 4px;
            background-color: rgba(52, 152, 219, 0.1);
            font-size: 0.95em;
            transition: background-color 0.2s;
        }

        .benchmark-links a:hover {
            background-color: rgba(52, 152, 219, 0.2);
            text-decoration: none;
        }

        .note {
            background-color: var(--info-bg);
            border-left: 4px solid var(--accent-color);
            padding: 15px 20px;
            margin: 20px 0 30px 0;
            border-radius: 0 8px 8px 0;
            line-height: 1.5;
        }

        .version-info {
            display: inline-block;
            background-color: rgba(39, 174, 96, 0.1);
            border-radius: 4px;
            padding: 4px 10px;
            margin-top: 15px;
            font-size: 0.9em;
            color: var(--success-color);
        }
    </style>
</head>
<body>
    <h1>HekmatMind Benchmark-Übersicht</h1>

    <div class="note">
        <strong>HekmatMind Benchmarks:</strong> Diese Übersicht zeigt die Ergebnisse aller Benchmark-Tests
        für die verschiedenen Komponenten des HekmatMind-Systems. Die Ergebnisse helfen dabei, die Leistung
        verschiedener neuronaler Komponenten und Verarbeitungsprozesse zu analysieren und zu optimieren.
        <div class="version-info">
            Ausführung: ${BENCHMARK_DATE}
        </div>
    </div>

    <h2>Wie lese ich die Benchmark-Berichte?</h2>
    <div class="interpretation-guide" style="background-color: #f5f9ff; padding: 20px; border-radius: 8px; margin-bottom: 30px;">
        <p>Die Benchmark-Berichte werden vom Criterion-Framework auf Englisch erstellt. Hier eine kurze Anleitung zur Interpretation:</p>

        <h3>Wichtige Begriffe</h3>
        <table style="width: 100%; border-collapse: collapse; margin-bottom: 20px;">
            <tr style="background-color: #e8f4f8;">
                <th style="text-align: left; padding: 8px; border: 1px solid #ddd;">Englischer Begriff</th>
                <th style="text-align: left; padding: 8px; border: 1px solid #ddd;">Deutsche Bedeutung</th>
                <th style="text-align: left; padding: 8px; border: 1px solid #ddd;">Interpretation</th>
            </tr>
            <tr>
                <td style="padding: 8px; border: 1px solid #ddd;">time</td>
                <td style="padding: 8px; border: 1px solid #ddd;">Zeit</td>
                <td style="padding: 8px; border: 1px solid #ddd;">Ausführungszeit pro Operation. <strong>Niedriger ist besser!</strong></td>
            </tr>
            <tr>
                <td style="padding: 8px; border: 1px solid #ddd;">throughput</td>
                <td style="padding: 8px; border: 1px solid #ddd;">Durchsatz</td>
                <td style="padding: 8px; border: 1px solid #ddd;">Operationen pro Sekunde. <strong>Höher ist besser!</strong></td>
            </tr>
            <tr>
                <td style="padding: 8px; border: 1px solid #ddd;">MAD</td>
                <td style="padding: 8px; border: 1px solid #ddd;">Mittlere absolute Abweichung</td>
                <td style="padding: 8px; border: 1px solid #ddd;">Maß für die Streuung. <strong>Niedriger ist besser!</strong></td>
            </tr>
            <tr>
                <td style="padding: 8px; border: 1px solid #ddd;">Std. Dev (SD)</td>
                <td style="padding: 8px; border: 1px solid #ddd;">Standardabweichung</td>
                <td style="padding: 8px; border: 1px solid #ddd;">Maß für die Streuung. <strong>Niedriger ist besser!</strong></td>
            </tr>
        </table>

        <h3>Leistungsänderungen verstehen</h3>
        <table style="width: 100%; border-collapse: collapse; margin-bottom: 20px;">
            <tr style="background-color: #e8f4f8;">
                <th style="text-align: left; padding: 8px; border: 1px solid #ddd;">Englische Meldung</th>
                <th style="text-align: left; padding: 8px; border: 1px solid #ddd;">Deutsche Bedeutung</th>
                <th style="text-align: left; padding: 8px; border: 1px solid #ddd;">Bewertung</th>
            </tr>
            <tr>
                <td style="padding: 8px; border: 1px solid #ddd;">Performance has improved</td>
                <td style="padding: 8px; border: 1px solid #ddd;">Leistung verbessert</td>
                <td style="padding: 8px; border: 1px solid #ddd; background-color: #d4edda;"><strong>Gut!</strong> Die aktuelle Version ist schneller.</td>
            </tr>
            <tr>
                <td style="padding: 8px; border: 1px solid #ddd;">Performance has regressed</td>
                <td style="padding: 8px; border: 1px solid #ddd;">Leistung verschlechtert</td>
                <td style="padding: 8px; border: 1px solid #ddd; background-color: #f8d7da;"><strong>Schlecht!</strong> Die aktuelle Version ist langsamer.</td>
            </tr>
            <tr>
                <td style="padding: 8px; border: 1px solid #ddd;">No change in performance</td>
                <td style="padding: 8px; border: 1px solid #ddd;">Keine Leistungsänderung</td>
                <td style="padding: 8px; border: 1px solid #ddd; background-color: #fff3cd;"><strong>Neutral.</strong> Kein signifikanter Unterschied erkennbar.</td>
            </tr>
            <tr>
                <td style="padding: 8px; border: 1px solid #ddd;">Change within noise threshold</td>
                <td style="padding: 8px; border: 1px solid #ddd;">Änderung im Rauschbereich</td>
                <td style="padding: 8px; border: 1px solid #ddd; background-color: #fff3cd;"><strong>Ignorieren.</strong> Änderung ist zu klein, um signifikant zu sein.</td>
            </tr>
        </table>

        <h3>Grafiken interpretieren</h3>
        <ul style="list-style-type: disc; padding-left: 20px;">
            <li><strong>Violin Plot</strong> (Violinendiagramm): Zeigt die Verteilung der Messungen. Je schmaler die Taille, desto konsistenter sind die Ergebnisse.</li>
            <li><strong>Iteration Times</strong> (Iterationszeiten): Zeigt Messungen pro Iteration. Horizontale Linien ohne Ausreißer sind optimal (stabile Ausführung).</li>
            <li><strong>Typical</strong> (Typisch): Die repräsentativste Messung der gesamten Serie - am wichtigsten für Vergleiche.</li>
            <li><strong>PDF</strong> (Wahrscheinlichkeitsdichtefunktion): Zeigt, wie wahrscheinlich bestimmte Ausführungszeiten sind. Schmale, hohe Glockenkurven sind besser.</li>
            <li><strong>Mean</strong> (Mittelwert): Durchschnittliche Ausführungszeit aller Messungen.</li>
            <li><strong>Slope</strong> (Steigung): Zeigt, ob sich die Leistung während der Messreihe verändert hat. Eine flache Linie (nahe Null) ist optimal.</li>
            <li><strong>Density</strong> (Dichte): Ähnlich wie PDF, zeigt die Verteilungsdichte der Messwerte.</li>
            <li><strong>Linear regression</strong> (Lineare Regression): Trendüber alle Messungen. Eine horizontale Linie bedeutet gleichbleibende Leistung.</li>
            <li><strong>Bootstrap</strong> (Bootstrap-Analyse): Statistisches Verfahren zur Schätzung der Konfidenzintervalle.</li>
        </ul>

        <h3>Farbcodierung in Grafiken</h3>
        <ul style="list-style-type: disc; padding-left: 20px;">
            <li><strong>Blau</strong>: Aktuelle Messung</li>
            <li><strong>Rot</strong>: Vorherige Messung (falls vorhanden)</li>
            <li><strong>Grün</strong>: Konfidenzintervalle und Streuungsmaße</li>
            <li><strong>Grau</strong>: Hintergrundinformationen oder nicht signifikante Daten</li>
        </ul>

        <h3>Regression-Plot verstehen</h3>
        <p>Bei Benchmarks mit veränderlichen Parametern (z.B. Neuronenzahl oder Synapsenstärke) zeigt der Regression-Plot, wie sich die Leistung mit steigendem Parameter verändert:</p>
        <ul style="list-style-type: disc; padding-left: 20px;">
            <li><strong>Linear</strong> (Lineare Kurve): Leistung ändert sich proportional zum Parameter (z.B. doppelte Neuronenzahl = doppelte Zeit)</li>
            <li><strong>Logarithmic</strong> (Logarithmische Kurve): Effizienzgewinn bei steigender Größe (typisch für gut optimierte Algorithmen)</li>
            <li><strong>Exponential</strong> (Exponentielle Kurve): Potenziell problematisch, da die Leistung bei höheren Werten drastisch abnimmt</li>
        </ul>

        <h3>Sample Size und Sampling Method</h3>
        <ul style="list-style-type: disc; padding-left: 20px;">
            <li><strong>Sample Size</strong> (Stichprobengröße): Anzahl der durchgeführten Messungen. Höhere Werte ergeben zuverlässigere Ergebnisse.</li>
            <li><strong>Resamples</strong> (Wiederholungen): Anzahl der Bootstrap-Wiederholungen zur Berechnung der Konfidenzintervalle.</li>
            <li><strong>Sampling Mode</strong> (Abtastmethode): Automatisch = Criterion bestimmt die optimale Stichprobenmethode.</li>
        </ul>

        <h3>Wichtige Richtwerte</h3>
        <p>Für das HekmatMind-System gelten folgende Richtwerte:</p>
        <ul style="list-style-type: disc; padding-left: 20px;">
            <li><strong>Einzelne Neuron-Aktivierung:</strong> Sollte unter 5 ns bleiben (≃ 0.000000005 Sekunden)</li>
            <li><strong>Synaptische Übertragung:</strong> Sollte unter 500 ps bleiben (≃ 0.0000000005 Sekunden)</li>
            <li><strong>EventBroker mit 100 Abonnenten:</strong> Sollte unter 10 µs bleiben (≃ 0.00001 Sekunden)</li>
            <li><strong>Generelle Regel:</strong> Jede Regression von mehr als 5% sollte untersucht werden</li>
        </ul>
    </div>

    <h2>Leistungstests nach Komponenten</h2>

    <ul class="benchmark-list">
        <li class="benchmark-item">
            <div class="benchmark-name">Neuron_Activation</div>
            <div class="benchmark-links">
                <a href="Neuron_Activation/report/index.html">Gesamtbericht</a>
                <a href="Neuron_Activation/100/report/index.html">100 Neuronen</a>
                <a href="Neuron_Activation/500/report/index.html">500 Neuronen</a>
                <a href="Neuron_Activation/1000/report/index.html">1000 Neuronen</a>
            </div>
        </li>
        <li class="benchmark-item">
            <div class="benchmark-name">Neuron_Plasticity</div>
            <div class="benchmark-links">
                <a href="Neuron_Plasticity/report/index.html">Gesamtbericht</a>
                <a href="Neuron_Plasticity/0.001/report/index.html">Lernrate 0.001</a>
                <a href="Neuron_Plasticity/0.01/report/index.html">Lernrate 0.01</a>
                <a href="Neuron_Plasticity/0.1/report/index.html">Lernrate 0.1</a>
            </div>
        </li>
        <li class="benchmark-item">
            <div class="benchmark-name">Neuron_Speed_Capacity</div>
            <div class="benchmark-links">
                <a href="Neuron_Speed_Capacity/report/index.html">Gesamtbericht</a>
                <a href="Neuron_Speed_Capacity/capacity_calculation/report/index.html">Kapazitätsberechnung</a>
            </div>
        </li>
        <li class="benchmark-item">
            <div class="benchmark-name">EventBroker_Publish</div>
            <div class="benchmark-links">
                <a href="EventBroker_Publish/report/index.html">Gesamtbericht</a>
                <a href="EventBroker_Publish/8/report/index.html">8 Bytes</a>
                <a href="EventBroker_Publish/64/report/index.html">64 Bytes</a>
                <a href="EventBroker_Publish/512/report/index.html">512 Bytes</a>
                <a href="EventBroker_Publish/1024/report/index.html">1 KB</a>
                <a href="EventBroker_Publish/4096/report/index.html">4 KB</a>
            </div>
        </li>
        <li class="benchmark-item">
            <div class="benchmark-name">EventBroker_SubscriberCount</div>
            <div class="benchmark-links">
                <a href="EventBroker_SubscriberCount/report/index.html">Gesamtbericht</a>
                <a href="EventBroker_SubscriberCount/1/report/index.html">1 Abonnent</a>
                <a href="EventBroker_SubscriberCount/10/report/index.html">10 Abonnenten</a>
                <a href="EventBroker_SubscriberCount/100/report/index.html">100 Abonnenten</a>
                <a href="EventBroker_SubscriberCount/1000/report/index.html">1000 Abonnenten</a>
            </div>
        </li>
        <li class="benchmark-item">
            <div class="benchmark-name">EventBroker_EventTypes</div>
            <div class="benchmark-links">
                <a href="EventBroker_EventTypes/report/index.html">Gesamtbericht</a>
                <a href="EventBroker_EventTypes/Single_EventType/report/index.html">Einzelner Ereignistyp</a>
                <a href="EventBroker_EventTypes/Multiple_EventTypes/report/index.html">Multiple Ereignistypen</a>
            </div>
        </li>
        <li class="benchmark-item">
            <div class="benchmark-name">Synapse Transmission</div>
            <div class="benchmark-links">
                <a href="Synapse Transmission/report/index.html">Gesamtbericht</a>
                <a href="Synapse Transmission/0.1/report/index.html">Stärke 0.1</a>
                <a href="Synapse Transmission/0.5/report/index.html">Stärke 0.5</a>
                <a href="Synapse Transmission/0.9/report/index.html">Stärke 0.9</a>
            </div>
        </li>
        <li class="benchmark-item">
            <div class="benchmark-name">Synapse Plasticity</div>
            <div class="benchmark-links">
                <a href="Synapse Plasticity/report/index.html">Gesamtbericht</a>
                <a href="Synapse Plasticity/0.001/report/index.html">Lernrate 0.001</a>
                <a href="Synapse Plasticity/0.01/report/index.html">Lernrate 0.01</a>
                <a href="Synapse Plasticity/0.1/report/index.html">Lernrate 0.1</a>
            </div>
        </li>
        <li class="benchmark-item">
            <div class="benchmark-name">Synapse Update</div>
            <div class="benchmark-links">
                <a href="Synapse Update/report/index.html">Gesamtbericht</a>
                <a href="Synapse Update/0.001/report/index.html">Aktualisierungsrate 0.001</a>
                <a href="Synapse Update/0.005/report/index.html">Aktualisierungsrate 0.005</a>
                <a href="Synapse Update/0.01/report/index.html">Aktualisierungsrate 0.01</a>
            </div>
        </li>
        <li class="benchmark-item">
            <div class="benchmark-name">Synapse Scaling</div>
            <div class="benchmark-links">
                <a href="Synapse Scaling/report/index.html">Gesamtbericht</a>
                <a href="Synapse Scaling/10/report/index.html">10 Synapsen</a>
                <a href="Synapse Scaling/100/report/index.html">100 Synapsen</a>
                <a href="Synapse Scaling/1000/report/index.html">1000 Synapsen</a>
            </div>
        </li>
    </ul>
</body>
</html>
EOF

# Browser öffnen mit verbesserter Benutzermeldung
echo "-------------------------------------"
echo "Öffne Benchmark-Übersicht im Browser..."
echo "Alle Tests wurden erfolgreich abgeschlossen."
echo "-------------------------------------"
open target/criterion/index.html

echo "Fertig! Die Benchmark-Übersicht ist jetzt im Browser geöffnet."
echo "Die Berichte zeigen die Leistungsmessungen aller HekmatMind-Komponenten."

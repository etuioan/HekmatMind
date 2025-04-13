# Neuron_Activation Benchmark

## Komponente
Neuron

## Was wird hier gemessen?
Dieser Benchmark misst, wie schnell ein Neuron auf Eingangssignale reagiert. Er vergleicht Neuronen mit verschiedenen Geschwindigkeiten (100, 500, 1000) und misst, wie lange die Verarbeitung von zwei Eingangssignalen und ein Durchlauf des neuronalen Zyklus dauert.

## Wie liest man die Werte?
Die Werte zeigen die Zeit in Nanosekunden, die ein Neuron für einen kompletten Verarbeitungszyklus benötigt. Niedrigere Werte bedeuten schnellere Verarbeitung. Die Parameter (100, 500, 1000) repräsentieren die Neuronen-Geschwindigkeit, wobei höhere Werte schnellere Reaktionszeiten ermöglichen sollten.

## Interpretation der Criterion-Ausgabe
* **Throughput**: Je höher, desto besser (Operationen pro Sekunde)
* **Average time**: Durchschnittliche Laufzeit (niedriger ist besser)
* **Slope**: Anstieg der Regression (wie sich die Zeit mit der Eingabegröße ändert)
* **MAD, SD**: Streuungsmaße - niedrigere Werte bedeuten konsistentere Ergebnisse
* **Bootstrapped CI**: Konfidenzintervall der durchschnittlichen Laufzeit

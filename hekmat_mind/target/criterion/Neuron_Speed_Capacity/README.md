# Neuron_Speed_Capacity Benchmark

## Komponente
Neuron

## Was wird hier gemessen?
Dieser Benchmark misst die Effizienz der Kapazitätsberechnung eines Neurons. Er berechnet die Kapazität (Informationsverarbeitungsfähigkeit) für Neuronen mit unterschiedlichen Geschwindigkeiten und summiert die Ergebnisse. Dies testet, wie schnell das System die Kapazität vieler Neuronen berechnen kann.

## Wie liest man die Werte?
Die Werte zeigen die Gesamtzeit in Nanosekunden, die benötigt wird, um die Kapazität aller Neuronen im Geschwindigkeitsbereich zu berechnen und zu summieren. Ein niedrigerer Wert bedeutet, dass das System neuronale Eigenschaften effizienter berechnen kann.

## Interpretation der Criterion-Ausgabe
* **Throughput**: Je höher, desto besser (Operationen pro Sekunde)
* **Average time**: Durchschnittliche Laufzeit (niedriger ist besser)
* **Slope**: Anstieg der Regression (wie sich die Zeit mit der Eingabegröße ändert)
* **MAD, SD**: Streuungsmaße - niedrigere Werte bedeuten konsistentere Ergebnisse
* **Bootstrapped CI**: Konfidenzintervall der durchschnittlichen Laufzeit

# Neuron_Plasticity Benchmark

## Komponente
Neuron

## Was wird hier gemessen?
Dieser Benchmark misst die Anpassungsfähigkeit (Plastizität) eines Neurons. Er testet, wie schnell ein Neuron seine Schwellenwerte anpassen kann, wobei verschiedene Plastizitätsraten (0.001, 0.01, 0.1) verwendet werden. Eine höhere Plastizitätsrate bedeutet schnellere Anpassung an neue Bedingungen.

## Wie liest man die Werte?
Die Werte zeigen die Zeit in Nanosekunden, die für 100 aufeinanderfolgende Schwellenwertanpassungen benötigt wird. Die Parameter (0.001, 0.01, 0.1) sind die Plastizitätsraten - höhere Werte sollten zu schnelleren Anpassungen führen, könnten aber instabiler sein.

## Interpretation der Criterion-Ausgabe
* **Throughput**: Je höher, desto besser (Operationen pro Sekunde)
* **Average time**: Durchschnittliche Laufzeit (niedriger ist besser)
* **Slope**: Anstieg der Regression (wie sich die Zeit mit der Eingabegröße ändert)
* **MAD, SD**: Streuungsmaße - niedrigere Werte bedeuten konsistentere Ergebnisse
* **Bootstrapped CI**: Konfidenzintervall der durchschnittlichen Laufzeit

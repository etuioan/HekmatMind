# EventBroker_Publish Benchmark

## Komponente
EventBroker

## Was wird hier gemessen?
Dieser Benchmark misst die Effizienz des EventBrokers beim Veröffentlichen von Ereignissen verschiedener Größen. Er testet, wie schnell der EventBroker Ereignisse mit Dateigrößen von 8 bis 4096 Bytes verarbeiten kann.

## Wie liest man die Werte?
Die Werte zeigen die Zeit in Nanosekunden, die zum Veröffentlichen eines Ereignisses benötigt wird. Kleinere Werte bedeuten schnellere Verarbeitung. Die Parameter (8, 64, 512, 1024, 4096) stellen die Größe des Ereignisses in Bytes dar – größere Ereignisse erfordern in der Regel mehr Verarbeitungszeit.

## Interpretation der Criterion-Ausgabe
* **Throughput**: Je höher, desto besser (Operationen pro Sekunde)
* **Average time**: Durchschnittliche Laufzeit (niedriger ist besser)
* **Slope**: Anstieg der Regression (wie sich die Zeit mit der Eingabegröße ändert)
* **MAD, SD**: Streuungsmaße - niedrigere Werte bedeuten konsistentere Ergebnisse
* **Bootstrapped CI**: Konfidenzintervall der durchschnittlichen Laufzeit

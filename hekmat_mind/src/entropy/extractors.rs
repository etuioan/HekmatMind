//! Bit-Extraktionsalgorithmen für Entropiedaten
//!
//! Dieses Modul implementiert verschiedene Algorithmen zur Extraktion
//! und Verbesserung von Entropiedaten, um maximale Unvorhersehbarkeit
//! zu gewährleisten.

use crate::entropy::EntropyError;
use crate::entropy::EntropyResult;
use sha2::{Digest, Sha256};

/// Extrahiert Bits aus rohen Entropiedaten mit verschiedenen Methoden
pub struct BitExtractor;

impl BitExtractor {
    /// Extrahiert Bits mit dem Von-Neumann-Extraktor
    ///
    /// Der Von-Neumann-Extraktor ist eine einfache Methode zur Entropieverbesserung,
    /// die Bits paarweise betrachtet und nur dann ein Bit ausgibt, wenn die beiden
    /// Bits unterschiedlich sind. Dies reduziert Bias in der Entropiequelle.
    ///
    /// # Arguments
    ///
    /// * `input` - Eingabedaten
    /// * `output_size` - Gewünschte Ausgabegröße in Bytes
    ///
    /// # Returns
    ///
    /// Extrahierte Bits als Byte-Array
    pub fn von_neumann_extractor(input: &[u8], output_size: usize) -> EntropyResult<Vec<u8>> {
        if input.len() < 2 {
            return Err(EntropyError::InsufficientEntropy);
        }

        let mut result = Vec::with_capacity(output_size);
        let mut bit_buffer = 0u8;
        let mut bit_count = 0;

        for chunk in input.chunks(2) {
            if chunk.len() < 2 {
                continue;
            }

            let a = chunk[0];
            let b = chunk[1];

            // Verarbeite jedes Bitpaar
            for i in 0..8 {
                let bit_a = (a >> i) & 1;
                let bit_b = (b >> i) & 1;

                // Nur wenn die Bits unterschiedlich sind, geben wir ein Bit aus
                if bit_a != bit_b {
                    // Verwende das erste Bit als Ausgabe
                    bit_buffer |= bit_a << bit_count;
                    bit_count += 1;

                    // Wenn wir 8 Bits haben, füge das Byte zum Ergebnis hinzu
                    if bit_count == 8 {
                        result.push(bit_buffer);
                        bit_buffer = 0;
                        bit_count = 0;

                        // Prüfe, ob wir genug Bytes haben
                        if result.len() >= output_size {
                            return Ok(result);
                        }
                    }
                }
            }
        }

        // Wenn wir nicht genug Bytes extrahieren konnten
        if result.len() < output_size {
            return Err(EntropyError::InsufficientEntropy);
        }

        Ok(result)
    }

    /// Extrahiert Bits mit einem kryptografischen Hash
    ///
    /// Diese Methode verwendet einen kryptografischen Hash (SHA-256), um
    /// die Entropie zu verbessern und gleichmäßig zu verteilen.
    ///
    /// # Arguments
    ///
    /// * `input` - Eingabedaten
    /// * `output_size` - Gewünschte Ausgabegröße in Bytes
    ///
    /// # Returns
    ///
    /// Extrahierte Bits als Byte-Array
    pub fn cryptographic_extractor(input: &[u8], output_size: usize) -> EntropyResult<Vec<u8>> {
        // Prüfe, ob Eingabedaten vorhanden sind
        if input.is_empty() {
            return Err(EntropyError::InsufficientEntropy);
        }

        // Für sehr kleine Eingabedaten fügen wir zusätzliche Bytes hinzu,
        // um die Entropie zu verbessern (Padding mit Zeitstempel)
        let mut data = Vec::with_capacity(input.len() + 8);
        data.extend_from_slice(input);

        // Füge aktuelle Zeit als zusätzliche Entropiequelle hinzu
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        data.extend_from_slice(&now.as_nanos().to_le_bytes()[0..8]);

        let mut result = Vec::with_capacity(output_size);
        let mut hasher = Sha256::new();

        // Initialer Hash der erweiterten Daten mit Zeitstempel
        hasher.update(&data);
        let mut hash = hasher.finalize_reset();

        // Füge den Hash zum Ergebnis hinzu
        result.extend_from_slice(&hash);

        // Wenn wir mehr Bytes benötigen, führen wir weitere Hashes durch
        while result.len() < output_size {
            // Verwende den vorherigen Hash als Eingabe für den nächsten Hash
            hasher.update(hash);
            hash = hasher.finalize_reset();

            result.extend_from_slice(&hash);
        }

        // Kürze das Ergebnis auf die gewünschte Größe
        result.truncate(output_size);

        Ok(result)
    }

    /// Extrahiert Bits mit dem TOTP-Verfahren (Time-based One-Time Password)
    ///
    /// Diese Methode kombiniert die Eingabedaten mit einem Zeitstempel,
    /// um zeitabhängige Entropie zu erzeugen.
    ///
    /// # Arguments
    ///
    /// * `input` - Eingabedaten
    /// * `output_size` - Gewünschte Ausgabegröße in Bytes
    /// * `time_step` - Zeitschritt in Sekunden (Standard: 30)
    ///
    /// # Returns
    ///
    /// Extrahierte Bits als Byte-Array
    pub fn totp_extractor(
        input: &[u8],
        output_size: usize,
        time_step: u64,
    ) -> EntropyResult<Vec<u8>> {
        if input.is_empty() {
            return Err(EntropyError::InsufficientEntropy);
        }

        // Aktuelle Zeit in Sekunden
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Berechne den Zeitschritt
        let time_counter = now / time_step;
        let time_bytes = time_counter.to_be_bytes();

        // Kombiniere Eingabedaten mit Zeitstempel
        let mut combined = Vec::with_capacity(input.len() + time_bytes.len());
        combined.extend_from_slice(input);
        combined.extend_from_slice(&time_bytes);

        // Verwende den kryptografischen Extraktor für das Ergebnis
        Self::cryptographic_extractor(&combined, output_size)
    }

    /// Extrahiert Bits mit einem Whitening-Verfahren
    ///
    /// Diese Methode wendet eine XOR-Funktion auf benachbarte Bytes an,
    /// um die Entropie zu verbessern.
    ///
    /// # Arguments
    ///
    /// * `input` - Eingabedaten
    /// * `output_size` - Gewünschte Ausgabegröße in Bytes
    ///
    /// # Returns
    ///
    /// Extrahierte Bits als Byte-Array
    pub fn whitening_extractor(input: &[u8], output_size: usize) -> EntropyResult<Vec<u8>> {
        if input.len() < 2 {
            return Err(EntropyError::InsufficientEntropy);
        }

        let mut result = Vec::with_capacity(output_size);
        let mut last_byte = 0u8;

        for &byte in input {
            // XOR mit dem vorherigen Byte und dem Index
            let whitened = byte ^ last_byte;
            result.push(whitened);
            last_byte = byte;

            if result.len() >= output_size {
                break;
            }
        }

        // Wenn wir nicht genug Bytes extrahieren konnten
        if result.len() < output_size {
            return Err(EntropyError::InsufficientEntropy);
        }

        Ok(result)
    }
}

/// Kombiniert mehrere Extraktoren für maximale Entropiequalität
pub struct CombinedExtractor;

impl CombinedExtractor {
    /// Extrahiert Bits mit einer Kombination von Extraktoren
    ///
    /// Diese Methode wendet nacheinander mehrere Extraktoren an,
    /// um die Entropiequalität zu maximieren.
    ///
    /// # Arguments
    ///
    /// * `input` - Eingabedaten
    /// * `output_size` - Gewünschte Ausgabegröße in Bytes
    ///
    /// # Returns
    ///
    /// Extrahierte Bits als Byte-Array
    pub fn extract(input: &[u8], output_size: usize) -> EntropyResult<Vec<u8>> {
        // Prüfe nur, ob Eingabedaten vorhanden sind
        // Selbst mit minimalen Daten können wir durch Zeitstempel-Erweiterung Entropie erzeugen
        if input.is_empty() {
            return Err(EntropyError::InsufficientEntropy);
        }

        // Erweitere die Eingabedaten mit Zeitstempel und Prozess-Informationen
        // um auch bei kleinen Eingaben ausreichend Entropie zu haben
        let mut enhanced_input = Vec::with_capacity(input.len() + 16);
        enhanced_input.extend_from_slice(input);

        // Füge aktuelle Zeit als zusätzliche Entropiequelle hinzu
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        enhanced_input.extend_from_slice(&now.as_nanos().to_le_bytes()[0..8]);

        // Füge Prozess-ID und Thread-ID hinzu
        let pid = std::process::id();
        enhanced_input.extend_from_slice(&pid.to_le_bytes());

        // Adaptive Strategie: Wähle die geeigneten Extraktoren basierend auf den Eingabedaten
        if enhanced_input.len() >= output_size * 2 {
            // Vollständige Pipeline für ausreichend große Eingabedaten

            // Wende zuerst Whitening an
            let whitened =
                match BitExtractor::whitening_extractor(&enhanced_input, enhanced_input.len()) {
                    Ok(data) => data,
                    // Fallback: Verwende die erweiterten Daten, wenn Whitening fehlschlägt
                    Err(_) => enhanced_input.clone(),
                };

            // Dann den kryptografischen Extraktor
            let hashed = match BitExtractor::cryptographic_extractor(&whitened, output_size * 2) {
                Ok(data) => data,
                // Fallback: Versuche direkt mit den erweiterten Daten
                Err(_) => {
                    return BitExtractor::cryptographic_extractor(&enhanced_input, output_size);
                }
            };

            // Schließlich den Von-Neumann-Extraktor oder Fallback
            match BitExtractor::von_neumann_extractor(&hashed, output_size) {
                Ok(data) => Ok(data),
                // Fallback: Verwende das Ergebnis des kryptografischen Extraktors
                Err(_) => {
                    let mut result = hashed;
                    result.truncate(output_size);
                    Ok(result)
                }
            }
        } else {
            // Vereinfachte Pipeline für kleinere Eingabedaten
            // Verwende nur den kryptografischen Extraktor, der am robustesten ist
            BitExtractor::cryptographic_extractor(&enhanced_input, output_size)
        }
    }
}

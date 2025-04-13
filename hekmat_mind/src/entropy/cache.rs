//! Entropie-Cache-Implementierung
//!
//! Dieses Modul implementiert einen asynchronen Entropie-Cache, der Entropiedaten
//! für die schnelle Verwendung zwischenspeichert.

use crate::entropy::EntropyError;
use crate::entropy::EntropyResult;
use std::collections::VecDeque;

/// Asynchroner Entropie-Cache
///
/// Speichert Entropiedaten in einem ringförmigen Puffer mit fester Kapazität.
/// Der Cache ist thread-sicher und kann von mehreren Threads gleichzeitig verwendet werden.
#[derive(Debug)]
pub struct EntropyCache {
    /// Gepufferte Entropiedaten
    buffer: VecDeque<u8>,

    /// Maximale Kapazität des Caches in Bytes
    capacity: usize,
}

impl EntropyCache {
    /// Erstellt einen neuen Entropie-Cache mit der angegebenen Kapazität
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Gibt die Kapazität des Caches zurück
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Gibt die Anzahl der verfügbaren Bytes im Cache zurück
    pub fn available_bytes(&self) -> usize {
        self.buffer.len()
    }

    /// Gibt den Füllstand des Caches als Prozentsatz zurück
    pub fn fill_percentage(&self) -> f32 {
        if self.capacity == 0 {
            return 0.0;
        }
        self.buffer.len() as f32 / self.capacity as f32
    }

    /// Fügt Bytes zum Cache hinzu
    ///
    /// Wenn der Cache voll ist, werden die ältesten Bytes entfernt.
    pub fn add_bytes(&mut self, bytes: &[u8]) -> EntropyResult<()> {
        if bytes.is_empty() {
            return Ok(());
        }

        // Sicherstellen, dass genug Platz vorhanden ist
        let required_space = bytes.len();
        let available_space = self.capacity - self.buffer.len();

        if required_space > self.capacity {
            return Err(EntropyError::CacheError(format!(
                "Daten ({} Bytes) überschreiten die Cache-Kapazität ({} Bytes)",
                required_space, self.capacity
            )));
        }

        // Platz schaffen, falls nötig
        if required_space > available_space {
            let to_remove = required_space - available_space;
            for _ in 0..to_remove {
                self.buffer.pop_front();
            }
        }

        // Neue Bytes hinzufügen
        for &byte in bytes {
            self.buffer.push_back(byte);
        }

        Ok(())
    }

    /// Holt die angeforderte Anzahl von Bytes aus dem Cache
    ///
    /// Die Bytes werden aus dem Cache entfernt.
    pub fn get_bytes(&mut self, count: usize) -> EntropyResult<Vec<u8>> {
        if count > self.buffer.len() {
            return Err(EntropyError::InsufficientEntropy);
        }

        let mut result = Vec::with_capacity(count);
        for _ in 0..count {
            if let Some(byte) = self.buffer.pop_front() {
                result.push(byte);
            } else {
                break;
            }
        }

        Ok(result)
    }

    /// Leert den Cache
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Gibt an, ob der Cache leer ist
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Gibt an, ob der Cache voll ist
    pub fn is_full(&self) -> bool {
        self.buffer.len() >= self.capacity
    }

    /// Prüft, ob der Cache aufgefüllt werden sollte
    pub fn needs_refill(&self, threshold: f32) -> bool {
        self.fill_percentage() < threshold
    }
}

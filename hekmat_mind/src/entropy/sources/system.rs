//! Systemrauschen-Entropiequelle
//!
//! Implementiert eine Entropiequelle, die Systemrauschen als Fallback-Mechanismus
//! für die Entropiegewinnung verwendet. Diese Quelle nutzt verschiedene
//! systeminterne Quellen wie Zeitstempel, Prozess-IDs, Speicherauslastung usw.

use crate::entropy::sources::priority;
use crate::entropy::{EntropyResult, EntropySource};
use async_trait::async_trait;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

/// Systemrauschen-Entropiequelle
pub struct SystemNoiseSource {
    /// Name der Quelle
    name: String,
}

impl SystemNoiseSource {
    /// Erstellt eine neue Systemrauschen-Entropiequelle
    pub fn new() -> Self {
        Self {
            name: "Systemrauschen".to_string(),
        }
    }

    /// Sammelt Systemrauschen aus verschiedenen Quellen
    fn collect_system_noise(&self, bytes_requested: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(bytes_requested);

        // Sammle initiale Entropie für den Seed
        let mut seed_data = [0u8; 32];

        // Aktuelle Zeit mit Nanosekunden-Präzision
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();

        let nanos = now.subsec_nanos();
        let secs = now.as_secs();

        // Prozess-ID
        let pid = process::id();

        // Fülle den Seed mit verschiedenen Entropiequellen
        for (i, byte) in seed_data.iter_mut().enumerate().take(8) {
            *byte = ((secs >> (i * 8)) & 0xFF) as u8;
        }

        for (i, byte) in seed_data.iter_mut().enumerate().skip(8).take(4) {
            *byte = ((nanos >> ((i - 8) * 8)) & 0xFF) as u8;
        }

        for (i, byte) in seed_data.iter_mut().enumerate().skip(12).take(4) {
            *byte = ((pid >> ((i - 12) * 8)) & 0xFF) as u8;
        }

        // Fülle den Rest mit zusätzlichen Rauschquellen
        self.add_additional_noise(&mut seed_data[16..]);

        // Erstelle einen PRNG mit dem gesammelten Seed
        let mut rng = StdRng::from_seed(seed_data);

        // Generiere die angeforderten Bytes
        for _ in 0..bytes_requested {
            let random_byte = rng.gen_range(0..=255) as u8;
            result.push(random_byte);
        }

        result
    }

    /// Fügt zusätzliches Rauschen zum Seed hinzu
    fn add_additional_noise(&self, buffer: &mut [u8]) {
        if buffer.is_empty() {
            return;
        }

        // Verwende Speicheradressen als Entropiequelle
        let ptr_value = buffer.as_ptr() as usize;
        for i in 0..std::cmp::min(buffer.len(), 8) {
            buffer[i] = ((ptr_value >> (i * 8)) & 0xFF) as u8;
        }

        // Verwende Thread-ID als weitere Entropiequelle
        if let Some(thread_id) = self.get_thread_id() {
            let start = std::cmp::min(buffer.len(), 8);
            let end = std::cmp::min(buffer.len(), 16);

            for (i, byte) in buffer.iter_mut().enumerate().skip(start).take(end - start) {
                *byte = ((thread_id >> ((i - start) * 8)) & 0xFF) as u8;
            }
        }

        // Verwende CPU-Zeit als weitere Entropiequelle
        if let Ok(cpu_time) = self.get_cpu_time() {
            let start = std::cmp::min(buffer.len(), 16);
            let end = std::cmp::min(buffer.len(), 24);

            for (i, byte) in buffer.iter_mut().enumerate().skip(start).take(end - start) {
                *byte = ((cpu_time >> ((i - start) * 8)) & 0xFF) as u8;
            }
        }

        // Fülle den Rest mit XOR-Operationen auf
        for i in 24..buffer.len() {
            buffer[i] = buffer[i % 8] ^ buffer[8 + (i % 8)] ^ buffer[16 + (i % 8)];
        }
    }

    /// Versucht, die aktuelle Thread-ID zu erhalten
    fn get_thread_id(&self) -> Option<u64> {
        // Dies ist plattformabhängig und nicht überall verfügbar
        #[cfg(target_os = "linux")]
        {
            use libc::pthread_self;

            unsafe {
                let id = pthread_self();
                Some(id as u64)
            }
        }

        // Fallback für nicht-Linux-Plattformen: Verwende die Adresse eines Stack-Objekts als Näherung
        #[cfg(not(target_os = "linux"))]
        {
            let local_var = 0u8;
            Some(&local_var as *const u8 as u64)
        }
    }

    /// Versucht, die CPU-Zeit des aktuellen Prozesses zu erhalten
    fn get_cpu_time(&self) -> Result<u64, ()> {
        #[cfg(target_os = "linux")]
        {
            use std::fs::File;
            use std::io::Read;

            if let Ok(mut file) = File::open("/proc/self/stat") {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    let fields: Vec<&str> = contents.split_whitespace().collect();
                    if fields.len() > 14 {
                        if let Ok(utime) = fields[13].parse::<u64>() {
                            if let Ok(stime) = fields[14].parse::<u64>() {
                                return Ok(utime + stime);
                            }
                        }
                    }
                }
            }
        }

        // Fallback: Verwende die aktuelle Zeit
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();

        Ok(now.as_secs() ^ (now.subsec_nanos() as u64))
    }
}

impl Default for SystemNoiseSource {
    /// Implementiert den Default-Trait für SystemNoiseSource
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EntropySource for SystemNoiseSource {
    fn name(&self) -> &str {
        &self.name
    }

    fn priority(&self) -> u8 {
        priority::TERTIARY
    }

    async fn is_available(&self) -> bool {
        // Systemrauschen ist immer verfügbar
        true
    }

    async fn collect_entropy(&self, bytes_requested: usize) -> EntropyResult<Vec<u8>> {
        Ok(self.collect_system_noise(bytes_requested))
    }
}

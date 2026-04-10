use std::{
    collections::HashMap,
    fmt::Debug,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use tokio::{
    fs::{self, OpenOptions},
    io::{AsyncReadExt as _, AsyncWriteExt as _},
};

use crate::infrastructure::meteogalicia::dtos::ResponseBody;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub data: ResponseBody,
    pub expires_at: u64,
}

impl CacheEntry {
    pub fn from_data(data: ResponseBody) -> Self {
        Self {
            data,
            expires_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 3600 * 24,
        }
    }

    pub fn new(data: ResponseBody, expires_at: u64) -> Self {
        Self { data, expires_at }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at
            < SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
    }
}

#[derive(Debug)]
pub struct Cache {
    entries: HashMap<String, CacheEntry>,
}

impl Cache {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn builder(capacity: usize) -> FileCacheBuilder {
        FileCacheBuilder::new(capacity)
    }

    pub fn get(&mut self, key: &str) -> Option<CacheEntry> {
        let cache_entry = self.entries.get(key)?.clone();

        if cache_entry.is_expired() {
            self.entries.remove(key);
            return None;
        }

        Some(cache_entry)
    }

    pub fn set(&mut self, key: &str, entry: CacheEntry) {
        self.entries.insert(key.to_string(), entry);
    }

    pub fn entries(&self) -> HashMap<String, CacheEntry> {
        self.entries.clone()
    }

    pub async fn save_entries(
        entries: HashMap<String, CacheEntry>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&entries)?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&cache_file())
            .await?;

        file.write_all(json.as_bytes()).await?;

        Ok(())
    }
}

pub struct FileCacheBuilder {
    capacity: usize,
}

impl FileCacheBuilder {
    pub fn new(capacity: usize) -> Self {
        Self { capacity }
    }

    pub async fn build(self) -> Result<Cache, Box<dyn std::error::Error>> {
        let mut cache = Cache::new();

        let path = cache_file();
        ensure_file(&path).await?;

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .await?;

        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        let entries: HashMap<String, CacheEntry> = if contents.trim().is_empty() {
            HashMap::new()
        } else {
            serde_json::from_str(&contents).map_err(|e| {
                log::error!("Error deserializing cache entries: {:?}", e);
                e.to_string()
            })?
        };
        cache.entries = entries
            .iter()
            .filter(|(_, entry)| !entry.is_expired())
            .map(|(key, entry)| (key.clone(), entry.clone()))
            .collect();

        cache.entries.reserve(self.capacity);

        Ok(cache)
    }
}

fn cache_file() -> PathBuf {
    dirs::cache_dir()
        .unwrap()
        .join("forecast-cli")
        .join("cache.json")
}

async fn ensure_file(path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }

    if !path.exists() {
        fs::File::create(path).await?;
    }

    Ok(())
}

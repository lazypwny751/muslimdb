use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write, Read};

#[derive(Debug)]
pub struct KurtlarVadisi {
    pub data: HashMap<String, String>,
}

impl KurtlarVadisi {
    pub fn new() -> Self {
        KurtlarVadisi {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    pub fn save(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;

        for (key, value) in &self.data {
            let key_bytes = key.as_bytes();
            let value_bytes = value.as_bytes();
            file.write_all(&(key_bytes.len() as u32).to_le_bytes())?;
            file.write_all(key_bytes)?;
            file.write_all(&(value_bytes.len() as u32).to_le_bytes())?;
            file.write_all(value_bytes)?;
        }
        Ok(())
    }

    pub fn load(filename: &str) -> io::Result<Self> {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let mut data = HashMap::new();
        let mut offset = 0;

        while offset < buffer.len() {
            let key_len = u32::from_le_bytes(buffer[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;
            let key = String::from_utf8(buffer[offset..offset + key_len].to_vec()).unwrap();
            offset += key_len;

            let value_len = u32::from_le_bytes(buffer[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;
            let value = String::from_utf8(buffer[offset..offset + value_len].to_vec()).unwrap();
            offset += value_len;

            data.insert(key, value);
        }

        Ok(KurtlarVadisi { data })
    }
}
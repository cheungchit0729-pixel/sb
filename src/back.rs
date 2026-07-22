use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer};

#[derive(Debug, serde::Deserialize, Clone, serde::Serialize)]
pub struct PurchaseEntry {
    #[serde(deserialize_with = "parse_naive_datetime")]
    pub(crate) date: NaiveDateTime,
    pub(crate) amount: f64,
    pub(crate) merchant: String,
    pub(crate) category: String,
    pub(crate) notes: String,
}

#[derive(Default)]
pub struct AppStorage {
    loaded_data: Vec<PurchaseEntry>,
}

impl AppStorage {
    pub fn add(&mut self, entry: PurchaseEntry) {
        self.loaded_data.push(entry);
    }

    pub fn add_many(&mut self, entries: Vec<PurchaseEntry>) {
        self.loaded_data.extend(entries);
    }

    pub fn remove(&mut self, id: usize) {
        self.loaded_data.remove(id);
    }

    pub fn remove_many(&mut self, mut idxes: Vec<usize>) {
        idxes.sort_unstable();
        idxes.dedup();
        for i in idxes.into_iter().rev() {
            if i < self.loaded_data.len() {
                self.loaded_data.remove(i);
            }
        }
    }

    pub fn get(&self, id: Option<usize>) -> Option<&PurchaseEntry> {
        match id {
            Some(i) => self.loaded_data.get(i),
            None => None,
        }
    }

    pub fn get_mut(&mut self, id: Option<usize>) -> Option<&mut PurchaseEntry> {
        match id {
            Some(i) => self.loaded_data.get_mut(i),
            None => None,
        }
    }


    pub fn get_all(&self) -> &[PurchaseEntry] {
        &self.loaded_data
    }
}
// this is stupid
fn parse_naive_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let s = s.trim();

    let (date_part, time_part) = s
        .split_once(' ')
        .ok_or_else(|| serde::de::Error::custom("expected 'YYYY-M-D H:M:S'"))?;

    let mut d = date_part.split('-');
    let y: i32 = d.next().ok_or_else(|| serde::de::Error::custom("missing year"))?.parse().map_err(serde::de::Error::custom)?;
    let m: u32 = d.next().ok_or_else(|| serde::de::Error::custom("missing month"))?.parse().map_err(serde::de::Error::custom)?;
    let day: u32 = d.next().ok_or_else(|| serde::de::Error::custom("missing day"))?.parse().map_err(serde::de::Error::custom)?;

    let mut t = time_part.split(':');
    let hh: u32 = t.next().ok_or_else(|| serde::de::Error::custom("missing hour"))?.parse().map_err(serde::de::Error::custom)?;
    let mm: u32 = t.next().ok_or_else(|| serde::de::Error::custom("missing minute"))?.parse().map_err(serde::de::Error::custom)?;
    let ss: u32 = t.next().ok_or_else(|| serde::de::Error::custom("missing second"))?.parse().map_err(serde::de::Error::custom)?;

    let normalized = format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", y, m, day, hh, mm, ss);
    NaiveDateTime::parse_from_str(&normalized, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| serde::de::Error::custom(format!("bad datetime: {e}")))
}

pub fn parse_data(path: Option<String>) -> Vec<PurchaseEntry> {
    let path = path.expect("missing path");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)
        .expect("cannot open csv");

    let mut entries = Vec::new();
    for result in rdr.deserialize() {
        let record: PurchaseEntry = result.expect("REASON");
        entries.push(record);
    }
    entries
}

pub fn write_data(path: Option<String>, entries: &[PurchaseEntry]) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.expect("missing path");
    let mut wtr = csv::WriterBuilder::new()
    .has_headers(true)
    .from_path(path)
    .expect("cannot open csv");

    for entry in entries {
        wtr.serialize(entry)?;
    }

    wtr.flush()?;
    Ok(())
}

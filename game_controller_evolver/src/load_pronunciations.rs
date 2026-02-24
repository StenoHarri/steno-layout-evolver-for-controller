use std::fs;
use std::collections::HashMap;

pub type ClusterMap = HashMap<String, f64>;
pub type PronunciationMap = HashMap<String, HashMap<String, f64>>;

pub fn load_initial_clusters(path: &str) -> ClusterMap {
    let data = fs::read_to_string(path).expect("Failed to read initial_clusters.json");
    serde_json::from_str(&data).expect("Failed to parse initial_clusters.json")
}

pub fn load_final_clusters(path: &str) -> ClusterMap {
    let data = fs::read_to_string(path).expect("Failed to read final_clusters.json");
    serde_json::from_str(&data).expect("Failed to parse final_clusters.json")
}

pub fn load_pronunciation_frequency(path: &str) -> PronunciationMap {
    let data = fs::read_to_string(path).expect("Failed to read pronunciation_frequency.json");
    serde_json::from_str(&data).expect("Failed to parse pronunciation_frequency.json")
}

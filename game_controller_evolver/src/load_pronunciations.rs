use std::collections::HashMap;
use std::fs;

pub type ClusterList = Vec<String>;
pub type ClusterFrequencyMap = HashMap<String, f64>;
pub type PronunciationMap = HashMap<String, HashMap<String, f64>>;

#[derive(Debug)]
pub struct PronunciationData {
    pub initial_clusters: ClusterList,
    pub final_clusters: ClusterList,
    pub common_initial_cluster_freqs: ClusterFrequencyMap,
    pub common_final_cluster_freqs: ClusterFrequencyMap,
    pub full_word_freqs: PronunciationMap,
}

fn load_json<T: serde::de::DeserializeOwned>(path: &str) -> T {
    let data = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Failed to read {}", path));

    serde_json::from_str(&data)
        .unwrap_or_else(|_| panic!("Failed to parse {}", path))
}

impl PronunciationData {
    pub fn load(manifest_dir: &str) -> Self {
        let base = format!("{}/../pronunciation_data", manifest_dir);

        Self {
            initial_clusters: load_json(&format!("{}/initial_clusters.json", base)),
            final_clusters: load_json(&format!("{}/final_clusters.json", base)),
            common_initial_cluster_freqs: load_json(&format!("{}/common_initial_clusters_frequencies.json", base)),
            common_final_cluster_freqs: load_json(&format!("{}/common_final_clusters_frequencies.json", base)),
            full_word_freqs: load_json(&format!("{}/pronunciation_frequency.json", base)),
        }
    }
}
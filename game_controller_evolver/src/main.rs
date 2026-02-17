mod load_pronunciations;

use load_pronunciations::{load_initial_clusters, load_final_clusters, load_pronunciation_frequency};

fn main() {
    println!("Hello, world!");
    
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    let initial_path = format!("{}/../pronunciation_data/initial_clusters.json", manifest_dir);
    let final_path   = format!("{}/../pronunciation_data/final_clusters.json", manifest_dir);
    let freq_path    = format!("{}/../pronunciation_data/pronunciation_frequency.json", manifest_dir);

    let initial_clusters = load_initial_clusters(&initial_path);
    let final_clusters   = load_final_clusters(&final_path);
    let pronunciation_freq = load_pronunciation_frequency(&freq_path);

    println!("Initial clusters: {:?}", &initial_clusters.keys().take(5).collect::<Vec<_>>());
    println!("Final clusters: {:?}", &final_clusters.keys().take(5).collect::<Vec<_>>());
    println!("Pronunciation freq samples: {:?}", &pronunciation_freq.iter().take(5).collect::<Vec<_>>());
}

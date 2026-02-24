mod load_pronunciations;
mod genetic_logic;

use load_pronunciations::{load_initial_clusters, load_final_clusters, load_pronunciation_frequency};

use crate::genetic_logic::seed_population::create_initial_population;
use crate::genetic_logic::evolve_population::evolve_population;

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

    let initial_population = create_initial_population(
        &initial_clusters,
        &final_clusters,
        3,
        5,
    );

    println!("Initial population: {:#?}", initial_population);

    let evolved_population = evolve_population(
        &initial_population,
        &initial_clusters,
        &final_clusters,
        100,
    );

    println!("\nEvolved population: {:#?}", evolved_population);


}

mod load_pronunciations;
mod genetic_logic;

use crate::genetic_logic::fitness_function::genes_into_chords::generate_explicit_chords;
use load_pronunciations::PronunciationData;
use std::collections::HashSet;

use crate::genetic_logic::seed_population::create_initial_population;
use crate::genetic_logic::evolve_population::evolve_population;

fn main() {
    println!("Hello, world!");

    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    let pronunciation_data = PronunciationData::load(manifest_dir);

    let combined_clusters: HashSet<String> = [
        pronunciation_data.initial_clusters.clone(),
        pronunciation_data.final_clusters.clone(),
    ]
    .concat()
    .into_iter()
    .collect();


    let initial_population = create_initial_population(
        &pronunciation_data.common_initial_cluster_freqs,
        &pronunciation_data.common_final_cluster_freqs,
        10,
        100,
    );

    println!("Initial population: {:#?}", initial_population.get(0));

    let evolved_population = evolve_population(
        &initial_population,
        &pronunciation_data.common_initial_cluster_freqs,
        &pronunciation_data.common_final_cluster_freqs,
        &combined_clusters,
        10,
        pronunciation_data.full_word_freqs,
    );


    let evolved_keyboardlayout = &evolved_population[0];
    println!("\nEvolved population: {:#?}", evolved_keyboardlayout);


    println!("\nLeft joystick: {:#?}", generate_explicit_chords(&evolved_keyboardlayout.0.left_chord_genes));
    println!("\nRight joystick: {:#?}", generate_explicit_chords(&evolved_keyboardlayout.0.right_chord_genes));


}

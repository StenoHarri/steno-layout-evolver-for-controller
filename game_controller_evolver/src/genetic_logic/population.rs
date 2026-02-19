use rand::{Rng, RngExt};
use std::collections::HashMap;
use rand::prelude::IndexedRandom;


use crate::genetic_logic::keyboard_layout::KeyboardLayout;
pub(crate) fn create_initial_population(
    initial_clusters: &HashMap<String, f64>,
    final_clusters: &HashMap<String, f64>,
    max_chords: usize,
    population_size: usize,
) -> Vec<KeyboardLayout> {
    let mut rng = rand::rng();

    let initial_keys: Vec<String> = initial_clusters.keys().cloned().collect();
    let final_keys: Vec<String> = final_clusters.keys().cloned().collect();

    let mut population = Vec::with_capacity(population_size);

    for _ in 0..population_size {
        let mut left_chords = Vec::new();
        let mut right_chords = Vec::new();

        // Keep adding chords until we reach max_chords
        while left_chords.len() < max_chords {
            let key = initial_keys.choose(&mut rng).unwrap().clone();
            let value = generate_value(&mut rng);
            left_chords.push((key, value));
        }

        while right_chords.len() < max_chords {
            let key = final_keys.choose(&mut rng).unwrap().clone();
            let value = generate_value(&mut rng);
            right_chords.push((key, value));
        }

        population.push(KeyboardLayout {
            left_chords,
            right_chords,
        });
    }

    population
}


fn generate_value<R: Rng>(rng: &mut R) -> String {
    let number = rng.random_range(1..=8).to_string();

    let suffix = match rng.random_range(0..3) {
        0 => "",
        1 => "L",
        _ => "R",
    };

    format!("{}{}", number, suffix)
}

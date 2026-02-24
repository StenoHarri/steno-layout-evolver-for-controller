use std::collections::HashMap;
use crate::genetic_logic::generating_genes::{random_consonant_cluster, random_joystick_location};


use crate::genetic_logic::keyboard_layout::KeyboardLayout;
pub(crate) fn create_initial_population(
    initial_clusters: &HashMap<String, f64>,
    final_clusters: &HashMap<String, f64>,
    max_chords: usize,
    population_size: usize,
) -> Vec<KeyboardLayout> {
    let mut rng = rand::rng();

    let mut population = Vec::with_capacity(population_size);

    for _ in 0..population_size {
        let mut left_chords = Vec::new();
        let mut right_chords = Vec::new();

        // Keep adding chords until it reaches max_chords
        while left_chords.len() < max_chords {
            let key = random_consonant_cluster(&mut rng, initial_clusters);
            let value = random_joystick_location(&mut rng);
            left_chords.push((key, value));
        }

        while right_chords.len() < max_chords {
            let key = random_consonant_cluster(&mut rng, final_clusters);
            let value = random_joystick_location(&mut rng);
            right_chords.push((key, value));
        }

        population.push(KeyboardLayout {
            left_chords,
            right_chords,
        });
    }

    population
}

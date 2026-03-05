use rand::RngExt;
use std::collections::{HashSet, HashMap};

use crate::genetic_logic::keyboard_layout::KeyboardLayout;
use crate::genetic_logic::measured_keyboard_layout::MeasuredKeyboardLayout;
use crate::genetic_logic::generating_genes::{random_consonant_cluster, random_joystick_location};

use crate::genetic_logic::fitness_function::measure_fitness::measure_layout;

pub(crate) fn evolve_population(
    initial_population: &[KeyboardLayout],
    initial_clusters: &HashMap<String, f64>,
    final_clusters: &HashMap<String, f64>,
    valid_sounds: &HashSet<String>,
    max_generations: usize,
    words_and_their_frequencies: HashMap<String, HashMap<String, f64>>,
) -> Vec<KeyboardLayout> {

    let mut population = initial_population.to_vec();

    for _ in 0..max_generations {

        println!("new generation");

        // Measure fitness (placeholder)
        // let measured_population = measure_population(population);
        let mut measured_population = population
            .par_iter()
            .map(|layout| {
                let fitness = measure_layout(layout, &words_and_their_frequencies, &valid_sounds);
                (layout.clone(), fitness)
            })
            .collect::<Vec<_>>();

        measured_population.sort_by(|a, b| 
            b.1.partial_cmp(&a.1).unwrap()
        );


        // Select top n (placeholder: keep first half)
        let survivors = population
            .iter()
            .take(population.len() / 2)
            .cloned()
            .collect::<Vec<_>>();

        // Repopulate with mutation
        let mut new_population = survivors.clone();

        while new_population.len() < population.len() {
            let mut individual = survivors[0].clone();

            mutate(&mut individual, initial_clusters, final_clusters);

            new_population.push(individual);
        }

        population = new_population;
    }

    population
}


fn mutate(
    layout: &mut KeyboardLayout,
    initial_clusters: &HashMap<String, f64>,
    final_clusters: &HashMap<String, f64>,
) {
    let mut rng = rand::rng();

    // Decide left or right side
    let mutate_left = rng.random_range(0..2) == 0;

    if mutate_left && !layout.left_chord_genes.is_empty() {
        let idx = rng.random_range(0..layout.left_chord_genes.len());
        let gene = &mut layout.left_chord_genes[idx];

        if rng.random_range(0..2) == 0 {
            // Mutate key
            gene.0 = random_consonant_cluster(&mut rng, initial_clusters);
        } else {
            // Mutate value
            gene.1 = random_joystick_location(&mut rng);
        }

    } else if !layout.right_chord_genes.is_empty() {
        let idx = rng.random_range(0..layout.right_chord_genes.len());
        let gene = &mut layout.right_chord_genes[idx];

        if rng.random_range(0..2) == 0 {
            gene.0 = random_consonant_cluster(&mut rng, final_clusters);
        } else {
            gene.1 = random_joystick_location(&mut rng);
        }
    }
}

use rayon::prelude::*;

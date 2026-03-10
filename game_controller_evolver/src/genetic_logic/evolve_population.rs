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
) -> Vec<(KeyboardLayout, f64)> {

    let mut population = initial_population.to_vec();
    let mut rng = rand::rng();

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

        println!("highest fitness: {}", measured_population[0].1);

        // Take only the fittest individual
        let fittest_layout = measured_population[0].0.clone();

        // Set the first of the new population as the old winner
        let mut new_population = vec![fittest_layout.clone()];

        // Fill the rest with mutated copies
        while new_population.len() < population.len() {
            let mut individual = fittest_layout.clone();

            // Randomly choose 1 or 2 mutations
            let mutations = rng.random_range(1..=2);

            for _ in 0..mutations{
                mutate(&mut individual, initial_clusters, final_clusters, &mut rng);
            }

            new_population.push(individual);
        }

        population = new_population;
    }

    println!("final generation");

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

    println!("highest fitness: {}", measured_population[0].1);

    measured_population

}


fn mutate(
    layout: &mut KeyboardLayout,
    initial_clusters: &HashMap<String, f64>,
    final_clusters: &HashMap<String, f64>,
    rng: &mut impl rand::Rng,
) {
    // Decide left or right side
    let mutate_left = rng.random_range(0..2) == 0;

    if mutate_left && !layout.left_chord_genes.is_empty() {
        let idx = rng.random_range(0..layout.left_chord_genes.len());
        let gene = &mut layout.left_chord_genes[idx];

        if rng.random_range(0..2) == 0 {
            // Mutate key
            gene.0 = random_consonant_cluster(rng, initial_clusters);
        } else {
            // Mutate value
            gene.1 = random_joystick_location(rng);
        }

    } else if !layout.right_chord_genes.is_empty() {
        let idx = rng.random_range(0..layout.right_chord_genes.len());
        let gene = &mut layout.right_chord_genes[idx];

        if rng.random_range(0..2) == 0 {
            gene.0 = random_consonant_cluster(rng, final_clusters);
        } else {
            gene.1 = random_joystick_location(rng);
        }
    }
}

use rayon::prelude::*;

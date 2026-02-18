use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

fn create_initial_population(
    initial_clusters: &HashMap<String, Vec<String>>,
    final_clusters: &HashMap<String, Vec<String>>,
    max_chords: usize,
    population_size: usize,
) -> Vec<Organism> {
    let mut rng = rand::thread_rng();

    let initial_keys: Vec<String> = initial_clusters.keys().cloned().collect();
    let final_keys: Vec<String> = final_clusters.keys().cloned().collect();

    let mut population = Vec::with_capacity(population_size);

    for _ in 0..population_size {
        let left_count = rng.gen_range(1..=max_chords);
        let right_count = rng.gen_range(1..=max_chords);

        let left_chords = initial_keys
            .choose_multiple(&mut rng, left_count)
            .map(|key| {
                let value = generate_value(&mut rng);
                (key.clone(), value)
            })
            .collect();

        let right_chords = final_keys
            .choose_multiple(&mut rng, right_count)
            .map(|key| {
                let value = generate_value(&mut rng);
                (key.clone(), value)
            })
            .collect();

        population.push(Organism {
            left_chords,
            right_chords,
        });
    }

    population
}


fn generate_value<R: Rng>(rng: &mut R) -> String {
    let number = rng.gen_range(1..=8).to_string();

    let suffix = match rng.gen_range(0..3) {
        0 => "",
        1 => "L",
        _ => "R",
    };

    format!("{}{}", number, suffix)
}

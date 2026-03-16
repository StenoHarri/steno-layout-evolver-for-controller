// src/genetic_logic/fitness_function/measure_fitness.rs
use std::collections::{HashSet, HashMap};
use crate::genetic_logic::keyboard_layout::KeyboardLayout;
use crate::genetic_logic::fitness_function::genes_into_chords::genes_into_chords;
use crate::genetic_logic::fitness_function::find_matches::find_matches;  

// Vowels will not be written with the joysticks, I will reuse the 4 vowel keys the stenotype has
// These can either be the 4 triggers or the 4 paddles, whichever is easiest to hit
//static VOWELS: &[&str] = &[
//    "AA","AE","AH","AO","AW","AY",
//    "EH","ER","EY","IH","IY","OW",
//    "OY","UH","UW"
//];

static VOWELS: &[&str] = &[
    "vowel"
];

pub fn measure_layout(
    layout: &KeyboardLayout,
    words_and_their_frequencies: &HashMap<String, HashMap<String, f64>>,
    valid_sounds: &HashSet<String>,
) -> (f64, f64, f64) {


    let left_chords_and_their_mappings = genes_into_chords(&layout.left_chord_genes, valid_sounds);
    let right_chords_and_their_mappings = genes_into_chords(&layout.right_chord_genes, valid_sounds);

    // println!("Left chords and their mappings: {:?}", &left_chords_and_their_mappings);

    let (coverage, collisions) = find_matches(
        &left_chords_and_their_mappings,
        &VOWELS,
        &right_chords_and_their_mappings,
        &words_and_their_frequencies,
    );

    // I think this is a good fitness score?
    let fitness = coverage - (10.0 * collisions as f64);

    (fitness, coverage, collisions)

}

use std::collections::{HashMap, HashSet};

pub fn find_matches(
    left_chords: &HashMap<(String, String), Vec<Vec<String>>>,
    vowels: &HashSet<String>,
    right_chords: &HashMap<(String, String), Vec<Vec<String>>>,
    words_and_their_frequencies: &HashMap<String, HashMap<String, f64>>,
) -> (f64, f64) {

    // Store combinations that have been used
    let mut used_combinations: HashSet<String> = HashSet::new();
    let mut collisions = 0;

    // Iterate over the left chords
    for (joystick1_location, joystick1_clusters) in left_chords {

        for joystick1_cluster in joystick1_clusters {

            // Iterate over the right chords
            for (joystick2_location, joystick2_clusters) in right_chords {

                for joystick2_cluster in joystick2_clusters {

                    // Iterate over vowels
                    for vowel in vowels {

                        // Construct the key by combining left, vowel, and right
                        let combination = format!(
                            "{} {} {}", 
                            joystick1_cluster.join(" "),
                            vowel, 
                            joystick2_cluster.join(" "),
                        );

                        //println!("a combination: {:#?}", combination);

                        // Check if the combination is in the words_and_their_frequencies
                        if let Some(word_map) = words_and_their_frequencies.get(&combination) {
                            // Found a valid match, now we need to check for collisions
                            
                            //for (word, zipf_frequency) in word_map {


                            if used_combinations.contains(&combination) {
                                // This combination has already been used, so it's a collision
                                collisions += 1;
                            } else {
                                // Record the combination as used

                                //println!("Novel combination: {:#?}", combination);

                                used_combinations.insert(combination);

                            }
                        }
                    }
                }
            }
        }
    }

    // coverage
    let coverage = (used_combinations.len() as f64) / (words_and_their_frequencies.len() as f64);

    // Return coverage and collisions
    (coverage, collisions as f64)
}
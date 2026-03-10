use std::collections::{HashMap, HashSet};

fn zipf_to_prob(zipf: f64) -> f64 {
    10f64.powf(zipf - 6.0)
}

pub fn find_matches(
    left_chords: &HashMap<(String, String), Vec<String>>,
    vowels: &[&str],
    right_chords: &HashMap<(String, String), Vec<String>>,
    words_and_their_frequencies: &HashMap<String, HashMap<String, f64>>,
) -> (f64, f64) {

    let mut already_seen_words: HashSet<String> = HashSet::new();

    let mut coverage_score = 0.0;
    let mut conflict_score = 0.0;

    let mut full_pronunciation = String::with_capacity(64);

    // Iterate over the left chords
    for (_, joystick1_clusters) in left_chords {

        // Iterate over the right chords
        for (_, joystick2_clusters) in right_chords {

            // Iterate over vowels
            for vowel in vowels {

                let mut words_that_all_share_this_mapping: Vec<(String, f64)> = Vec::new();

                    for joystick1_cluster in joystick1_clusters {
                        for joystick2_cluster in joystick2_clusters {

                            // Construct the key by combining left, vowel, and right
                            full_pronunciation.clear();
                            full_pronunciation.push_str(joystick1_cluster);
                            full_pronunciation.push(' ');
                            full_pronunciation.push_str(vowel);
                            full_pronunciation.push(' ');
                            full_pronunciation.push_str(joystick2_cluster);

                            // if this pronunciation is a word, add it
                            if let Some(word_map) = words_and_their_frequencies.get(&full_pronunciation) {
                            for (word, zipf) in word_map {
                                words_that_all_share_this_mapping.push((word.clone(), *zipf));
                            }
                        }
                    }
                }

                if words_that_all_share_this_mapping.is_empty() {
                    continue;
                }

                // Sort by probability descending so the most common word can "win" the collision
                words_that_all_share_this_mapping.sort_by(|a, b| {
                    zipf_to_prob(b.1)
                        .partial_cmp(&zipf_to_prob(a.1))
                        .unwrap()
                });

                // all unseen words contribute to the coverage score
                // all words other than the first, contribute to the conflict score
                for (i, (word, zipf)) in words_that_all_share_this_mapping.iter().enumerate() {
                    let prob = zipf_to_prob(*zipf);
                    if !already_seen_words.contains(word) {
                        coverage_score += prob;
                        already_seen_words.insert(word.clone());
                    }

                    if i > 0 {
                        conflict_score += prob;
                    }
                }
            }
        }
    }

    (coverage_score, conflict_score)

}
use rand::Rng;
use rand::RngExt;

use rand::distr::weighted::WeightedIndex;
use rand::distr::Distribution;

pub(crate) fn random_consonant_cluster<R: Rng>(
    rng: &mut R,
    clusters: &std::collections::HashMap<String, f64>,
) -> String {

    let keys: Vec<&String> = clusters.keys().collect();
    let weights: Vec<f64> = keys
        .iter()
        .map(|k| clusters.get(*k).copied().unwrap_or(0.0))
        .collect();

    let dist = WeightedIndex::new(&weights)
        .expect("Invalid weights");

    let index = dist.sample(rng);

    keys[index].clone().to_string()
}

pub(crate) fn random_joystick_location<R: Rng>(
    rng: &mut R,
) -> String {

    // My layout I'm defining as 8 directions you can go in
    // 1 2 3
    // 4   5
    // 6 7 8
    // plus a further rotation L or R (optional)

    // that means 8*3 genes can all have unique locations.
    // however hopefully this algorithm will find that 
    // some consonant clusters will be able to share locations.

    // maybe 4R can map to both 'G L' and 'T W'

    // I won't bother adding a cost for L and R,
    // even though it's obviously more work than no rotation
    // I'll just put the most common ones first.


    let number = rng.random_range(1..=8).to_string();

    let suffix = match rng.random_range(0..3) {
        0 => "",
        1 => "L",
        _ => "R",
    };

    format!("{}{}", number, suffix)
}

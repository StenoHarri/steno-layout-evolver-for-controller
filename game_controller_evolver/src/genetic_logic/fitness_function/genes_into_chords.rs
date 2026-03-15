use std::collections::{HashMap, HashSet};

pub(crate) fn genes_into_chords(
    genes: &Vec<(String, String)>,
    valid_sounds: &HashSet<String>,
) -> HashMap<(String, String), Vec<String>> {
    // define the different types of chords
    let explicit_chords = generate_explicit_chords(genes);
    let implied_chords = generate_implied_chords(&explicit_chords, valid_sounds);

    // too expensive right now
    let sandwich_chords: HashMap<(String, String), Vec<String>> = HashMap::new();
    // let sandwich_chords = generate_sandwich_chords(&explicit_chords, &implied_chords, valid_sounds);

    // merge everything
    let mut result: HashMap<(String, String), Vec<String>> = HashMap::new();

    for (location, clusters) in explicit_chords {
        result
            .entry((location.clone(), String::new()))
            .or_insert_with(Vec::new)
            .extend(clusters);
    }

    for ((loc1, loc2), clusters) in implied_chords {
        result
            .entry((loc1, loc2))
            .or_insert_with(Vec::new)
            .extend(clusters);
    }

    for ((loc1, loc2), clusters) in sandwich_chords {
        result
            .entry((loc1, loc2))
            .or_insert_with(Vec::new)
            .extend(clusters);
    }

    result
}

pub(crate) fn generate_explicit_chords(
    genes: &[(String, String)],
) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for (cluster, location) in genes {
        map.entry(location.clone())
            .or_insert_with(Vec::new)
            .push(cluster.clone());
    }

    map
}

fn generate_implied_chords(
    explicit_chords: &HashMap<String, Vec<String>>,
    valid_sounds: &HashSet<String>,
) -> HashMap<(String, String), Vec<String>> {

    let mut implied_chords: HashMap<(String, String), Vec<String>> = HashMap::new();

    // collect explicit clusters for fast lookup
    let explicit_set: HashSet<String> = explicit_chords
        .values()
        .flat_map(|v| v.iter())
        .cloned()
        .collect();

    for (location1, clusters1) in explicit_chords {
        for (location2, clusters2) in explicit_chords {

            for cluster1 in clusters1 {
                for cluster2 in clusters2 {

                    let combined = format!("{} {}", cluster1, cluster2);

                    if !explicit_set.contains(&combined)
                        && valid_sounds.contains(&combined)
                    {
                        implied_chords
                            .entry((location1.clone(), location2.clone()))
                            .or_insert_with(Vec::new)
                            .push(combined);
                    }
                }
            }
        }
    }

    implied_chords
}

#[allow(dead_code)] // this is just too computationally expensive
fn generate_sandwich_chords(
    explicit_chords: &HashMap<String, Vec<String>>,
    implied_chords: &HashMap<(String, String), Vec<String>>,
    valid_sounds: &HashSet<String>,
) -> HashMap<(String, String), Vec<String>> {

    let mut sandwich_chords: HashMap<(String, String), Vec<String>> = HashMap::new();

    let explicit_set: HashSet<String> = explicit_chords
        .values()
        .flat_map(|v| v.iter())
        .cloned()
        .collect();

    let implied_set: HashSet<String> = implied_chords
        .values()
        .flat_map(|v| v.iter())
        .cloned()
        .collect();

    for (location1, clusters1) in explicit_chords {
        for (_dropped_location, clusters2) in explicit_chords {
            for (location2, clusters3) in explicit_chords {

                for cluster1 in clusters1 {
                    for cluster2 in clusters2 {
                        for cluster3 in clusters3 {

                            let combined = format!("{} {} {}", cluster1, cluster2, cluster3);

                            if !explicit_set.contains(&combined)
                                && !implied_set.contains(&combined)
                                && valid_sounds.contains(&combined)
                            {
                                sandwich_chords
                                    .entry((location1.clone(), location2.clone()))
                                    .or_insert_with(Vec::new)
                                    .push(combined);
                            }
                        }
                    }
                }
            }
        }
    }

    sandwich_chords
}
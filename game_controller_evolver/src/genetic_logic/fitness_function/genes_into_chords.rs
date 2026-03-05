use std::collections::HashSet;
use std::collections::HashMap;



#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExplicitChord {
    pub clusters: Vec<String>, // a list of all the possible clusters
    pub location: String, // a location is a number 1-8 and optionally a R or L
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImpliedChord {
    pub clusters: Vec<String>,
    pub location: Vec<String>, // two locations
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SandwichChord {
    pub clusters: Vec<String>,
    pub location: Vec<String>, // two locations
}


pub(crate) fn genes_into_chords(
    genes: &Vec<(String, String)>,
    valid_sounds: &HashSet<String>,
) -> HashMap<(String, String), Vec<Vec<String>>> { // for every pair of locations, a list of all the different clusters they can map to

    

    // define the different types of chords
    let explicit_chords = generate_explicit_chords(genes);
    let implied_chords = generate_implied_chords(&explicit_chords, &valid_sounds);
    let sandwich_chords = generate_sandwich_chords(&explicit_chords, &implied_chords, &valid_sounds);


    // add the chords to one place

    let mut result: HashMap<(String, String), Vec<Vec<String>>> = HashMap::new();

    for (location, clusters) in explicit_chords {
        result
            .entry((location.clone(), String::new()))
            .or_insert_with(Vec::new)
            .extend(clusters.clone());
    }
    for ((loc1, loc2), clusters) in implied_chords {
        result
            .entry((loc1.clone(), loc2.clone()))
            .or_insert_with(Vec::new)
            .extend(clusters.clone());
    }
    for ((loc1, loc2), clusters) in sandwich_chords {
        result
            .entry((loc1.clone(), loc2.clone()))
            .or_insert_with(Vec::new)
            .extend(clusters.clone());
    }

    result
}



fn generate_explicit_chords(
    genes: &[(String, String)],
) -> HashMap<String, Vec<Vec<String>>> {

    let mut map: HashMap<String, Vec<Vec<String>>> = HashMap::new();

    for (cluster, location) in genes { // swap names
        map.entry(location.clone())               // location is the key
            .or_insert_with(Vec::new)
            .push(vec![cluster.clone()]);        // cluster is the value
    }
    map
}

fn generate_implied_chords(
    explicit_chords: &HashMap<String, Vec<Vec<String>>>,
    valid_sounds: &HashSet<String>,
) -> HashMap<(String, String), Vec<Vec<String>>> {

    let mut implied_chords  = HashMap::new();


    // not too confident this is doing what I want, but lets me do if !explicit_strings.contains(&combined_string)
    let explicit_set: HashSet<Vec<String>> =
        explicit_chords
            .values()
            .flat_map(|v| v.iter())
            .cloned()
            .collect();
    
    for (location1, clusters1) in explicit_chords {
        for (location2, clusters2) in explicit_chords {
            
            for cluster1 in clusters1 {
                for cluster2 in clusters2 {


                    // combine the clusters
                    let mut combined = Vec::with_capacity(
                        cluster1.len() + cluster2.len()
                    );

                    combined.extend_from_slice(cluster1);
                    combined.extend_from_slice(cluster2);

                    // Making it a string so it can be hashed against valid_sounds
                    let combined_string= combined.join(""); // This joins all elements into one string


                    // make sure it's not already an explicit chord
                    if !explicit_set.contains(&combined) 
                        && valid_sounds.contains(&combined_string) 
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



fn generate_sandwich_chords(
    explicit_chords: &HashMap<String, Vec<Vec<String>>>,
    implied_chords: &HashMap<(String, String), Vec<Vec<String>>>,
    valid_sounds: &HashSet<String>,
) -> HashMap<(String, String), Vec<Vec<String>>> {

    let mut sandwich_chords = HashMap::new();
    
    // Precompute explicit strings
    let explicit_set: HashSet<Vec<String>> =
    explicit_chords
        .values()
        .flat_map(|v| v.iter())
        .cloned()
        .collect();

    // Precompute implied strings
    let implied_set: HashSet<Vec<String>> =
    implied_chords
        .values()
        .flat_map(|v| v.iter())
        .cloned()
        .collect();


    // the whole point of a sandwich chord is that there's three chords, but you drop the middle one's location
    
    for (location1, clusters1) in explicit_chords {
        for (dropped_cluster_location, clusters2) in explicit_chords {
            for (location2, clusters3) in explicit_chords {
                
                for cluster1 in clusters1 {
                    for cluster2 in clusters2 {
                        for cluster3 in clusters3 {


                            // combine the clusters
                            let mut combined = Vec::with_capacity(
                                cluster1.len() + cluster2.len() + cluster3.len()
                            );

                            combined.extend_from_slice(cluster1);
                            combined.extend_from_slice(cluster2);
                            combined.extend_from_slice(cluster3);

                            // Making it a string so it can be hashed against valid_sounds
                            let combined_string= combined.join(""); // This joins all elements into one string


                            // make sure it's not already an explicit chord or implicit chord
                            if !explicit_set.contains(&combined)
                                && !implied_set.contains(&combined) 
                                && valid_sounds.contains(&combined_string) 
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





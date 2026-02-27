use crate::genetic_logic::keyboard_layout::KeyboardLayout;
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
    valid_sounds: HashSet<String>,
) -> HashMap<(String, String), Vec<Vec<String>>> { // for every pair of locations, a list of all the different clusters they can map to

    let mut result: HashMap<(String, String), Vec<Vec<String>>> = HashMap::new();
    
    let explicit_chords = generate_explicit_chords(genes.clone()); //these are a single location

    let implied_chords = generate_implied_chords( // if it ends in a suffix ^S ^G maybe logic?
        explicit_chords, // explicit + explicit = implicit (unless there is already an explicit chord covering that)
    );

    let sandwich_chords = generate_sandwich_chords(
        explicit_chords, // explicit + explicit(but drop the location) + explicit = sandwich
        implied_chords, // if the chord can be made with implied chords, exclude it from sandwiches
    );

}



fn generate_explicit_chords(
    genes: Vec<(String, String)>,
) -> HashMap<String, Vec<Vec<String>>> {

    let mut map: HashMap<String, Vec<Vec<String>>> = HashMap::new();

    for (location, cluster) in genes { // For every gene, stick its cluster at a given location
        map.entry(location)
            .or_insert_with(Vec::new)
            .push(vec![cluster]);
    }
    map
}

fn generate_implied_chords(
    explicit_chords: &HashMap<String, Vec<Vec<String>>>,
    valid_sounds: &HashSet<String>,
) -> HashMap<(String, String), Vec<Vec<String>>> {

    let mut implied_chords: HashMap<(String, String), Vec<Vec<String>>> = HashMap::new();
    
    for (location1, cluster1) in explicit_chords {
        for (location2, cluster2) in explicit_chords {
            
            for cluster1 in clusters1:
                for cluster2 in cluster2:
                


            }
    }
    implied_chords
}
use crate::genetic_logic::keyboard_layout::KeyboardLayout;
use std::collections::HashSet;



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
    left_genes: Vec<(String, String)>,
    right_genes: Vec<(String, String)>,
) -> HashMap<(String, String), Vec<Vec<String>>> { // for every pair of locations, a list of all the different clusters they can map to


        explicit_chords = generate_explicit_chords()

        implied_chords = generate_implied_chords( // if it ends in a suffix ^S ^G maybe logic?
            explicit_chords, // explicit + explicit = implicit
        )

        sandwich_chords = generate_sandwich_chords(
            explicit_chords, // explicit + explicit(but drop the location) + explicit = sandwich
            implied_chords, // if the chord can be made with implied chords, exclude it from sandwiches
        )

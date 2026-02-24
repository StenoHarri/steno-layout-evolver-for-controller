

pub(crate) fn genes_into_chords(
    genes: KeyboardLayout,
    joystick: It's either gunna be left_chords or right_chords
) -> 


        explicit_chords = generate_explicit_chords()

        implied_chords = generate_implied_chords( // if it ends in a suffix ^S ^G maybe logic?
            explicit_chords, // explicit + explicit = implicit
        )

        sandwich_chords = generate_sandwich_chords(
            explicit_chords, // explicit + explicit(but drop the location) + explicit = sandwich
            implied_chords, // if the chord can be made with implied chords, exclude it from sandwiches
        )

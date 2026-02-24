


pub(crate) fn measure_layout(
    keyboard_genes: KeyboardLayoutd,
) -> 

    left_chords = genes_into_chords(
        keyboard_chords,
        "left",
    );

    right_chords = genes_into_chords(
        keyboard_chords,
        "right",
    );

    // Vowels will not be written with the joysticks, I will reuse the 4 vowel keys the stenotype has
    // These can either be the 4 triggers or the 4 paddles, whichever is easiest to hit
    vowels = {"AA", "AE", "AH", "AO", "AW", "AY",
          "EH", "ER", "EY", "IH", "IY", "OW", "OY", "UH", "UW"}

    coverage, collisions = find_matches(
        left_chords,
        vowels,
        right_chords,
        words_and_their_frequencies,
    )

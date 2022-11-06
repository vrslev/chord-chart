use std::str::FromStr;

use chord_chart::{Chart, Error, Note, Transpose};

#[test]
fn transpose_integration() -> Result<(), Error> {
    let current_key = Note::from_str("E")?;
    let current_chords = Chart::from_str("| A/E | E |\n| C#m |")?;
    let new_key = Note::from_str("Db")?;
    let expected_new_chords = "| Gb/Db | Db |\n| Bbm |";

    assert_eq!(
        current_chords
            .transpose(
                &current_key.get_semitones_diff(&new_key),
                &new_key.accidental().scale()
            )
            .to_string(),
        expected_new_chords
    );

    Ok(())
}

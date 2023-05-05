use std::collections::HashMap;
use strum_macros::EnumIter;


#[derive(EnumIter)]
pub enum ReadingFrame {
    One,
    Two,
    Three
}

/**
 * Translates a given sequence for a given reading frame using the given codon table.
 * 
 * Returns a Vector of Strings of possible translated sequences based on reading frame
 */
pub fn translate<'a>(sequence: &'a str, codon_table: &HashMap<&'a str, char>, reading_frame: Option<ReadingFrame>) -> String {
    let frame: ReadingFrame = reading_frame.unwrap_or(ReadingFrame::One);
    let mut start_index;
    match frame {
        ReadingFrame::One => start_index = 0,
        ReadingFrame::Two => start_index = 1,
        ReadingFrame::Three => start_index = 2
    }
    let normalized_seq = sequence.replace(" ", "").to_uppercase();
    let mut peptide = String::new();
    while start_index + 3 <= normalized_seq.len() {
        let reading_frame = &normalized_seq[start_index..start_index + 3];
        let residue = codon_table.get(&reading_frame);
        match residue {
            None => panic!("{} does not correlate to a codon in the standard table", reading_frame),
            Some(amino_acid) => {
                peptide = format!("{}{}", peptide, amino_acid.to_string());
            }
        }
        start_index += 3;
    }
    return peptide;
}




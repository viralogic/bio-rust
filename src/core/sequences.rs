use crate::core::language;


pub struct Sequence<'a> {
    sequence: String,
    language: &'a language::Language
}

impl Sequence<'_> {
    pub fn new<'a>(sequence: &str, language: &'a language::Language) -> Sequence<'a> {
        return Sequence {
            sequence: String::from(sequence).replace(" ", "").to_uppercase(),
            language
        }
    }
    /* Calculates the molecular weight of the sequence
     */
    pub fn mol_wt(&self) -> f64 {
        return self.language.weights.mol_wt(&self.sequence);
    }

    /* Calculates the length of the sequence
     */
    pub fn length(&self) -> usize {
        return self.sequence.len();
    }
}

use crate::core::language;
use crate::core::weights::MolWt;
use crate::core::complement::Nucleoside;
use crate::algorithms::translation::{
    translate,
    ReadingFrame
};
use strum::IntoEnumIterator;

pub struct Peptide<'a> {
    sequence: String,
    language: &'a dyn MolWt
}

impl<'a> Peptide<'a> {
    pub fn new(sequence: &str) -> Peptide<'a> {
        let seq = String::from(sequence).replace(" ", "").to_uppercase();
        let alphabet = &*language::PEPTIDE;
        let illegal_residues = seq.chars().any(
            |c| !alphabet.weights.residue_weights.contains_key(&c)
        );
        if illegal_residues {
            panic!("{} is not a valid peptide sequence", sequence);
        }
        return Peptide {
            sequence: seq,
            language: alphabet
        }
    }
    
    pub fn mol_wt(&self) -> f64 {
        return self.language.mol_wt(&self.sequence);
    }

    pub fn length(&self) -> usize {
        return self.sequence.len();
    }
}

impl<'a> PartialEq for Peptide<'a> {
    fn eq(&self, other: &Self) -> bool {
        return self.sequence == other.sequence;
    }

    fn ne(&self, other: &Self) -> bool {
        return self.sequence != other.sequence;
    }
}

pub struct RNA<'a> {
    sequence: String,
    language: &'a language::Nucleoside<'a>
}

impl<'a> RNA<'a> {
    pub fn new(sequence: &str) -> RNA<'a> {
        return RNA {
            sequence: String::from(sequence).replace(" ", "").to_uppercase(),
            language: &*language::RNA
        }
    }

    pub fn mol_wt(&self) -> f64 {
        return self.language.mol_wt(&self.sequence);
    }

    pub fn length(&self) -> usize {
        return self.sequence.len();
    }

    pub fn back_transcribe(&self) -> DNA {
        return DNA {
            sequence: String::from(&self.sequence.replace("U", "T")),
            language: self.language
        }
    }

    /**
     * Provides translations of the RNA sequence for each of the 3 possible
     * reading frames.
     */
    pub fn translate(&self) -> Vec<Peptide> {
        let mut translations: Vec<Peptide> = Vec::new();

        for frame in ReadingFrame::iter() {
            let translated_sequence = translate(&self.sequence, &self.language.codon_table, Some(frame));
            translations.push(Peptide::new(
                &translated_sequence
            ))
        }
        return translations;
    }
}


pub struct DNA<'a> {
    sequence: String,
    language: &'a language::Nucleoside<'a>
}

impl<'a> DNA<'a> {
    pub fn new(sequence: &str) -> DNA<'a> {
        return DNA {
            sequence: String::from(sequence).replace(" ", "").to_uppercase(),
            language: &*language::DNA
        }
    }

    pub fn mol_wt(&self) -> f64 {
        return self.language.mol_wt(&self.sequence);
    }

    pub fn length(&self) -> usize {
        return self.sequence.len();
    }

    pub fn complement(&self) -> DNA<'a> {
        return DNA {
            sequence: self.language.complement(&self.sequence),
            language: self.language
        }
    }

    pub fn reverse_complement(&self) -> DNA<'a> {
        return DNA { 
            sequence: self.language.reverse_complement(&self.sequence),
            language: self.language
        }
    }

    pub fn transcribe(&self) -> RNA {
        return RNA {
            sequence: String::from(&self.sequence.replace("T", "U")),
            language: self.language
        }
    }

    pub fn translate(&self) -> Vec<Peptide> {
        let mut translations: Vec<Peptide> = Vec::new();
        for frame in ReadingFrame::iter() {
            let translation: String = translate(
                &self.sequence, &self.language.codon_table, Some(frame)
            );
            translations.push(Peptide::new(&translation));
        }

        let complement = self.complement().sequence;
        for frame in ReadingFrame::iter() {
            let translation: String = translate(
                &complement, &self.language.codon_table, Some(frame)
            );
            translations.push(Peptide::new(&translation));
        }
        return translations;
    }
}
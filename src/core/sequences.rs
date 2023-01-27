use crate::core::language;
use crate::core::weights::MolWt;
use crate::core::complement::Nucleoside;

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

pub struct Oligo<'a, T: MolWt + Nucleoside> {
    sequence: String,
    language: &'a T
}

impl<'a, T: MolWt + Nucleoside> Oligo<'a, T> {
    pub fn new(sequence: &str, language: &'a T) -> Oligo<'a, T> {
        return Oligo {
            sequence: String::from(sequence).replace(" ", "").to_uppercase(),
            language
        }
    }

    pub fn mol_wt(&self) -> f64 {
        return self.language.mol_wt(&self.sequence);
    }

    pub fn length(&self) -> usize {
        return self.sequence.len();
    }

    pub fn complement(&self) -> Oligo<'a, T> {
        return Oligo {
            sequence: self.language.complement(&self.sequence),
            language: self.language
        }
    }

    pub fn reverse_complement(&self) -> Oligo<'a, T> {
        return Oligo { 
            sequence: self.language.reverse_complement(&self.sequence),
            language: self.language
        }
    }
}


use crate::core::language;
use crate::core::weights::MolWt;
use crate::core::complement::Nucleoside;

pub struct Peptide<'a> {
    sequence: String,
    language: &'a dyn MolWt
}

impl<'a> Peptide<'a> {
    pub fn new(sequence: &str) -> Peptide<'a> {
        return Peptide {
            sequence: String::from(sequence).replace(" ", "").to_uppercase(),
            language: &*language::PEPTIDE
        }
    }
    
    pub fn mol_wt(&self) -> f64 {
        return self.language.mol_wt(&self.sequence);
    }

    pub fn length(&self) -> usize {
        return self.sequence.len();
    }
}

pub struct Nucleotide<'a, T: MolWt + Nucleoside> {
    sequence: String,
    language: &'a T
}

impl<'a, T: MolWt + Nucleoside> Nucleotide<'a, T> {
    pub fn new(sequence: &str, language: &'a T) -> Nucleotide<'a, T> {
        return Nucleotide {
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

    pub fn complement(&self) -> Nucleotide<'a, T> {
        return Nucleotide {
            sequence: self.language.complement(&self.sequence),
            language: self.language
        }
    }

    pub fn reverse_complement(&self) -> Nucleotide<'a, T> {
        return Nucleotide { 
            sequence: self.language.reverse_complement(&self.sequence),
            language: self.language
        }
    }
}


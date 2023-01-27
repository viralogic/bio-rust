use std::collections::HashMap;

use crate::core::weights;
use crate::core::complement;
use crate::core::complement::Complement;


pub struct Language<'a> {
    pub weights: &'a weights::Weights,
}

impl weights::MolWt for Language<'_> {
    fn mol_wt(&self, sequence: &str) -> f64 {
        return self.weights.mol_wt(sequence);
    }
}

pub struct Nucleoside<'a> {
    pub weights: &'a weights::Weights,
    pub complement: Complement
}

impl weights::MolWt for Nucleoside<'_> {
    fn mol_wt(&self, sequence: &str) -> f64 {
        return self.weights.mol_wt(sequence);
    }
}

impl complement::Nucleoside for Nucleoside<'_> {
    fn complement(&self, sequence: &str) -> String {
        return self.complement.complement(sequence);
    }

    fn reverse(&self, sequence: &str) -> String {
        return self.complement.reverse(sequence);
    }

    fn reverse_complement(&self, sequence: &str) -> String {
        return self.complement.reverse_complement(sequence);
    }
}

lazy_static! {
    pub static ref DNA: Nucleoside<'static> = Nucleoside { 
        weights: &*weights::DNA,
        complement: Complement {
            complements: HashMap::from([('A', 'T'), ('T', 'A'), ('C', 'G'), ('G', 'C')])
        }
    };
    pub static ref RNA: Nucleoside<'static> = Nucleoside {
        weights: &*weights::RNA,
        complement: Complement {
            complements: HashMap::from([('A', 'U'), ('U', 'A'), ('C', 'G'), ('G', 'C')])
        }
    };
    pub static ref PEPTIDE: Language<'static> = Language {
        weights: &*weights::PEPTIDE
    };
} 

use std::collections::HashMap;
use super::traits::{
    MolWt,
    Seq,
};

struct Sequence {
    sequence: String,
    weight_mappings: HashMap<char, f64>
}

impl Seq for Sequence {
    fn length(&self) -> usize {
        return self.sequence.len();
    }
}

impl MolWt for Sequence {
    fn mol_wt(&self) -> f64 {
        let mut sum = 0f64;
        for residue in self.sequence.chars() {
            if !self.weight_mappings.contains_key(&residue) {
                panic!("{} is not a valid DNA residue", &residue);
            }
            sum += self.weight_mappings[&residue];
        }
        return sum;
    }
}

pub struct DNA {
    sequence: Sequence
}

impl DNA {
    pub fn new(sequence: String) -> DNA {
        return DNA {
            sequence: Sequence {
                sequence: sequence.replace(" ", "").to_uppercase(),
                weight_mappings: [
                    ('A', 313.21),
                    ('G', 329.21),
                    ('C', 289.18),
                    ('T', 304.2),
                ].iter().cloned().collect()
            }
        };
    }
}

impl MolWt for DNA {
    fn mol_wt(&self) -> f64 {
        return self.sequence.mol_wt() - 61.96;
    }
}

impl Seq for DNA {
    fn length(&self) -> usize {
        return self.sequence.length();
    }
}

pub struct RNA {
    sequence: Sequence
}

impl RNA {
    pub fn new(sequence: String) -> RNA {
        return RNA {
            sequence: Sequence {
                sequence: sequence.replace(" ", "").to_uppercase(),
                weight_mappings: [
                    ('A', 329.2),
                    ('G', 345.2),
                    ('C', 305.2),
                    ('U', 306.2)
                ].iter().cloned().collect()
            }
        };
    }
}

impl MolWt for RNA {
    fn mol_wt(&self) -> f64 {
        return self.sequence.mol_wt() + 159.0;
    }
}

pub struct Peptide {
    sequence: Sequence
}

impl Peptide {
    pub fn new(sequence: String) -> Peptide {
        return Peptide {
            sequence: Sequence {
                sequence: sequence.replace(" ", "").to_uppercase(),
                weight_mappings: [
                    ('A', 71.08),
                    ('R', 156.19),
                    ('N', 114.11),
                    ('D', 115.09),
                    ('C', 103.15),
                    ('E', 129.12),
                    ('Q', 128.13),
                    ('G', 57.05),
                    ('H', 137.14),
                    ('I', 113.16),
                    ('L', 113.16),
                    ('K', 128.18),
                    ('M', 131.20),
                    ('F', 147.18),
                    ('P', 97.12),
                    ('S', 87.08),
                    ('T', 101.11),
                    ('W', 186.22),
                    ('Y', 163.18),
                    ('V', 99.13),
                ].iter().cloned().collect()
            }
        }
    }
}

impl MolWt for Peptide {
    fn mol_wt(&self) -> f64 {
        return self.sequence.mol_wt() + 18.02;
    }
}

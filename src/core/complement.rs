use std::collections::HashMap;

pub trait Nucleoside {
    fn complement(&self, sequence: &str) -> String;
    fn reverse(&self, sequence: &str) -> String;
    fn reverse_complement(&self, sequence: &str) -> String;
}

pub struct Complement {
    pub complements: HashMap<char, char>,
}

impl Nucleoside for Complement {
    fn complement(&self, sequence: &str) -> String {
        let mut comp: Vec<char> = Vec::new();
        let normalized = sequence.to_string().replace(" ", "").to_uppercase();
        for residue in normalized.chars() {
            if !self.complements.contains_key(&residue) {
                panic!("{} is not a valid residue", residue);
            }
            let value = self.complements[&residue];
            comp.push(value);
        }
        return comp.into_iter().collect();
    }

    fn reverse(&self, sequence: &str) -> String {
        return sequence.chars().rev().collect();
    }

    fn reverse_complement(&self, sequence: &str) -> String {
        return self.reverse(&self.complement(sequence));
    }
}


use std::collections::HashMap;

pub trait MolWt {
    fn mol_wt(&self, sequence: &str) -> f64;
}

pub struct Weights {
    residue_weights: HashMap<char, f64>,
    factor: f64
}

impl Weights {
    fn new(weights: Vec<(char, f64)>, factor: f64) -> Weights {
        let mut mapped_weights: HashMap<char, f64> = HashMap::new();
        for (residue, mol_wt) in weights {
            mapped_weights.insert(residue, mol_wt);
        } 
        return Weights {
            residue_weights: mapped_weights,
            factor
        };
    }
}

impl MolWt for Weights {
    fn mol_wt(&self, sequence: &str) -> f64 {
        let mut sum = 0f64;
        let normalized = sequence.to_string().replace(" ", "").to_uppercase();
        for residue in normalized.chars() {
            if !self.residue_weights.contains_key(&residue) {
                panic!("{} is not a valid residue", &residue);
            }
            sum += self.residue_weights[&residue];
        }
        return sum + self.factor;
    }
}

lazy_static! {
    pub static ref DNA: Weights = Weights::new(
        vec![
            ('A', 313.21),
            ('G', 329.21),
            ('C', 289.18),
            ('T', 304.2),
        ],
        -61.96);

    pub static ref RNA: Weights = Weights::new(
        vec![
            ('A', 329.2),
            ('G', 345.2),
            ('C', 305.2),
            ('U', 306.2)
        ],
        159.0);

    pub static ref PEPTIDE: Weights = Weights::new(
        vec![
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
        ],
        18.02);
}

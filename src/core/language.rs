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
    pub complement: Complement,
    pub codon_table: HashMap<&'a str, char>,
    pub start_codon: HashMap<&'a str, char>
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
        },
        codon_table: HashMap::from([
            ("GCT", 'A'),
            ("GCC", 'A'),
            ("GCA", 'A'),
            ("GCG", 'A'),
            ("CGT", 'R'),
            ("CGC", 'R'),
            ("CGA", 'R'),
            ("CGG", 'R'),
            ("AGA", 'R'),
            ("AGG", 'R'),
            ("AAT", 'N'),
            ("AAC", 'N'),
            ("GAT", 'D'),
            ("GAC", 'D'),
            ("TGT", 'C'),
            ("TGC", 'C'),
            ("CAA", 'Q'),
            ("CAG", 'Q'),
            ("GAA", 'E'),
            ("GAG", 'E'),
            ("GGT", 'G'),
            ("GGC", 'G'),
            ("GGA", 'G'),
            ("GGG", 'G'),
            ("CAT", 'H'),
            ("CAC", 'H'),
            ("ATT", 'I'),
            ("ATC", 'I'),
            ("ATA", 'I'),
            ("CTT", 'L'),
            ("CTC", 'L'),
            ("CTA", 'L'),
            ("CTG", 'L'),
            ("TTA", 'L'),
            ("TTG", 'L'),
            ("AAA", 'K'),
            ("AAG", 'K'),
            ("ATG", 'M'),
            ("TTT", 'F'),
            ("TTC", 'F'),
            ("CCT", 'P'),
            ("CCC", 'P'),
            ("CCA", 'P'),
            ("CCG", 'P'),
            ("TCT", 'S'),
            ("TCC", 'S'),
            ("TCA", 'S'),
            ("TCG", 'S'),
            ("AGT", 'S'),
            ("AGC", 'S'),
            ("ACT", 'T'),
            ("ACC", 'T'),
            ("ACA", 'T'),
            ("ACG", 'T'),
            ("TGG", 'W'),
            ("TAT", 'Y'),
            ("TAC", 'Y'),
            ("GTT", 'V'),
            ("GTC", 'V'),
            ("GTA", 'V'),
            ("GTG", 'V'),
            ("TAA", '<'),
            ("TGA", '<'),
            ("TAG", '<')
        ]),
        start_codon: HashMap::from([
            ("ATG", '>'),
            ("TTG", '>'),
            ("GTG", '>')
        ])
    };
    pub static ref RNA: Nucleoside<'static> = Nucleoside {
        weights: &*weights::RNA,
        complement: Complement {
            complements: HashMap::from([('A', 'U'), ('U', 'A'), ('C', 'G'), ('G', 'C')])
        },
        codon_table: HashMap::from([
            ("GCU", 'A'),
            ("GCC", 'A'),
            ("GCA", 'A'),
            ("GCG", 'A'),
            ("CGU", 'R'),
            ("CGC", 'R'),
            ("CGA", 'R'),
            ("CGG", 'R'),
            ("AGA", 'R'),
            ("AGG", 'R'),
            ("AAU", 'N'),
            ("AAC", 'N'),
            ("GAU", 'D'),
            ("GAC", 'D'),
            ("UGU", 'C'),
            ("UGC", 'C'),
            ("CAA", 'Q'),
            ("CAG", 'Q'),
            ("GAA", 'E'),
            ("GAG", 'E'),
            ("GGU", 'G'),
            ("GGC", 'G'),
            ("GGA", 'G'),
            ("GGG", 'G'),
            ("CAU", 'H'),
            ("CAC", 'H'),
            ("AUU", 'I'),
            ("AUC", 'I'),
            ("AUA", 'I'),
            ("CUU", 'L'),
            ("CUC", 'L'),
            ("CUA", 'L'),
            ("CUG", 'L'),
            ("UUA", 'L'),
            ("UUG", 'L'),
            ("AAA", 'K'),
            ("AAG", 'K'),
            ("AUG", 'M'),
            ("UUU", 'F'),
            ("UUC", 'F'),
            ("CCU", 'P'),
            ("CCC", 'P'),
            ("CCA", 'P'),
            ("CCG", 'P'),
            ("UCU", 'S'),
            ("UCC", 'S'),
            ("UCA", 'S'),
            ("UCG", 'S'),
            ("AGC", 'S'),
            ("AGU", 'S'),
            ("ACU", 'T'),
            ("ACC", 'T'),
            ("ACA", 'T'),
            ("ACG", 'T'),
            ("UGG", 'W'),
            ("UAU", 'Y'),
            ("UAC", 'Y'),
            ("GUU", 'V'),
            ("GUC", 'V'),
            ("GUA", 'V'),
            ("GUG", 'V'),
            ("UAA", '<'),
            ("UGA", '<'),
            ("UAG", '<')
        ]),
        start_codon: HashMap::from([
            ("AUG", '>'),
            ("UUG", '>'),
            ("GUG", '>')
        ])
    };
    pub static ref PEPTIDE: Language<'static> = Language {
        weights: &*weights::PEPTIDE
    };
} 

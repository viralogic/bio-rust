use super::super::super::core::sequences::{
    DNA,
    RNA,
    Peptide
};
use super::super::super::core::traits::{MolWt, Seq};
use rstest::rstest;

#[rstest]
#[case(String::from("cccccccccc"), 2830.0)]
#[case(String::from("GGGGGGGGGG"), 3230.0)]
#[case(String::from("ATC GAT CGT T"), 3018.0)]
#[case(String::from("aaaAAAAaaa"), 3070.0)]
#[case(String::from("ttt TTT ttt T"), 2980.0)]
fn test_dna_sequence_mol_wt(#[case] sequence: String, #[case] mol_wt: f64) {
    let dna_sequence = DNA::new(sequence);
    assert_eq!(dna_sequence.mol_wt().round(), mol_wt);
}

#[rstest]
#[case(String::from("cccccccccc"), 10)]
#[case(String::from("GGGGGGG"), 7)]
#[case(String::from("ATC GAT CGT"), 9)]
#[case(String::from("aaaAAAAaaATC GAT CGT"), 18)]
fn test_sequence_length(#[case] sequence: String, #[case] length: usize) {
    let sequence = DNA::new(sequence);
    assert_eq!(sequence.length(), length);
}

#[rstest]
#[case(String::from("cccccccccc"), 3211.0)]
#[case(String::from("GGGGGGGGGG"), 3611.0)]
#[case(String::from("AUC GAU CGU U"), 3343.0)]
#[case(String::from("aaaAAAAaaa"), 3451.0)]
#[case(String::from("uuu UUU uuu U"), 3221.0)]
fn test_rna_sequence_mol_wt(#[case] sequence: String, #[case] mol_wt: f64) {
    let rna_sequence = RNA::new(sequence);
    assert_eq!(rna_sequence.mol_wt().round(), mol_wt)
}

#[rstest]
#[case(String::from("ARNDCEQGHILKMFPSTWYV"), 2396.0)]
#[case(String::from("aRNDCEQGHILKMFPSTWYV"), 2396.0)]
#[case(String::from("aRNDCEQGHIL    KMFPSTWYV"), 2396.0)]
fn test_protein_sequence_mol_wt(#[case] sequence: String, #[case] mol_wt: f64) {
    let protein_sequence = Peptide::new(sequence);
    assert_eq!(protein_sequence.mol_wt().round(), mol_wt)
}
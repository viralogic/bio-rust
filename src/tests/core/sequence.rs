use crate::core::sequences::{
    DNA,
    RNA,
    Peptide
};
use rstest::rstest;

#[rstest]
#[case("cccccccccc", 10)]
#[case("GGGGGGG", 7)]
#[case("ATC GAT CGT", 9)]
#[case("aaaAAAAaaATC GAT CGT", 18)]
#[case("", 0)]
fn test_dna_sequence_length(#[case] sequence: &str, #[case] length: usize) {
    let sequence = DNA::new(sequence);
    assert_eq!(sequence.length(), length);
}

#[rstest]
#[case("ARNDCEQGHILKMFPSTWYV")]
#[case("AUC GAU CGU")]
fn test_dna_sequence_should_panic(#[case] sequence: &str) {
    DNA::new(sequence);
}

#[rstest]
#[case("ARNDCEQGHILKMFPSTWYV")]
#[case("ATC GAT CGT")]
fn test_rna_sequence_should_panic(#[case] sequence: &str) {
    RNA::new(sequence);
}

#[rstest]
#[case("", 0)]
#[case("ARNDCEQGHILKMFPSTWYV", 20)]
#[case("aRNDCEQ GHILKMFP", 15)]
#[case("aRNDCEQGHIL    KMFPSTW", 18)]
fn test_peptide_sequence_length(#[case] sequence: &str, #[case] length: usize) {
    let sequence = Peptide::new(sequence);
    assert_eq!(sequence.length(), length);
}

#[rstest]
#[should_panic]
#[case("ccccccccccU")]
#[case("ccccccccccT")]
fn test_peptide_seqquence_panic(#[case] sequence: &str) {
    Peptide::new(sequence);
}


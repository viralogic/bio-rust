use crate::core::sequences::{
    Oligo,
    Peptide
};
use crate::core::language;
use rstest::rstest;

#[rstest]
#[case("cccccccccc", 10)]
#[case("GGGGGGG", 7)]
#[case("ATC GAT CGT", 9)]
#[case("aaaAAAAaaATC GAT CGT", 18)]
#[case("AUC GAU CGU", 9)]
#[case("", 0)]
fn test_oligo_sequence_length(#[case] sequence: &str, #[case] length: usize) {
    let sequence = Oligo::new(sequence, &*language::DNA);
    assert_eq!(sequence.length(), length);
}

#[rstest]
#[case("ARNDCEQGHILKMFPSTWYV")]
fn test_dna_sequence_should_panic(#[case] sequence: &str) {
    Oligo::new(sequence, &*language::DNA);
}

#[rstest]
#[case("ARNDCEQGHILKMFPSTWYV")]
fn test_rna_sequence_should_panic(#[case] sequence: &str) {
    Oligo::new(sequence, &*language::RNA);
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
fn test_peptide_seqquence_panic(#[case] sequence: &str) {
    Peptide::new(sequence);
}


use crate::core::sequences::{
    Nucleotide,
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
fn test_nucleoside_sequence_length(#[case] sequence: &str, #[case] length: usize) {
    let sequence = Nucleotide::new(sequence, &*language::DNA);
    assert_eq!(sequence.length(), length);
}

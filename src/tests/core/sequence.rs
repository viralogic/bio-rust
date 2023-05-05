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
#[case(
    RNA::new("GUCAUGGCCAUUGUAAUGGGCCGCUGAAAGGGUGCCCGAUAGUUG"),
    vec![Peptide::new("VMAIVMGR<KGAR<L"), Peptide::new("SWPL<WAAERVPDS"), Peptide::new("HGHCNGPLKGCPIV")]
)]
#[case(
    RNA::new("AUGUACUCGAUAGGCUGUAGCGAGUAG"),
    vec![Peptide::new("MYSIGCSE<"), Peptide::new("CTR<AVAS"), Peptide::new("VLDRL<RV")]
)]
#[case(RNA::new(""), vec![Peptide::new(""), Peptide::new("")])]
fn test_rna_translate(#[case] sequence: RNA, #[case] expected: Vec<Peptide>) {
    for peptide in expected.iter() {
        assert!(sequence.translate().contains(peptide));
    }
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

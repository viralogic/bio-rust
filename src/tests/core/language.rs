use crate::core::language;

use rstest::rstest;


#[rstest]
#[case(&*language::DNA, "cccccccccc", 2830.0)]
#[case(&*language::DNA, "GGGGGGGGGG", 3230.0)]
#[case(&*language::DNA, "ATC GAT CGT T", 3018.0)]
#[case(&*language::DNA, "aaaAAAAaaa", 3070.0)]
#[case(&*language::DNA, "ttt TTT ttt T", 2980.0)]
#[case(&*language::RNA, "cccccccccc", 3211.0)]
#[case(&*language::RNA, "GGGGGGGGGG", 3611.0)]
#[case(&*language::RNA, "AUC GAU CGU U", 3343.0)]
#[case(&*language::RNA, "aaaAAAAaaa", 3451.0)]
#[case(&*language::RNA, "uuu UUU uuu U", 3221.0)]
#[case(&*language::PEPTIDE, "ARNDCEQGHILKMFPSTWYV", 2396.0)]
#[case(&*language::PEPTIDE, "aRNDCEQGHILKMFPSTWYV", 2396.0)]
#[case(&*language::PEPTIDE, "aRNDCEQGHIL    KMFPSTWYV", 2396.0)]
fn test_sequence_mol_wt(#[case] language: &language::Language, #[case] sequence: &str, #[case] mol_wt: f64) {
    assert_eq!(language.weights.mol_wt(sequence).round(), mol_wt);
}

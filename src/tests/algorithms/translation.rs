use std::collections::HashMap;
use crate::algorithms::translation::{
    translate,
    ReadingFrame
};
use crate::core::language;


use rstest::rstest;

#[rstest]
#[case("GUCAUGGCCAUUGUAAUGGGCCGCUGAAAGGGUGCCCGAUAGUUG", &language::RNA.codon_table, Option::None, "VMAIVMGR<KGAR<L")]
#[case("GUCAUGGCCAUUGUAAUGGGCCGCUGAAAGGGUGCCCGAUAGUUG", &language::RNA.codon_table, Some(ReadingFrame::One), "VMAIVMGR<KGAR<L")]
#[case("GUCAUGGCCAUUGUAAUGGGCCGCUGAAAGGGUGCCCGAUAGUUG", &language::RNA.codon_table, Some(ReadingFrame::Two), "SWPL<WAAERVPDS")]
#[case["GUCAUGGCCAUUGUAAUGGGCCGCUGAAAGGGUGCCCGAUAGUUG", &language::RNA.codon_table, Some(ReadingFrame::Three), "HGHCNGPLKGCPIV"]]
#[case("AUGUACUCGAUAGGCUGUAGCGAGUAG", &language::RNA.codon_table, Option::None, "MYSIGCSE<")]
#[case("AUGUACUCGAUAGGCUGUAGCGAGUAG", &language::RNA.codon_table, Some(ReadingFrame::One), "MYSIGCSE<")]
#[case("AUGUACUCGAUAGGCUGUAGCGAGUAG", &language::RNA.codon_table, Some(ReadingFrame::Two), "CTR<AVAS")]
#[case("AUGUACUCGAUAGGCUGUAGCGAGUAG", &language::RNA.codon_table, Some(ReadingFrame::Three), "VLDRL<RV")]
#[case["", &language::RNA.codon_table, Some(ReadingFrame::Two), ""]]
#[case("GTCATGGCCATTGTAATGGGCCGCTGAAAGGGTGCCCGATAGTTG", &language::DNA.codon_table, Option::None, "VMAIVMGR<KGAR<L")]
#[case("GTCATGGCCATTGTAATGGGCCGCTGAAAGGGTGCCCGATAGTTG", &language::DNA.codon_table, Some(ReadingFrame::One), "VMAIVMGR<KGAR<L")]
#[case("GTCATGGCCATTGTAATGGGCCGCTGAAAGGGTGCCCGATAGTTG", &language::DNA.codon_table, Some(ReadingFrame::Two), "SWPL<WAAERVPDS")]
#[case["GTCATGGCCATTGTAATGGGCCGCTGAAAGGGTGCCCGATAGTTG", &language::DNA.codon_table, Some(ReadingFrame::Three), "HGHCNGPLKGCPIV"]]
#[case("ATGTACTCGATAGGCTGTAGCGAGTAG", &language::DNA.codon_table, Option::None, "MYSIGCSE<")]
#[case("ATGTACTCGATAGGCTGTAGCGAGTAG", &language::DNA.codon_table, Some(ReadingFrame::One), "MYSIGCSE<")]
#[case("ATGTACTCGATAGGCTGTAGCGAGTAG", &language::DNA.codon_table, Some(ReadingFrame::Two), "CTR<AVAS")]
#[case("ATGTACTCGATAGGCTGTAGCGAGTAG", &language::DNA.codon_table, Some(ReadingFrame::Three), "VLDRL<RV")]
#[case["", &language::DNA.codon_table, Some(ReadingFrame::Two), ""]]
fn test_translate<'a>(#[case] sequence: &str, #[case] codon_table: &HashMap<&'a str, char>, #[case] reading_frame: Option<ReadingFrame>, #[case] expected: &str) {
    let peptide = translate(sequence, codon_table, reading_frame);
    assert_eq!(expected.to_string(), peptide);
}

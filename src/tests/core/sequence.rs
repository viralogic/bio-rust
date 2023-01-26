use crate::core::sequences::Sequence;
use crate::core::language;
use rstest::rstest;

#[rstest]
#[case("cccccccccc", 10)]
#[case("GGGGGGG", 7)]
#[case("ATC GAT CGT", 9)]
#[case("aaaAAAAaaATC GAT CGT", 18)]
fn test_sequence_length(#[case] sequence: &str, #[case] length: usize) {
    let sequence = Sequence::new(sequence, &*language::DNA);
    assert_eq!(sequence.length(), length);
}

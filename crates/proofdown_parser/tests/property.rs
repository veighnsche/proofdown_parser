use proptest::prelude::*;

proptest! {
    #[test]
    fn parse_never_panics(input in ".{0,4096}") {
        let _ = proofdown_parser::parse(&input);
    }
}

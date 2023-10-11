use c_lexer_logos::Lexer;

const PART_GCC_TEST: &'static str = include_str!("./part_gcc_test.i");

#[test]
fn it_works_for_part_gcc_test() {
    let result = Lexer::lex(PART_GCC_TEST);
    // assert_eq!(result, Ok(vec![]));
    assert_eq!(result.ok().map(|x| x.len()), Some(89672));
}

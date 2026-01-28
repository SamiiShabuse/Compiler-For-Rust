use compiler_frontend::Tokenizer;
use compiler_common::Token;

#[test]
fn test_tokenizer_simple_expression() {
    let input = "print(42 + x)";
    let mut tokenizer = Tokenizer::new(input);

    let expected_tokens = vec![
        Token::Print,
        Token::LeftParen,
        Token::Number(42),
        Token::Operator('+'),
        Token::Identifier("x".to_string()),
        Token::RightParen,
        Token::Eof,
    ];

    for expected in expected_tokens {
        let token = tokenizer.next_token();
        assert_eq!(token, expected);
    }
}

use compiler_for_rust::parser::Parser;
use compiler_for_rust::token::{self, Token};
use compiler_for_rust::tokenizer::Tokenizer;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Err(err) = compiler_for_rust::validate_args(&args) {
        eprintln!("{}", err);
        std::process::exit(1);
    }

    // going to copy general flow from professor's Java code
    // TODO: Refactor later
    let sub = &args[1];

    let mut input = String::new();
    for (i, part) in args[2..].iter().enumerate() {
        input.push_str(part);
        if i != args.len() - 3 {
            input.push(' ');
        }
    }

    // create this
    let tok = Tokenizer::new(&input);

    match sub.as_str() {
        "tokenize" => {
            let mut tokenizer = tok;
            loop {
                let p = tokenizer.peek();
                if p == Token::Eof {
                    break;
                }
                println!("{:?}", p);
            }
        }
        "parseExpr" => {
            let mut parser = Parser::new(tok);
            let expr = parser.parse_expression();
            println!("Parsed expression: {:?}", expr);  
        }
        _ => {
            eprintln!("Unknown subcommand: {}", sub);
            std::process::exit(1);
        }
    }
}


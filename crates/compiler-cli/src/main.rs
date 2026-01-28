use compiler_frontend::{Parser, Token, Tokenizer};
use compiler_ir::print_program;
use compiler_backend::lower_main;

fn validate_args(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("Usage: <comp> {tokenize|parseExpr|parseStmt|lowerMain|lowerMain2} [args..]".to_string());
    }
    Ok(()) 
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Err(err) = validate_args(&args) {
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
                let p = tokenizer.next_token();
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
        "parseStmt" => {
            let mut parser = Parser::new(tok);
            let statement = parser.parse_statement();
            println!("Parsed statement: {:?}", statement);  
        }
        
        "lowerMain" => {
            let mut parser = Parser::new(tok);
            let statements = parser.parse_statement();
            let program_ir = lower_main(&[statements]);
            println!("{}", print_program(&program_ir));
        }

        "lowerMain2" => {
            let mut parser = Parser::new(tok);

            let statement1 = parser.parse_statement();
            let mut stmts = vec![statement1];

            // parse a second statement if exists
            if parser.peek_token() != Token::Eof {
                let statement2 = parser.parse_statement();
                stmts.push(statement2);
            }

            let prog = lower_main(&stmts);
            print!("{}", print_program(&prog));
        }

        _ => {
            eprintln!("Unknown subcommand: {}", sub);
            std::process::exit(1);
        }
    }
}

fn validate_args(args: &[String]) -> Result<(), String> {
     if args.len() < 2 {
        eprintln!("Usage: <comp> {{tokenize|parseExpr}} [args..]");
        std::process::exit(1);
    }
    Ok(()) 
    
    // TODO: Implement further argument validation based on cmd 
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    validate_args(&args).unwrap();
}


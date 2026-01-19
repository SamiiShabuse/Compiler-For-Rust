pub mod token;
pub mod tokenizer;
pub mod parser;


pub fn validate_args(args: &[String]) -> Result<(), String> {
     if args.len() < 2 {
        return Err("Usage: <comp> {tokenize|parseExpr} [args..]".to_string());
    }
    Ok(()) 
    
    // TODO: Implement further argument validation based on cmd 
}
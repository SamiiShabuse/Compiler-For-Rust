// validate_args is now in compiler-cli binary
// You can add it to a library if needed for code reuse
/*
use compiler_cli::validate_args;

#[test]
fn test_validate_args_no_args() {
    let args: Vec<String> = vec!["comp".to_string()];
    let result = validate_args(&args);
    assert!(result.is_err());
}

#[test]
fn test_validate_args_with_args() {
    let args: Vec<String> = vec!["comp".to_string(), "tokenize".to_string()];
    let result = validate_args(&args);
    assert!(result.is_ok());
}
*/
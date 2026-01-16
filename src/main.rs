fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Err(err) = compiler_for_rust::validate_args(&args) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}


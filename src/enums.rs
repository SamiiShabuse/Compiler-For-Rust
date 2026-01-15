enum TokenType {
    /*
        right now just going to copy what the Professor has word for word
        Want to understand the format and then just adjust it to match for 
        Rust. 
     */

    // Fixed punctuation
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    CARET,
    AMPERSAND,
    ATSIGN,
    NOT,
    DOT,
    COLON,
    COMMA

    // Key Words 
    THIS,
    IF,
    IFONLY,
    WHILE,
    RETURN,
    PRINT,
    EOF,
    
    // Tokens with data
    OPERATOR,
    NUMBER,
    IDENTIFIER,
}
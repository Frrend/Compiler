use crate::lexer::{Lexer, TokenType};
mod lexer;

fn main() {
    let source = "+- */";
    let mut lexer = Lexer::new(source);

    let mut token = lexer.get_token();
    while token.kind == TokenType::EOF {
        println!("{:?}", token.kind);
        token = lexer.get_token();
    }
}

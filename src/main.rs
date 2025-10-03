use crate::lexer::{Lexer, TokenType};
mod lexer;

fn main() {
    let source = "+- */";
    let mut lexer = Lexer::new(source);
    lexer.next_char();

    let mut token = lexer.get_token();
    while token.kind != TokenType::EOF {
        println!("{:?}", token.kind);
        token = lexer.get_token();
    }
}

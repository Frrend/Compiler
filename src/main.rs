use crate::lexer::Lexer;

mod lexer;

fn main() {
    let source = "LET foobar = 123";
    let mut lexer = Lexer::new(source);

    while lexer.peek() != '\0' {
        println!("{}", lexer.cur_char);
        lexer.next_char();
    }
}

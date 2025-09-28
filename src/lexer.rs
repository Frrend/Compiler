pub struct Lexer {
    source: String,
    pub cur_char: char,
    cur_pos: i32,
    next_char: fn(),
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.to_string() + "\n",
            cur_char: '\0',
            cur_pos: -1,
            next_char: || {},
        }
    }

    pub fn next_char(&mut self) {
        self.cur_pos += 1;
        if self.cur_pos >= self.source.len() as i32 {
            self.cur_char = '\0';
        } else {
            self.cur_char = self
                .source
                .chars()
                .nth(self.cur_pos as usize)
                .expect("Failed to get char");
        }
    }

    pub fn peek(&mut self) -> char {
        if self.cur_pos + 1 > self.source.len() as i32 {
            return '\0';
        }
        let cur_pos = self.cur_pos + 1;
        self.source.chars().nth(cur_pos as usize).unwrap_or('\0')
    }
}

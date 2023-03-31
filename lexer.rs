use std::str::Chars;

use crate::token::Kind;

pub(crate) struct Lexer<'a> {

    pub source: &'a str,

    pub chars: Chars<'a>,

    // pub current: &'a str,

    // ///Source line
    // pub line_num: usize
    
}

impl<'a> Lexer<'a> {
    
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars(),
        }
    }
    

    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }
    fn is_eof(&self) -> bool{
        self.offset() >= self.source.len()
    }

    /// Moves to the next character.
    fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;

        Some(c)
    }

    fn read_str(&self, from: usize, to: usize) -> &'a str {
        &self.source[from..to]
    }

    fn skip_while(&mut self, mut predicate: impl FnMut(char,bool) -> bool){
        let mut escaped = false;

        while !self.is_eof() && predicate(self.peek().unwrap(), escaped) {
            escaped = self.bump() == Some('\\')
        }
    }

}

impl<'a> Iterator for Lexer<'a> {
    type Item = Kind<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.offset();
        let token = match self.bump()? {
            c if c.is_alphabetic() || c == '_' => {
                self.skip_while(|c,_| c.is_alphanumeric() || c == '_');
                let ident = self.read_str(start,self.offset());
                ident.into()
            }

            '0'..='9' => {
                let mut decimal = false;

                self.skip_while(|c,_| {
                    if c.is_ascii_digit() {
                        return true;
                    }

                    if !decimal && c == '.' {
                        decimal = true;
                        return true;
                    }
                    false
                });

                let str_val = self.read_str(start, self.offset());

                if decimal {
                    Kind::Float(str_val)
                }else {
                    Kind::Int(str_val)
                }
            }
            '"' => {
                self.skip_while(|c, esc| c != '"' || esc);

                self.bump()?;

                Kind::String(self.read_str(start + 1, self.offset() - 1))
            }

            c if c.is_whitespace() => return self.next(),

            '=' => {
                if self.peek() == Some('=') {
                    Kind::EqualsEquals
                }else {
                    Kind::Equals  
                }

            }

            '-' => {
                if self.peek() == Some('-'){
                    Kind::MinusMinus
                }else {
                    Kind::Minus
                }
            }
            '+' => {
                if self.peek() == Some('+'){
                    Kind::PlusPlus
                }else {
                    Kind::Plus
                }
            },
            '&' => {
                if self.peek()?  == '&' {
                    Kind::And
                }else {
                    Kind::Amp
                }
            },
            '|' if self.peek()? == '|' => Kind::Or,
            '{' => Kind::LCurlyBrac,
            '}' => Kind::RCurlyBrac,
            '[' => Kind::LSquareBrac,
            ']' => Kind::RSquareBrac,
            '(' => Kind::LBrac,
            ')' => Kind::RBrac,
            '<' => Kind::LAngleBrac,
            '>' => Kind::RAngleBrac,
            '?' => Kind::Ques,
            '!' => Kind::Bang,
            '*' => Kind::Star,
            '/' => Kind::Slash,
            '^' => Kind::Caret,
            '%' => Kind::Percent,
            _ => Kind::Error
        };

        match token {
            Kind::MinusMinus | Kind::EqualsEquals | Kind::PlusPlus | Kind::And | Kind::Or  => self.bump(),
            _ => None,
            
        };
        Some(token)
    }
}

#[cfg(test)]
mod lexer_tests {
    use crate::{token::Kind, lexer::Lexer};

    //test keywords
    #[test]
    fn keyword_test() {    
        let mut scanner = Lexer::new("var for while if else null return true false fun import class");
        assert_eq!(scanner.next().unwrap(), Kind::Var);
        assert_eq!(scanner.next().unwrap(), Kind::For);
        assert_eq!(scanner.next().unwrap(), Kind::While);
        assert_eq!(scanner.next().unwrap(), Kind::If);
        assert_eq!(scanner.next().unwrap(), Kind::Else);
        assert_eq!(scanner.next().unwrap(), Kind::Null);
        assert_eq!(scanner.next().unwrap(), Kind::Return);
        assert_eq!(scanner.next().unwrap(), Kind::True);
        assert_eq!(scanner.next().unwrap(), Kind::False);
        assert_eq!(scanner.next().unwrap(), Kind::Fun);
        assert_eq!(scanner.next().unwrap(), Kind::Import);
        assert_eq!(scanner.next().unwrap(), Kind::Class);
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn number_test(){
        let mut scanner = Lexer::new("3 3.4 3.4444");
        assert_eq!(scanner.next().unwrap(), Kind::Int("3"));
        assert_eq!(scanner.next().unwrap(), Kind::Float("3.4"));
        assert_eq!(scanner.next().unwrap(), Kind::Float("3.4444"))
    }

    #[test]
    fn arithmetic_test() {
        for (input , expected) in [
            ("3*3", vec![Kind::Int("3"), Kind::Star, Kind::Int("3")]),
            ("3-3", vec![Kind::Int("3"), Kind::Minus, Kind::Int("3")]),
            ("3.4+3", vec![Kind::Float("3.4"), Kind::Plus, Kind::Int("3")]),
            ("3/3", vec![Kind::Int("3"), Kind::Slash, Kind::Int("3")]),
            ("3&3", vec![Kind::Int("3"), Kind::Amp, Kind::Int("3")]),
            ("3^3", vec![Kind::Int("3"), Kind::Caret, Kind::Int("3")])
        ]{
            let lexer = Lexer::new(input);

            assert_eq!(expected, lexer.collect::<Vec<Kind>>(),"lexer input: {}", input)
        }
    }
}
use crate::{lexer::Lexer, token::{Token, Kind}, ast::{Program, Node}};

pub struct Parser<'a> {
    ///Source code
    source: &'a str,
    lexer: Lexer<'a>,

    ///Current Token consumed from the lexer
    cur_token: Token<'a>,

    /// The end range of the previous token
    prev_token_end: usize,
}    

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, lexer: Lexer::new(source), cur_token: todo!(), prev_token_end: todo!() }
    }

    fn start_node(&self) ->Node{
        let token = self.cur_token();
        Node::new(token.start,0)
    }

    fn cur_token(&self) -> &Token {
        &self.cur_token
    }

    fn cur_kind(&self) -> Kind {
        self.cur_token.kind
    }

    ///Checks if the current index has token `Kind`
    fn at(&self, kind: Kind) -> bool {
        self.cur_kind() == kind
    }

    /// Advance if we are at `Kind`
    fn bump(&mut self , kind: Kind){
        if self.at(kind) {
            self.advance();
        }
    }

    ///Advance any token
    fn bump_any(&mut self) {
        self.advance();
    }

    ///Advance and return true if we are at `Kind`, otherwise return false
    fn eat(&mut self, kind: Kind) -> bool {
        if self.at(kind) {
            self.advance();
            return true;
        }
        false
    }

    ///Move to the next token
    fn advance(&mut self) {
        let token = self.lexer.read_next_token();
        self.prev_token_end = self.cur_token.end;

        self.cur_token = token;
    }

    pub fn parse(&mut self) -> Program {
        Program {
            node: Node {
                start: 0,
                end: self.source.len(),
            },
            body: vec![]
        }
    }
}
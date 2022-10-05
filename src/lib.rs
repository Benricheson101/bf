use std::io::{stdin, Read};

use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

pub const TAPE_SIZE: usize = 30_000;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct BFParser;

pub struct Runtime {
    ptr: usize,
    tape: [u8; TAPE_SIZE],
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            ptr: 0,
            tape: [0; TAPE_SIZE],
        }
    }

    pub fn run(
        &mut self,
        source: &str,
    ) -> Result<(), pest::error::Error<Rule>> {
        let tokens = BFParser::parse(Rule::file, source)?;
        self._run(tokens);

        Ok(())
    }

    fn _run(&mut self, tokens: Pairs<Rule>) {
        for token in tokens {
            if let Rule::op = token.as_rule() {
                for inner in token.into_inner() {
                    match inner.as_rule() {
                        Rule::add => self.tape[self.ptr] += 1,
                        Rule::sub => self.tape[self.ptr] -= 1,
                        Rule::left => self.ptr -= 1,
                        Rule::right => self.ptr += 1,
                        Rule::write => {
                            print!("{}", self.tape[self.ptr] as char)
                        },
                        Rule::while_loop => {
                            while self.tape[self.ptr] != 0 {
                                let loop_tokens = inner.clone().into_inner();
                                self._run(loop_tokens);
                            }
                        },
                        Rule::read => {
                            let mut buf = [0u8];
                            stdin()
                                .read(&mut buf)
                                .expect("failed to read from stdin");
                            self.tape[self.ptr] = buf[0];
                        },

                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}

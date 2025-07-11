mod lexer;
mod ast;
mod parser;
mod interpreter;


use std::io::{self, BufRead, BufReader};
use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {
   //let mut d = Path::new("main.zc");
    let file = OpenOptions::new().read(true).open("main.zc").expect("couldn't open main.zc");
    let input =io::BufReader::new(file);
    let mut count=1;
    let mut inter=interpreter::Interpreter{variables:std::collections::HashMap::new()};

    for line in input.lines() {
      let line = line.unwrap();
      let mut lex=lexer::Token::tokenize(&line);
      let mut par =parser::Parser{tokens:lex,current_token:0,line:count};

        //println!("{:?}", par.tokens);
        inter.interpret(par.parser(),count);
        count+=1;


  }

    Ok(())
}

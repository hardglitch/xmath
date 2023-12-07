use xmath::parser::parser::Token;
use xmath::parser::tokens::command::{Command, CommandName};
use xmath::parser::tokens::number::Number;

fn main() {

    let token = Token::new(
        1,
        Number::new(2.0),
        0,
    );
    dbg!(token);


    let token = Token::new(
        1,
        Command::new(CommandName::Sin, 23),
        0,
    );
    dbg!(token);
}
mod parser;

use std::error::Error;
use parser::Parser;

fn main() -> Result<(), Box<dyn Error>> {

    let mut parser: Parser = Parser::new("%{word:name} is %{word:gender}, %{int:age} years old and weighs %{int:weight} kilograms".to_string());

    let result = parser.parse("gary is male, 25 years old and weighs 68 kilograms".to_string())?;

    println!("Result: {result}");

    Ok(())
}
mod block;
mod function;
mod variable;

use crate::{
  Environment,
  typechecker::{
    check_expression,
    TTypes,
  },
};

use sflyn_parser::{
  Error,
  Statement,
  Statements,
  tokens::Types,
};

pub fn check_statement(
  statement: &Box<Statements>,
  environment: &mut Environment,
) -> Result<TTypes, Error> {
  // Block
  if let Some(block_stmt) = statement.get_block() {
    return block::check(&block_stmt, environment);
  }

  // Export

  // Expression
  if let Some(expression) = statement.get_expression() {
    return check_expression(&expression.get_expression(), environment);
  }

  // Function
  if let Some(function_stmt) = statement.get_function() {
    return function::check(&function_stmt, environment);
  }

  // If else

  // Import

  // Interface

  // Return
  if let Some(return_s) = statement.get_return() {
    if let Some(value) = return_s.get_value() {
      return check_expression(&value, environment);
    }

    return Ok(TTypes::new_type(Types::VOID, String::from("void"), return_s.get_token()));
  }

  // Variable
  if let Some(variable_stmt) = statement.get_variable() {
    return variable::check(&variable_stmt, environment);
  }

  // Default
  Err(Error::from_token(
    String::from("unknown statement."),
    statement.token(),
  ))
}
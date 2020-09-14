use crate::{
  compiler::{
    Boolean,
    Error,
    Number,
    Objects,
    StringO,
  },
  Environment,
  Store,
};

use sflyn_parser::{
  Expression,
  Infix,
  tokens::Signs,
};

use super::evaluate_expression;

pub fn evaluate(
  infix: &Infix,
  environment: &mut Environment,
) -> Box<Objects> {
  let error = Error::new(
    format!("Unknown infix: {}", infix.string()),
    infix.get_token(),
  );

  // Evaluate left expression.
  let mut left_object = evaluate_expression(&infix.get_left(), environment);

  // Check if the left object is an error.
  if left_object.get_error().is_some() {
    return left_object;
  }

  // Check if the left object is a return.
  if let Some(return_o) = left_object.get_return() {
    left_object = return_o.get_value();
  }

  // Create a new environment.
  let mut right_environment = environment.clone();

  // Set the new store.
  right_environment.store = Store::from_store(environment.store.clone());

  // Check if the infix is a method.
  if infix.is_method() {
    let mut name = "";

    // Check if the left object is a hashmap.
    if let Some(hashmap) = left_object.get_hashmap() {
      // Set the data keys to the new environment.
      for item in hashmap.get_data() {
        right_environment.store.set_object(item.key, item.value);
      }
    } else if left_object.get_number().is_some() {
      name = "Number";
    } else if left_object.get_boolean().is_some() {
      name = "Boolean";
    } else if left_object.get_array().is_some() {
      name = "Array";
    }

    if !name.is_empty() {
      // Get the object from the environment.
      if let Some(obj) = environment.store.get_object(&name.to_string()) {
        if let Some(hashmap) = obj.get_hashmap() {
          // Set the data keys to the new environment.
          for item in hashmap.get_data() {
            right_environment.store.set_object(item.key, item.value);
          }
        }
      }
    }
  }

  // Evaluate right expression.
  let mut right_object = evaluate_expression(&infix.get_right(), &mut right_environment);

  // Check if the right object is an error.
  if right_object.get_error().is_some() {
    return right_object;
  }

  // Check if the right object is a return.
  if let Some(return_o) = right_object.get_return() {
    right_object = return_o.get_value();
  }

  // Parse method.
  if infix.is_method() {
    let right_token = infix.get_right().token();

    if (
      left_object.get_number().is_some() ||
      left_object.get_boolean().is_some() ||
      left_object.get_array().is_some()
    ) && right_token.value == "toString" {
      return crate::compiler::builtins::to_string(infix.get_token(), [left_object].to_vec());
    }

    return right_object;
  }
  // Parse infix.
  else if infix.is_infix() {
    // Check if left and right objects are numbers.
    if left_object.get_number().is_some() &&
      right_object.get_number().is_some() {
      let left_value = left_object.get_number().unwrap().get_value();
      let right_value = right_object.get_number().unwrap().get_value();

      return match infix.get_token().token.get_sign().unwrap() {
        Signs::PLUS => Number::new(left_value + right_value),
        Signs::MINUS => Number::new(left_value - right_value),
        Signs::MULTIPLY => Number::new(left_value * right_value),
        Signs::DIVIDE => Number::new(left_value / right_value),
        Signs::EMPOWERMENT | Signs::CARER => Number::new(left_value.powf(right_value)),
        Signs::MODULE => Number::new(left_value % right_value),
        Signs::LESSTHAN => Boolean::new(left_value < right_value),
        Signs::LESSOREQUALTHAN => Boolean::new(left_value <= right_value),
        Signs::GREATERTHAN => Boolean::new(left_value > right_value),
        Signs::GREATEROREQUALTHAN => Boolean::new(left_value >= right_value),
        Signs::EQUAL => Boolean::new(left_value == right_value),
        Signs::NOTEQUAL => Boolean::new(left_value != right_value),
        _ => error.clone(),
      };
    }
    // Check if left or right object is a string.
    else if infix.get_token().token.expect_sign(&Signs::PLUS) && (
      left_object.get_string().is_some() ||
      right_object.get_string().is_some()
    ) {
      return StringO::new(left_object.string() + &right_object.string());
    }
    // Check if the operator is an equal sign.
    else if infix.get_token().token.expect_sign(&Signs::EQUAL) {
      return Boolean::new(left_object == right_object);
    }
    // Check if the operator is a not equal sign.
    else if infix.get_token().token.expect_sign(&Signs::NOTEQUAL) {
      return Boolean::new(left_object != right_object);
    }
    // Check if the operator is an or sign.
    else if infix.get_token().token.expect_sign(&Signs::OR) {
      // TODO: The rest of the expressions.
      // Null objects.
      let mut return_right = left_object.get_null().is_some();

      // Empty strings
      if let Some(string) = left_object.get_string() {
        return_right = string.get_value().len() == 0;
      }

      // false boolean
      if let Some(boolean) = left_object.get_boolean() {
        return_right = boolean.get_value() == false;
      }

      // Return the object.
      return if return_right { right_object } else { left_object };
    }
    // Check if the operator is an and sign.
    else if infix.get_token().token.expect_sign(&Signs::AND) &&
      left_object.get_boolean().is_some() &&
      right_object.get_boolean().is_some() {
      return Boolean::new(
        left_object.get_boolean().unwrap().get_value() &&
        right_object.get_boolean().unwrap().get_value()
      );
    }
  }

  error
}
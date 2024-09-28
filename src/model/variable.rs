use nom::error::ParseError as _;
use serde::{Deserialize, Serialize};

use crate::model::datatype::DataType;

/// A struct representing a variable with a name, data type, and value.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Variable<T>
where
    T: Clone + PartialEq + Eq + Serialize,
{
    /// The name of the variable.
    pub name: String,
    /// The data type of the variable.
    pub data_type: DataType,
    /// The value of the variable.
    pub value: T,
}

impl<T> Variable<T>
where
    T: Clone + PartialEq + Eq + Serialize + Default,
{
    /// Parses a string input to create a `Variable` instance.
    ///
    /// The input string should be in the format `[data_type]$variable_name`.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that holds the input to be parsed.
    ///
    /// # Returns
    ///
    /// * `nom::IResult<&str, Self, nom::error::VerboseError<&str>>` - A result containing the remaining input and the parsed `Variable` instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the input string does not match the expected format.
    #[allow(dead_code)]
    pub fn parse(input: &str) -> nom::IResult<&str, Self, nom::error::VerboseError<&str>> {
        use nom::{
            bytes::complete::take_while,
            character::complete::char,
            sequence::{preceded, tuple},
        };

        /// Checks if a character is alphanumeric or an underscore.
        ///
        /// # Arguments
        ///
        /// * `c` - The character to check.
        ///
        /// # Returns
        ///
        /// * `bool` - `true` if the character is alphanumeric or an underscore, `false` otherwise.
        fn is_alphanumeric(c: char) -> bool {
            c.is_alphanumeric() || c == '_'
        }

        let parse_name = preceded(char('$'), take_while(is_alphanumeric));

        let (input, (data_type, name)) = tuple((DataType::parse, parse_name))(input)?;

        if name.is_empty() {
            return Err(nom::Err::Error(nom::error::VerboseError::from_error_kind(
                input,
                nom::error::ErrorKind::Verify,
            )));
        }

        Ok((
            input,
            Self {
                name: name.to_string(),
                value: Default::default(),
                data_type,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::datatype::DataType;

    #[test]
    fn test_parse_variable() {
        let input = "[int]$variable_name]";
        let expected = Variable {
            name: "variable_name".to_string(),
            data_type: DataType::I32,
            value: 0,
        };

        let (_, result): (_, Variable<i32>) = Variable::parse(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_variable_with_invalid_input() {
        let input = "invalid_input";
        let result: Result<(_, Variable<i32>), _> = Variable::parse(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_variable_with_different_data_type() {
        let input = "[string]$variable_name]";
        let expected = Variable {
            name: "variable_name".to_string(),
            data_type: DataType::String,
            value: "".to_string(),
        };

        let (_, result): (_, Variable<String>) = Variable::parse(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_variable_with_empty_name() {
        let input = "[int]$]";
        let result: Result<(_, Variable<i32>), _> = Variable::parse(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_variable_with_missing_data_type() {
        let input = "$variable_name]";
        let result: Result<(_, Variable<i32>), _> = Variable::parse(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_variable_with_special_characters_in_name() {
        let input = "[int]$variable_name_123]";
        let expected = Variable {
            name: "variable_name_123".to_string(),
            data_type: DataType::I32,
            value: 0,
        };

        let (_, result): (_, Variable<i32>) = Variable::parse(input).unwrap();
        assert_eq!(result, expected);
    }
}

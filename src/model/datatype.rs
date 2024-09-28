use serde::{Deserialize, Serialize};

/// Represents an array of a specific `DataType`.
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct Array(Box<DataType>);

/// Enum representing various data types.
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum DataType {
    String,
    Bool,
    Switch,
    Ui32,
    Ui64,
    I32,
    I64,
    F32,
    F64,
    Credential,
    SecureString,
    Undefined,
    Unknown,
    Array(Array),
}

impl DataType {
    /// Parses a string input to determine the corresponding `DataType`.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that holds the data type representation.
    ///
    /// # Returns
    ///
    /// * `DataType` - The parsed data type.
    #[allow(dead_code)]
    pub fn parse(input: &str) -> nom::IResult<&str, Self, nom::error::VerboseError<&str>> {
        use nom::{
            branch::alt,
            bytes::complete::tag,
            character::complete::char,
            combinator::map,
            sequence::{preceded, terminated},
            IResult,
        };

        fn parse_data_type(input: &str) -> IResult<&str, DataType, nom::error::VerboseError<&str>> {
            use nom::bytes::complete::tag_no_case;

            alt((
                map(tag_no_case("STRING"), |_| DataType::String),
                map(tag_no_case("BOOL"), |_| DataType::Bool),
                map(tag_no_case("SWITCH"), |_| DataType::Switch),
                map(tag_no_case("UI32"), |_| DataType::Ui32),
                map(tag_no_case("UI64"), |_| DataType::Ui64),
                map(alt((tag_no_case("I32"), tag_no_case("INT"))), |_| {
                    DataType::I32
                }),
                map(tag_no_case("I64"), |_| DataType::I64),
                map(tag_no_case("F32"), |_| DataType::F32),
                map(tag_no_case("F64"), |_| DataType::F64),
                map(tag_no_case("PSCREDENTIAL"), |_| DataType::Credential),
                map(tag_no_case("SECURESTRING"), |_| DataType::SecureString),
            ))(input)
        }
        fn parse_array(input: &str) -> IResult<&str, DataType, nom::error::VerboseError<&str>> {
            map(terminated(parse_data_type, tag("[]")), |dt| {
                DataType::Array(Array(Box::new(dt)))
            })(input)
        }

        if input.is_empty() {
            return Ok(("", Self::Undefined));
        }

        let (input, data_type) = preceded(
            char('['),
            terminated(
                alt((
                    parse_array,
                    parse_data_type,
                    map(tag(""), |_| Self::Undefined),
                )),
                char(']'),
            ),
        )(input)?;

        Ok((input, data_type))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        assert_eq!(DataType::parse("[string]").unwrap().1, DataType::String);
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(DataType::parse("[BOOL]").unwrap().1, DataType::Bool);
    }

    #[test]
    fn test_parse_switch() {
        assert_eq!(DataType::parse("[SWITCH]").unwrap().1, DataType::Switch);
    }

    #[test]
    fn test_parse_ui32() {
        assert_eq!(DataType::parse("[UI32]").unwrap().1, DataType::Ui32);
    }

    #[test]
    fn test_parse_ui64() {
        assert_eq!(DataType::parse("[UI64]").unwrap().1, DataType::Ui64);
    }

    #[test]
    fn test_parse_i32() {
        assert_eq!(DataType::parse("[I32]").unwrap().1, DataType::I32);
    }

    #[test]
    fn test_parse_int() {
        assert_eq!(DataType::parse("[INT]").unwrap().1, DataType::I32);
    }

    #[test]
    fn test_parse_i64() {
        assert_eq!(DataType::parse("[I64]").unwrap().1, DataType::I64);
    }

    #[test]
    fn test_parse_f32() {
        assert_eq!(DataType::parse("[F32]").unwrap().1, DataType::F32);
    }

    #[test]
    fn test_parse_f64() {
        assert_eq!(DataType::parse("[F64]").unwrap().1, DataType::F64);
    }

    #[test]
    fn test_parse_credential() {
        assert_eq!(
            DataType::parse("[PSCREDENTIAL]").unwrap().1,
            DataType::Credential
        );
    }

    #[test]
    fn test_parse_secure_string() {
        assert_eq!(
            DataType::parse("[SECURESTRING]").unwrap().1,
            DataType::SecureString
        );
    }

    #[test]
    fn test_parse_array() {
        assert_eq!(
            DataType::parse("[string[]]").unwrap().1,
            DataType::Array(Array(Box::new(DataType::String)))
        );
        assert!(
            DataType::parse("[[STRING[]]]").is_err(),
            "Invalid array format"
        );
    }

    #[test]
    fn test_parse_undefined() {
        assert_eq!(DataType::parse("[]").unwrap().1, DataType::Undefined);
    }

    #[test]
    fn test_parse_empty_input() {
        assert_eq!(DataType::parse("").unwrap().1, DataType::Undefined);
    }
}

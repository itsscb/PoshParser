use serde::{Deserialize, Serialize};

use crate::model::datatype::DataType;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_until, take_while},
    character::complete::{char, multispace0},
    sequence::{delimited, preceded, tuple},
    IResult,
};

/// Represents a parameter with a name, data type, mandatory flag, and an optional help message.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Parameter {
    /// The name of the parameter.
    pub name: String,
    /// The data type of the parameter.
    pub data_type: DataType,
    /// Indicates whether the parameter is mandatory.
    pub mandatory: Mandatory,
    /// An optional help message describing the parameter.
    pub help_message: Option<String>,
}

impl Parameter {
    /// Parses a string input to create a `Parameter` instance.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that holds the input to be parsed.
    ///
    /// # Returns
    ///
    /// * `IResult<&str, Self, nom::error::VerboseError<&str>>` - The remaining input and the parsed `Parameter`.
    #[allow(dead_code)]
    pub fn parse(input: &str) -> IResult<&str, Self, nom::error::VerboseError<&str>> {
        fn is_alphanumeric(c: char) -> bool {
            c.is_alphanumeric() || c == '_'
        }

        let (input, _) = tag("[Parameter(")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, mandatory) = Mandatory::parse(input)?;
        let (input, _) = char(',')(input)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = tag("HelpMessage = ")(input)?;
        let (input, help_message) = delimited(char('"'), take_until("\""), char('"'))(input)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = tag("]")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, (data_type, name)) = tuple((
            DataType::parse,
            preceded(
                preceded(multispace0, char('$')),
                take_while(is_alphanumeric),
            ),
        ))(input)?;

        Ok((
            input,
            Self {
                data_type,
                mandatory,
                name: name.to_owned(),
                help_message: Some(help_message.to_owned()),
            },
        ))
    }
}

/// Represents whether a parameter is mandatory.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Mandatory(bool);

impl Mandatory {
    /// Parses a string input to create a `Mandatory` instance.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that holds the input to be parsed.
    ///
    /// # Returns
    ///
    /// * `IResult<&str, Self, nom::error::VerboseError<&str>>` - The remaining input and the parsed `Mandatory`.
    pub fn parse(input: &str) -> IResult<&str, Self, nom::error::VerboseError<&str>> {
        let (input, _) = tag_no_case("Mandatory")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, arg) = preceded(char('='), preceded(multispace0, char('$')))(input)?;

        if arg != '$' {
            return Ok((input, Self(true)));
        }

        let (input, value) = alt((tag_no_case("true"), tag_no_case("false")))(input)?;

        Ok((input, Self(value.to_lowercase() == "true")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::datatype::DataType;

    #[test]
    fn test_parse_parameter() {
        let input = r#"[Parameter(
                Mandatory = $true,
                HelpMessage = "The name of the person."
            ]
            [string]$Test"#;

        let want = Parameter {
            name: "Test".to_string(),
            data_type: DataType::String,
            mandatory: Mandatory(true),
            help_message: Some("The name of the person.".to_string()),
        };

        let got = Parameter::parse(input);

        assert_eq!(got.unwrap().1, want);
    }

    #[test]
    fn test_parse_mandatory_true() {
        let input = "Mandatory = $true";
        let want = Mandatory(true);
        let got = Mandatory::parse(input);
        assert_eq!(got.unwrap().1, want);
    }

    #[test]
    fn test_parse_mandatory_false() {
        let input = "Mandatory = $false";
        let want = Mandatory(false);
        let got = Mandatory::parse(input);
        assert_eq!(got.unwrap().1, want);
    }

    #[test]
    fn test_parse_mandatory_no_dollar() {
        let input = "Mandatory = true";
        let got = Mandatory::parse(input);
        assert!(got.is_err());
    }
}

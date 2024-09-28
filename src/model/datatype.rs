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
    pub fn parse(input: &str) -> Self {
        if input.is_empty() {
            return Self::Undefined;
        }

        if !input.starts_with('[') {
            return Self::Unknown;
        }

        if !input.ends_with(']') {
            return Self::Unknown;
        }

        let mut is_array = false;
        let input: String = if input.ends_with("[]]") {
            is_array = true;
            let i = input
                .chars()
                .rev()
                .skip(3)
                .collect::<String>()
                .chars()
                .rev()
                .collect::<String>();
            i + "]"
        } else {
            input.to_owned()
        };
        let dt = match input.to_uppercase().as_str() {
            "[STRING]" => Self::String,
            "[BOOL]" => Self::Bool,
            "[SWITCH]" => Self::Switch,
            "[UI32]" => Self::Ui32,
            "[UI64]" => Self::Ui64,
            "[I32]" | "[INT]" => Self::I32,
            "[I64]" => Self::I64,
            "[F32]" => Self::F32,
            "[F64]" => Self::F64,
            "[PSCREDENTIAL]" => Self::Credential,
            "[SECURESTRING]" => Self::SecureString,
            "" => Self::Undefined,
            _ => Self::Unknown,
        };

        if is_array {
            Self::Array(Array(Box::new(dt)))
        } else {
            dt
        }
    }
}

impl std::str::FromStr for DataType {
    type Err = ();

    /// Converts a string slice to a `DataType`.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that holds the data type representation.
    ///
    /// # Returns
    ///
    /// * `Result<DataType, ()>` - The parsed data type or an error.
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!("[string]".parse::<DataType>(), Ok(DataType::String));
        assert_eq!("[bool]".parse::<DataType>(), Ok(DataType::Bool));
        assert_eq!("[switch]".parse::<DataType>(), Ok(DataType::Switch));
        assert_eq!("[ui32]".parse::<DataType>(), Ok(DataType::Ui32));
        assert_eq!("[ui64]".parse::<DataType>(), Ok(DataType::Ui64));
        assert_eq!("[i32]".parse::<DataType>(), Ok(DataType::I32));
        assert_eq!("[i64]".parse::<DataType>(), Ok(DataType::I64));
        assert_eq!("[f32]".parse::<DataType>(), Ok(DataType::F32));
        assert_eq!("[f64]".parse::<DataType>(), Ok(DataType::F64));
        assert_eq!(
            "[pscredential]".parse::<DataType>(),
            Ok(DataType::Credential)
        );
        assert_eq!(
            "[SECURESTRING]".parse::<DataType>(),
            Ok(DataType::SecureString)
        );
        assert_eq!("".parse::<DataType>(), Ok(DataType::Undefined));
        assert_eq!("UNKNOWN".parse::<DataType>(), Ok(DataType::Unknown));
        assert_eq!("SOMETHING_ELSE".parse::<DataType>(), Ok(DataType::Unknown));
        assert_eq!(
            "[STRING[]]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::String))))
        );
        assert_eq!(
            "[BOOL[]]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::Bool))))
        );
        assert_eq!(
            "[SWITCH[]]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::Switch))))
        );
        assert_eq!(
            "[UI32[]]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::Ui32))))
        );
        assert_eq!(
            "[UI64[]]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::Ui64))))
        );
        assert_eq!(
            "[I32[]]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::I32))))
        );
        assert_eq!(
            "[I64[]]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::I64))))
        );
        assert_eq!(
            "[F32[]]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::F32))))
        );
        assert_eq!(
            "[F64[]]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::F64))))
        );
        assert_eq!(
            "[psCREDENTIAL[]]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::Credential))))
        );
        assert_eq!(
            "[UNKNOWN[]]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::Unknown))))
        );
        assert_eq!(
            "[SOMETHING_ELSE[]]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::Unknown))))
        );
    }
}

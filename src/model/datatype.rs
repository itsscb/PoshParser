use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct Array(Box<DataType>);

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
    Undefined,
    Unknown,
    Array(Array),
}

impl std::str::FromStr for DataType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut is_array = false;
        let input: String = if input.ends_with("[]") {
            is_array = true;
            input
                .chars()
                .rev()
                .skip(2)
                .collect::<String>()
                .chars()
                .rev()
                .collect::<String>()
        } else {
            input.to_owned()
        };
        let dt = match input.as_str() {
            "STRING" => Self::String,
            "BOOL" => Self::Bool,
            "SWITCH" => Self::Switch,
            "UI32" => Self::Ui32,
            "UI64" => Self::Ui64,
            "I32" => Self::I32,
            "I64" => Self::I64,
            "F32" => Self::F32,
            "F64" => Self::F64,
            "CREDENTIAL" => Self::Credential,
            "" => Self::Undefined,
            _ => Self::Unknown,
        };

        if is_array {
            Ok(Self::Array(Array(Box::new(dt))))
        } else {
            Ok(dt)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!("STRING".parse::<DataType>(), Ok(DataType::String));
        assert_eq!("BOOL".parse::<DataType>(), Ok(DataType::Bool));
        assert_eq!("SWITCH".parse::<DataType>(), Ok(DataType::Switch));
        assert_eq!("UI32".parse::<DataType>(), Ok(DataType::Ui32));
        assert_eq!("UI64".parse::<DataType>(), Ok(DataType::Ui64));
        assert_eq!("I32".parse::<DataType>(), Ok(DataType::I32));
        assert_eq!("I64".parse::<DataType>(), Ok(DataType::I64));
        assert_eq!("F32".parse::<DataType>(), Ok(DataType::F32));
        assert_eq!("F64".parse::<DataType>(), Ok(DataType::F64));
        assert_eq!("CREDENTIAL".parse::<DataType>(), Ok(DataType::Credential));
        assert_eq!("".parse::<DataType>(), Ok(DataType::Undefined));
        assert_eq!("UNKNOWN".parse::<DataType>(), Ok(DataType::Unknown));
        assert_eq!("SOMETHING_ELSE".parse::<DataType>(), Ok(DataType::Unknown));
        assert_eq!(
            "STRING[]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::String))))
        );
        assert_eq!(
            "BOOL[]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::Bool))))
        );
        assert_eq!(
            "SWITCH[]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::Switch))))
        );
        assert_eq!(
            "UI32[]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::Ui32))))
        );
        assert_eq!(
            "UI64[]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::Ui64))))
        );
        assert_eq!(
            "I32[]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::I32))))
        );
        assert_eq!(
            "I64[]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::I64))))
        );
        assert_eq!(
            "F32[]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::F32))))
        );
        assert_eq!(
            "F64[]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::F64))))
        );
        assert_eq!(
            "CREDENTIAL[]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::Credential))))
        );
        assert_eq!(
            "UNKNOWN[]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::Unknown))))
        );
        assert_eq!(
            "SOMETHING_ELSE[]".parse::<DataType>(),
            Ok(DataType::Array(Array(Box::new(DataType::Unknown))))
        );
    }
}

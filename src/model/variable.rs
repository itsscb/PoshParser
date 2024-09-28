use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Variable<T>
where
    T: Clone + PartialEq + Eq + Serialize,
{
    pub name: String,
    pub value: T,
}

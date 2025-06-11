use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload {
    pub name: String,
    pub data: Option<Value>,
    pub success: bool 
}

impl Payload {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: None,
            success: true
        }
    }

    pub fn with_data<T: Serialize>(name: impl Into<String>, data: &T) -> Self {
        let value = serde_json::to_value(data).expect("Invalid input serialization");
        Self {
            name: name.into(),
            data: Some(value),
            success: true
        }
    }

    pub fn input_as<T: for<'de> Deserialize<'de>>(&self) -> Option<T> {
        self.data
            .as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    pub fn op_name(&self) -> Option<&str> {
        if let Some( (_, op_name)) = self.name.split_once('.') {
            Some(op_name)
        }else {
            None
        }
    }

    pub fn op_name_unchecked(&self) -> &str {
        self.op_name().expect("Operation name not defined")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeResult {
    pub operation: String,
    pub success: bool,
    pub data: Option<Value>,
}
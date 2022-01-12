use std::collections::HashMap;

use aws_sdk_dynamodb::model::AttributeValue;

pub trait AttributeValueExt {
    fn get_s(&self, key: &str) -> Option<String>;
    fn get_n(&self, key: &str) -> Option<f64>;
}

impl AttributeValueExt for HashMap<String, AttributeValue> {
    fn get_s(&self, key: &str) -> Option<String> {
        Some(self.get(key)?.as_s().ok()?.to_owned())
    }

    fn get_n(&self, key: &str) -> Option<f64> {
        self.get(key)?.as_n().ok()?.parse::<f64>().ok()
    }
}
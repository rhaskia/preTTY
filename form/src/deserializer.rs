use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

use dioxus::html::FormValue;
use serde::de::MapAccess;
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};

use crate::Error;

pub fn to_value(mut values: HashMap<String, FormValue>) -> Value {
    // let mut values = values.iter().map(|(key, value)| (key, FormInter::Value(value))).collect();
    let mut result = Map::new();
    for (key, value) in values {
        let mut tree = key.split('.').collect::<Vec<&str>>();
        let t = tree.pop().unwrap();
        let last = tree.pop().unwrap();
        let mut current = &mut result;

        for branch in tree {
            if branch.ends_with(']') { 

            } else {

            }
            current = result
                .entry(branch.to_string())
                .or_insert(Value::Object(Map::new()))
                .as_object_mut().unwrap();
            println!("{current:?}");
        }

        let v = match t {
            "s" => Value::String(value.0[0].clone()),
            "b" => Value::Bool(value.0[0] == "true"),
            "n" => Value::Number(FromStr::from_str(&value.0[0]).unwrap()),
            _ => Value::Array(value.0.into_iter().map(|s| Value::String(s)).collect()),
        };
        current.insert(last.to_string(), v);
    }

    Value::Object(result)
}

pub fn from_values<'a, T>(values: HashMap<String, FormValue>) -> Result<T, Error>
where
    T: for<'de> Deserialize<'de>,
{
    println!("Form {values:?}");
    let value = to_value(values.clone());
    println!("RAWVALUES {value:?}");
    let t = serde_json::from_value(value);
    Ok(t.unwrap())
}

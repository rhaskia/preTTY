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
    let mut result = Value::Object(Map::new());

    for (key, value) in values {
        let mut tree = key.split('.').collect::<Vec<&str>>();
        let t = tree.pop().unwrap();
        let mut last = tree.pop().unwrap().to_string();
        let mut current = &mut result;

        for branch in tree {
            let mut branch = branch.to_string();
            if branch.ends_with(']') {
                branch.pop();
                let mut n = String::new();
                while let Some(ch) = branch.pop() { if ch == '[' { break; }; n.push(ch); }
                let number: usize = n.parse().unwrap();

                current = current
                    .as_object_mut()
                    .unwrap()
                    .entry(branch.to_string())
                    .or_insert(Value::Array(Vec::new()));

                let arr = current.as_array_mut().unwrap();
                if arr.len() >= number { arr.push(Value::Object(Map::new())); }
                current = &mut arr[number];
            } else {
                // REDO
                current = current
                    .as_object_mut()
                    .unwrap()
                    .entry(branch.to_string())
                    .or_insert(Value::Object(Map::new()))
            }
        }

        let v = match t {
            "s" => Value::String(value.0[0].clone()),
            "b" => Value::Bool(value.0[0] == "on"),
            "n" => Value::Number(FromStr::from_str(&value.0[0]).unwrap()),
            _ => Value::Array(value.0.into_iter().map(|s| Value::String(s)).collect()),
        };

        if last.ends_with(']') {
            last.pop();
            let mut n = String::new();
            while let Some(ch) = last.pop() { if ch == '[' { break; }; n.push(ch); }
            let number: usize = n.parse().unwrap();

            current = current
                .as_object_mut()
                .unwrap()
                .entry(last.to_string())
                .or_insert(Value::Array(Vec::new()));

            let arr = current.as_array_mut().unwrap();
            if number >= arr.len() { arr.resize(number + 1, Value::Null); }
            arr[number] = v;
        } else {
            current.as_object_mut().unwrap().insert(last.to_string(), v);
        }
    }

    result
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

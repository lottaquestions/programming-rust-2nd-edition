#![recursion_limit = "256"]
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

impl From<bool> for Json {
    fn from(value: bool) -> Json {
        Json::Boolean(value)
    }
}
/* Numbers should look like the below, but since they are transformed into the same destination
type, we define a macro to generate the code for all number types
see: impl_from_num_for_json
impl From<i32> for Json {
    fn from(value: i32) -> Json {
        Json::Number(value as f64)
    }
}
*/
impl From<String> for Json {
    fn from(value: String) -> Json {
        Json::String(value)
    }
}
impl<'a> From<&'a str> for Json {
    fn from(value: &'a str) -> Json {
        Json::String(value.to_string())
    }
}

// Note the ty for types in the macro below
macro_rules! impl_from_num_for_json {
    ($( $t:ty )*) => {
        $(
        impl From<$t> for Json {
            fn from(n: $t) -> Json {
                Json::Number(n as f64)
            }
        }
        )*
    };
}

impl_from_num_for_json!(u8 i8 u16  i16 u32 i32 u64 i64 u128 i128 usize isize f32 f64);

macro_rules! json {
    (null) => {
        Json::Null
    };
    ([ $( $element:tt), * ]) => {Json::Array(vec![ $( json!($element) ),* ])};
    ({ $( $key:tt: $value:tt ), * }) => {Json::Object(
        Box::new(vec![ $( ($key.to_string(), json!($value)) ),*].into_iter().collect())
    )};
    ( $other:tt ) => {
        // Return number, string or boolean
        Json::from($other)
    };
}

fn main() {
    let width = 4.0;
    let desc = json!({
        "width": width,
        "height": (width * 9.0 / 4.0)
    });
    println!("{:?}", desc);
}

#[test]
fn json_null() {
    assert_eq!(json!(null), Json::Null);
}

#[test]
fn json_array_with_json_element() {
    let macro_generated_value = json!([
        // Valid json
        {
            "pitch" : 440.0
        }
    ]);

    let hand_coded_value = Json::Array(vec![Json::Object(Box::new(
        vec![("pitch".to_string(), Json::Number(440.0))]
            .into_iter()
            .collect(),
    ))]);

    assert_eq!(macro_generated_value, hand_coded_value);
}

#![recursion_limit = "256"]
pub use std::collections::HashMap;
pub use std::boxed::Box;
pub use std::string::ToString;

#[derive(Clone, PartialEq, Debug)]
pub enum Json {
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

#[macro_export]
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

#[macro_export]
macro_rules! json_v2 {
    (null) => {
        $crate::Json::Null
    };
    ([ $( $element:tt ),* ]) => {
        $crate::Json::Array(vec![ $( json_v2!($element)),*])
    };
    ( { $( $key:tt : $value:tt), *}) => {
        let mut fields = $crate::macros::Box::new(
            $crate::macros::HashMap::new()
        );
        $(
            fields.insert($crate::macros::ToString::to_string($key), json_v2!($value));
        )*
        $crate::Json::Object(fields)
    };
    ($other:tt) => {
        $crate::Json::from($other)
    };
}
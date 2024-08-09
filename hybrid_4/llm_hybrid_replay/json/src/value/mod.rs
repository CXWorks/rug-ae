//! The Value enum, a loosely typed way of representing any valid JSON value.
//!
//! # Constructing JSON
//!
//! Serde JSON provides a [`json!` macro][macro] to build `serde_json::Value`
//! objects with very natural JSON syntax.
//!
//! ```
//! use serde_json::json;
//!
//! fn main() {
//!     // The type of `john` is `serde_json::Value`
//!     let john = json!({
//!         "name": "John Doe",
//!         "age": 43,
//!         "phones": [
//!             "+44 1234567",
//!             "+44 2345678"
//!         ]
//!     });
//!
//!     println!("first phone number: {}", john["phones"][0]);
//!
//!     // Convert to a string of JSON and print it out
//!     println!("{}", john.to_string());
//! }
//! ```
//!
//! The `Value::to_string()` function converts a `serde_json::Value` into a
//! `String` of JSON text.
//!
//! One neat thing about the `json!` macro is that variables and expressions can
//! be interpolated directly into the JSON value as you are building it. Serde
//! will check at compile time that the value you are interpolating is able to
//! be represented as JSON.
//!
//! ```
//! # use serde_json::json;
//! #
//! # fn random_phone() -> u16 { 0 }
//! #
//! let full_name = "John Doe";
//! let age_last_year = 42;
//!
//! // The type of `john` is `serde_json::Value`
//! let john = json!({
//!     "name": full_name,
//!     "age": age_last_year + 1,
//!     "phones": [
//!         format!("+44 {}", random_phone())
//!     ]
//! });
//! ```
//!
//! A string of JSON data can be parsed into a `serde_json::Value` by the
//! [`serde_json::from_str`][from_str] function. There is also
//! [`from_slice`][from_slice] for parsing from a byte slice `&[u8]` and
//! [`from_reader`][from_reader] for parsing from any `io::Read` like a File or
//! a TCP stream.
//!
//! ```
//! use serde_json::{json, Value, Error};
//!
//! fn untyped_example() -> Result<(), Error> {
//!     // Some JSON input data as a &str. Maybe this comes from the user.
//!     let data = r#"
//!         {
//!             "name": "John Doe",
//!             "age": 43,
//!             "phones": [
//!                 "+44 1234567",
//!                 "+44 2345678"
//!             ]
//!         }"#;
//!
//!     // Parse the string of data into serde_json::Value.
//!     let v: Value = serde_json::from_str(data)?;
//!
//!     // Access parts of the data by indexing with square brackets.
//!     println!("Please call {} at the number {}", v["name"], v["phones"][0]);
//!
//!     Ok(())
//! }
//! #
//! # untyped_example().unwrap();
//! ```
//!
//! [macro]: crate::json
//! [from_str]: crate::de::from_str
//! [from_slice]: crate::de::from_slice
//! [from_reader]: crate::de::from_reader
use crate::error::Error;
use crate::io;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{self, Debug, Display};
use core::mem;
use core::str;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
pub use self::index::Index;
pub use self::ser::Serializer;
pub use crate::map::Map;
pub use crate::number::Number;
#[cfg(feature = "raw_value")]
pub use crate::raw::{to_raw_value, RawValue};
/// Represents any valid JSON value.
///
/// See the [`serde_json::value` module documentation](self) for usage examples.
#[derive(Clone, Eq, PartialEq)]
pub enum Value {
    /// Represents a JSON null value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(null);
    /// ```
    Null,
    /// Represents a JSON boolean.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(true);
    /// ```
    Bool(bool),
    /// Represents a JSON number, whether integer or floating point.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(12.5);
    /// ```
    Number(Number),
    /// Represents a JSON string.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!("a string");
    /// ```
    String(String),
    /// Represents a JSON array.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(["an", "array"]);
    /// ```
    Array(Vec<Value>),
    /// Represents a JSON object.
    ///
    /// By default the map is backed by a BTreeMap. Enable the `preserve_order`
    /// feature of serde_json to use IndexMap instead, which preserves
    /// entries in the order they are inserted into the map. In particular, this
    /// allows JSON data to be deserialized into a Value and serialized to a
    /// string while retaining the order of map keys in the input.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "an": "object" });
    /// ```
    Object(Map<String, Value>),
}
impl Debug for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => formatter.write_str("Null"),
            Value::Bool(boolean) => write!(formatter, "Bool({})", boolean),
            Value::Number(number) => Debug::fmt(number, formatter),
            Value::String(string) => write!(formatter, "String({:?})", string),
            Value::Array(vec) => {
                formatter.write_str("Array ")?;
                Debug::fmt(vec, formatter)
            }
            Value::Object(map) => {
                formatter.write_str("Object ")?;
                Debug::fmt(map, formatter)
            }
        }
    }
}
impl Display for Value {
    /// Display a JSON value as a string.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let json = json!({ "city": "London", "street": "10 Downing Street" });
    ///
    /// // Compact format:
    /// //
    /// // {"city":"London","street":"10 Downing Street"}
    /// let compact = format!("{}", json);
    /// assert_eq!(compact,
    ///     "{\"city\":\"London\",\"street\":\"10 Downing Street\"}");
    ///
    /// // Pretty format:
    /// //
    /// // {
    /// //   "city": "London",
    /// //   "street": "10 Downing Street"
    /// // }
    /// let pretty = format!("{:#}", json);
    /// assert_eq!(pretty,
    ///     "{\n  \"city\": \"London\",\n  \"street\": \"10 Downing Street\"\n}");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct WriterFormatter<'a, 'b: 'a> {
            inner: &'a mut fmt::Formatter<'b>,
        }
        impl<'a, 'b> io::Write for WriterFormatter<'a, 'b> {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                let s = unsafe { str::from_utf8_unchecked(buf) };
                tri!(self.inner.write_str(s).map_err(io_error));
                Ok(buf.len())
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        fn io_error(_: fmt::Error) -> io::Error {
            io::Error::new(io::ErrorKind::Other, "fmt error")
        }
        let alternate = f.alternate();
        let mut wr = WriterFormatter { inner: f };
        if alternate {
            super::ser::to_writer_pretty(&mut wr, self).map_err(|_| fmt::Error)
        } else {
            super::ser::to_writer(&mut wr, self).map_err(|_| fmt::Error)
        }
    }
}
fn parse_index(s: &str) -> Option<usize> {
    if s.starts_with('+') || (s.starts_with('0') && s.len() != 1) {
        return None;
    }
    s.parse().ok()
}
impl Value {
    /// Index into a JSON array or map. A string index can be used to access a
    /// value in a map, and a usize index can be used to access an element of an
    /// array.
    ///
    /// Returns `None` if the type of `self` does not match the type of the
    /// index, for example if the index is a string and `self` is an array or a
    /// number. Also returns `None` if the given key does not exist in the map
    /// or the given index is not within the bounds of the array.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let object = json!({ "A": 65, "B": 66, "C": 67 });
    /// assert_eq!(*object.get("A").unwrap(), json!(65));
    ///
    /// let array = json!([ "A", "B", "C" ]);
    /// assert_eq!(*array.get(2).unwrap(), json!("C"));
    ///
    /// assert_eq!(array.get("A"), None);
    /// ```
    ///
    /// Square brackets can also be used to index into a value in a more concise
    /// way. This returns `Value::Null` in cases where `get` would have returned
    /// `None`.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let object = json!({
    ///     "A": ["a", "á", "à"],
    ///     "B": ["b", "b́"],
    ///     "C": ["c", "ć", "ć̣", "ḉ"],
    /// });
    /// assert_eq!(object["B"][0], json!("b"));
    ///
    /// assert_eq!(object["D"], json!(null));
    /// assert_eq!(object[0]["x"]["y"]["z"], json!(null));
    /// ```
    pub fn get<I: Index>(&self, index: I) -> Option<&Value> {
        index.index_into(self)
    }
    /// Mutably index into a JSON array or map. A string index can be used to
    /// access a value in a map, and a usize index can be used to access an
    /// element of an array.
    ///
    /// Returns `None` if the type of `self` does not match the type of the
    /// index, for example if the index is a string and `self` is an array or a
    /// number. Also returns `None` if the given key does not exist in the map
    /// or the given index is not within the bounds of the array.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let mut object = json!({ "A": 65, "B": 66, "C": 67 });
    /// *object.get_mut("A").unwrap() = json!(69);
    ///
    /// let mut array = json!([ "A", "B", "C" ]);
    /// *array.get_mut(2).unwrap() = json!("D");
    /// ```
    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Value> {
        index.index_into_mut(self)
    }
    /// Returns true if the `Value` is an Object. Returns false otherwise.
    ///
    /// For any Value on which `is_object` returns true, `as_object` and
    /// `as_object_mut` are guaranteed to return the map representation of the
    /// object.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let obj = json!({ "a": { "nested": true }, "b": ["an", "array"] });
    ///
    /// assert!(obj.is_object());
    /// assert!(obj["a"].is_object());
    ///
    /// // array, not an object
    /// assert!(!obj["b"].is_object());
    /// ```
    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }
    /// If the `Value` is an Object, returns the associated Map. Returns None
    /// otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": { "nested": true }, "b": ["an", "array"] });
    ///
    /// // The length of `{"nested": true}` is 1 entry.
    /// assert_eq!(v["a"].as_object().unwrap().len(), 1);
    ///
    /// // The array `["an", "array"]` is not an object.
    /// assert_eq!(v["b"].as_object(), None);
    /// ```
    pub fn as_object(&self) -> Option<&Map<String, Value>> {
        match self {
            Value::Object(map) => Some(map),
            _ => None,
        }
    }
    /// If the `Value` is an Object, returns the associated mutable Map.
    /// Returns None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let mut v = json!({ "a": { "nested": true } });
    ///
    /// v["a"].as_object_mut().unwrap().clear();
    /// assert_eq!(v, json!({ "a": {} }));
    /// ```
    pub fn as_object_mut(&mut self) -> Option<&mut Map<String, Value>> {
        match self {
            Value::Object(map) => Some(map),
            _ => None,
        }
    }
    /// Returns true if the `Value` is an Array. Returns false otherwise.
    ///
    /// For any Value on which `is_array` returns true, `as_array` and
    /// `as_array_mut` are guaranteed to return the vector representing the
    /// array.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let obj = json!({ "a": ["an", "array"], "b": { "an": "object" } });
    ///
    /// assert!(obj["a"].is_array());
    ///
    /// // an object, not an array
    /// assert!(!obj["b"].is_array());
    /// ```
    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }
    /// If the `Value` is an Array, returns the associated vector. Returns None
    /// otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": ["an", "array"], "b": { "an": "object" } });
    ///
    /// // The length of `["an", "array"]` is 2 elements.
    /// assert_eq!(v["a"].as_array().unwrap().len(), 2);
    ///
    /// // The object `{"an": "object"}` is not an array.
    /// assert_eq!(v["b"].as_array(), None);
    /// ```
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(array) => Some(array),
            _ => None,
        }
    }
    /// If the `Value` is an Array, returns the associated mutable vector.
    /// Returns None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let mut v = json!({ "a": ["an", "array"] });
    ///
    /// v["a"].as_array_mut().unwrap().clear();
    /// assert_eq!(v, json!({ "a": [] }));
    /// ```
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Value::Array(list) => Some(list),
            _ => None,
        }
    }
    /// Returns true if the `Value` is a String. Returns false otherwise.
    ///
    /// For any Value on which `is_string` returns true, `as_str` is guaranteed
    /// to return the string slice.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": "some string", "b": false });
    ///
    /// assert!(v["a"].is_string());
    ///
    /// // The boolean `false` is not a string.
    /// assert!(!v["b"].is_string());
    /// ```
    pub fn is_string(&self) -> bool {
        self.as_str().is_some()
    }
    /// If the `Value` is a String, returns the associated str. Returns None
    /// otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": "some string", "b": false });
    ///
    /// assert_eq!(v["a"].as_str(), Some("some string"));
    ///
    /// // The boolean `false` is not a string.
    /// assert_eq!(v["b"].as_str(), None);
    ///
    /// // JSON values are printed in JSON representation, so strings are in quotes.
    /// //
    /// //    The value is: "some string"
    /// println!("The value is: {}", v["a"]);
    ///
    /// // Rust strings are printed without quotes.
    /// //
    /// //    The value is: some string
    /// println!("The value is: {}", v["a"].as_str().unwrap());
    /// ```
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
    /// Returns true if the `Value` is a Number. Returns false otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 1, "b": "2" });
    ///
    /// assert!(v["a"].is_number());
    ///
    /// // The string `"2"` is a string, not a number.
    /// assert!(!v["b"].is_number());
    /// ```
    pub fn is_number(&self) -> bool {
        match *self {
            Value::Number(_) => true,
            _ => false,
        }
    }
    /// Returns true if the `Value` is an integer between `i64::MIN` and
    /// `i64::MAX`.
    ///
    /// For any Value on which `is_i64` returns true, `as_i64` is guaranteed to
    /// return the integer value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let big = i64::max_value() as u64 + 10;
    /// let v = json!({ "a": 64, "b": big, "c": 256.0 });
    ///
    /// assert!(v["a"].is_i64());
    ///
    /// // Greater than i64::MAX.
    /// assert!(!v["b"].is_i64());
    ///
    /// // Numbers with a decimal point are not considered integers.
    /// assert!(!v["c"].is_i64());
    /// ```
    pub fn is_i64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_i64(),
            _ => false,
        }
    }
    /// Returns true if the `Value` is an integer between zero and `u64::MAX`.
    ///
    /// For any Value on which `is_u64` returns true, `as_u64` is guaranteed to
    /// return the integer value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 64, "b": -64, "c": 256.0 });
    ///
    /// assert!(v["a"].is_u64());
    ///
    /// // Negative integer.
    /// assert!(!v["b"].is_u64());
    ///
    /// // Numbers with a decimal point are not considered integers.
    /// assert!(!v["c"].is_u64());
    /// ```
    pub fn is_u64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_u64(),
            _ => false,
        }
    }
    /// Returns true if the `Value` is a number that can be represented by f64.
    ///
    /// For any Value on which `is_f64` returns true, `as_f64` is guaranteed to
    /// return the floating point value.
    ///
    /// Currently this function returns true if and only if both `is_i64` and
    /// `is_u64` return false but this is not a guarantee in the future.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 256.0, "b": 64, "c": -64 });
    ///
    /// assert!(v["a"].is_f64());
    ///
    /// // Integers.
    /// assert!(!v["b"].is_f64());
    /// assert!(!v["c"].is_f64());
    /// ```
    pub fn is_f64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_f64(),
            _ => false,
        }
    }
    /// If the `Value` is an integer, represent it as i64 if possible. Returns
    /// None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let big = i64::max_value() as u64 + 10;
    /// let v = json!({ "a": 64, "b": big, "c": 256.0 });
    ///
    /// assert_eq!(v["a"].as_i64(), Some(64));
    /// assert_eq!(v["b"].as_i64(), None);
    /// assert_eq!(v["c"].as_i64(), None);
    /// ```
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Number(n) => n.as_i64(),
            _ => None,
        }
    }
    /// If the `Value` is an integer, represent it as u64 if possible. Returns
    /// None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 64, "b": -64, "c": 256.0 });
    ///
    /// assert_eq!(v["a"].as_u64(), Some(64));
    /// assert_eq!(v["b"].as_u64(), None);
    /// assert_eq!(v["c"].as_u64(), None);
    /// ```
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Value::Number(n) => n.as_u64(),
            _ => None,
        }
    }
    /// If the `Value` is a number, represent it as f64 if possible. Returns
    /// None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 256.0, "b": 64, "c": -64 });
    ///
    /// assert_eq!(v["a"].as_f64(), Some(256.0));
    /// assert_eq!(v["b"].as_f64(), Some(64.0));
    /// assert_eq!(v["c"].as_f64(), Some(-64.0));
    /// ```
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Number(n) => n.as_f64(),
            _ => None,
        }
    }
    /// Returns true if the `Value` is a Boolean. Returns false otherwise.
    ///
    /// For any Value on which `is_boolean` returns true, `as_bool` is
    /// guaranteed to return the boolean value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": false, "b": "false" });
    ///
    /// assert!(v["a"].is_boolean());
    ///
    /// // The string `"false"` is a string, not a boolean.
    /// assert!(!v["b"].is_boolean());
    /// ```
    pub fn is_boolean(&self) -> bool {
        self.as_bool().is_some()
    }
    /// If the `Value` is a Boolean, returns the associated bool. Returns None
    /// otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": false, "b": "false" });
    ///
    /// assert_eq!(v["a"].as_bool(), Some(false));
    ///
    /// // The string `"false"` is a string, not a boolean.
    /// assert_eq!(v["b"].as_bool(), None);
    /// ```
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }
    /// Returns true if the `Value` is a Null. Returns false otherwise.
    ///
    /// For any Value on which `is_null` returns true, `as_null` is guaranteed
    /// to return `Some(())`.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": null, "b": false });
    ///
    /// assert!(v["a"].is_null());
    ///
    /// // The boolean `false` is not null.
    /// assert!(!v["b"].is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        self.as_null().is_some()
    }
    /// If the `Value` is a Null, returns (). Returns None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": null, "b": false });
    ///
    /// assert_eq!(v["a"].as_null(), Some(()));
    ///
    /// // The boolean `false` is not null.
    /// assert_eq!(v["b"].as_null(), None);
    /// ```
    pub fn as_null(&self) -> Option<()> {
        match *self {
            Value::Null => Some(()),
            _ => None,
        }
    }
    /// Looks up a value by a JSON Pointer.
    ///
    /// JSON Pointer defines a string syntax for identifying a specific value
    /// within a JavaScript Object Notation (JSON) document.
    ///
    /// A Pointer is a Unicode string with the reference tokens separated by `/`.
    /// Inside tokens `/` is replaced by `~1` and `~` is replaced by `~0`. The
    /// addressed value is returned and if there is no such value `None` is
    /// returned.
    ///
    /// For more information read [RFC6901](https://tools.ietf.org/html/rfc6901).
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let data = json!({
    ///     "x": {
    ///         "y": ["z", "zz"]
    ///     }
    /// });
    ///
    /// assert_eq!(data.pointer("/x/y/1").unwrap(), &json!("zz"));
    /// assert_eq!(data.pointer("/a/b/c"), None);
    /// ```
    pub fn pointer(&self, pointer: &str) -> Option<&Value> {
        if pointer.is_empty() {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        pointer
            .split('/')
            .skip(1)
            .map(|x| x.replace("~1", "/").replace("~0", "~"))
            .try_fold(
                self,
                |target, token| match target {
                    Value::Object(map) => map.get(&token),
                    Value::Array(list) => parse_index(&token).and_then(|x| list.get(x)),
                    _ => None,
                },
            )
    }
    /// Looks up a value by a JSON Pointer and returns a mutable reference to
    /// that value.
    ///
    /// JSON Pointer defines a string syntax for identifying a specific value
    /// within a JavaScript Object Notation (JSON) document.
    ///
    /// A Pointer is a Unicode string with the reference tokens separated by `/`.
    /// Inside tokens `/` is replaced by `~1` and `~` is replaced by `~0`. The
    /// addressed value is returned and if there is no such value `None` is
    /// returned.
    ///
    /// For more information read [RFC6901](https://tools.ietf.org/html/rfc6901).
    ///
    /// # Example of Use
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// fn main() {
    ///     let s = r#"{"x": 1.0, "y": 2.0}"#;
    ///     let mut value: Value = serde_json::from_str(s).unwrap();
    ///
    ///     // Check value using read-only pointer
    ///     assert_eq!(value.pointer("/x"), Some(&1.0.into()));
    ///     // Change value with direct assignment
    ///     *value.pointer_mut("/x").unwrap() = 1.5.into();
    ///     // Check that new value was written
    ///     assert_eq!(value.pointer("/x"), Some(&1.5.into()));
    ///     // Or change the value only if it exists
    ///     value.pointer_mut("/x").map(|v| *v = 1.5.into());
    ///
    ///     // "Steal" ownership of a value. Can replace with any valid Value.
    ///     let old_x = value.pointer_mut("/x").map(Value::take).unwrap();
    ///     assert_eq!(old_x, 1.5);
    ///     assert_eq!(value.pointer("/x").unwrap(), &Value::Null);
    /// }
    /// ```
    pub fn pointer_mut(&mut self, pointer: &str) -> Option<&mut Value> {
        if pointer.is_empty() {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        pointer
            .split('/')
            .skip(1)
            .map(|x| x.replace("~1", "/").replace("~0", "~"))
            .try_fold(
                self,
                |target, token| match target {
                    Value::Object(map) => map.get_mut(&token),
                    Value::Array(list) => {
                        parse_index(&token).and_then(move |x| list.get_mut(x))
                    }
                    _ => None,
                },
            )
    }
    /// Takes the value out of the `Value`, leaving a `Null` in its place.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let mut v = json!({ "x": "y" });
    /// assert_eq!(v["x"].take(), json!("y"));
    /// assert_eq!(v, json!({ "x": null }));
    /// ```
    pub fn take(&mut self) -> Value {
        mem::replace(self, Value::Null)
    }
}
/// The default value is `Value::Null`.
///
/// This is useful for handling omitted `Value` fields when deserializing.
///
/// # Examples
///
/// ```
/// # use serde::Deserialize;
/// use serde_json::Value;
///
/// #[derive(Deserialize)]
/// struct Settings {
///     level: i32,
///     #[serde(default)]
///     extras: Value,
/// }
///
/// # fn try_main() -> Result<(), serde_json::Error> {
/// let data = r#" { "level": 42 } "#;
/// let s: Settings = serde_json::from_str(data)?;
///
/// assert_eq!(s.level, 42);
/// assert_eq!(s.extras, Value::Null);
/// #
/// #     Ok(())
/// # }
/// #
/// # try_main().unwrap()
/// ```
impl Default for Value {
    fn default() -> Value {
        Value::Null
    }
}
mod de;
mod from;
mod index;
mod partial_eq;
mod ser;
/// Convert a `T` into `serde_json::Value` which is an enum that can represent
/// any valid JSON data.
///
/// # Example
///
/// ```
/// use serde::Serialize;
/// use serde_json::json;
///
/// use std::error::Error;
///
/// #[derive(Serialize)]
/// struct User {
///     fingerprint: String,
///     location: String,
/// }
///
/// fn compare_json_values() -> Result<(), Box<Error>> {
///     let u = User {
///         fingerprint: "0xF9BA143B95FF6D82".to_owned(),
///         location: "Menlo Park, CA".to_owned(),
///     };
///
///     // The type of `expected` is `serde_json::Value`
///     let expected = json!({
///         "fingerprint": "0xF9BA143B95FF6D82",
///         "location": "Menlo Park, CA",
///     });
///
///     let v = serde_json::to_value(u).unwrap();
///     assert_eq!(v, expected);
///
///     Ok(())
/// }
/// #
/// # compare_json_values().unwrap();
/// ```
///
/// # Errors
///
/// This conversion can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
///
/// ```
/// use std::collections::BTreeMap;
///
/// fn main() {
///     // The keys in this map are vectors, not strings.
///     let mut map = BTreeMap::new();
///     map.insert(vec![32, 64], "x86");
///
///     println!("{}", serde_json::to_value(map).unwrap_err());
/// }
/// ```
pub fn to_value<T>(value: T) -> Result<Value, Error>
where
    T: Serialize,
{
    value.serialize(Serializer)
}
/// Interpret a `serde_json::Value` as an instance of type `T`.
///
/// # Example
///
/// ```
/// use serde::Deserialize;
/// use serde_json::json;
///
/// #[derive(Deserialize, Debug)]
/// struct User {
///     fingerprint: String,
///     location: String,
/// }
///
/// fn main() {
///     // The type of `j` is `serde_json::Value`
///     let j = json!({
///         "fingerprint": "0xF9BA143B95FF6D82",
///         "location": "Menlo Park, CA"
///     });
///
///     let u: User = serde_json::from_value(j).unwrap();
///     println!("{:#?}", u);
/// }
/// ```
///
/// # Errors
///
/// This conversion can fail if the structure of the Value does not match the
/// structure expected by `T`, for example if `T` is a struct type but the Value
/// contains something other than a JSON map. It can also fail if the structure
/// is correct but `T`'s implementation of `Deserialize` decides that something
/// is wrong with the data, for example required struct fields are missing from
/// the JSON map or some number is too big to fit in the expected primitive
/// type.
pub fn from_value<T>(value: Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    T::deserialize(value)
}
#[cfg(test)]
mod tests_llm_16_299 {
    use crate::{Map, Number, Value};
    #[test]
    fn test_default_value_is_null() {
        let _rug_st_tests_llm_16_299_rrrruuuugggg_test_default_value_is_null = 0;
        let default_value: Value = Value::default();
        if let Value::Null = default_value {} else {
            panic!("Default Value should be null");
        }
        let _rug_ed_tests_llm_16_299_rrrruuuugggg_test_default_value_is_null = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_604 {
    use super::*;
    use crate::*;
    use crate::json;
    #[test]
    fn test_as_array_some() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let array_value = json!([rug_fuzz_0, rug_fuzz_1, rug_fuzz_2]);
        let array = array_value.as_array();
        debug_assert!(array.is_some());
        debug_assert_eq!(array.unwrap().len(), 3);
             }
}
}
}    }
    #[test]
    fn test_as_array_none() {
        let _rug_st_tests_llm_16_604_rrrruuuugggg_test_as_array_none = 0;
        let non_array_value = json!({ "key" : "value" });
        debug_assert!(non_array_value.as_array().is_none());
        let _rug_ed_tests_llm_16_604_rrrruuuugggg_test_as_array_none = 0;
    }
    #[test]
    fn test_as_array_null() {
        let _rug_st_tests_llm_16_604_rrrruuuugggg_test_as_array_null = 0;
        let null_value = json!(null);
        debug_assert!(null_value.as_array().is_none());
        let _rug_ed_tests_llm_16_604_rrrruuuugggg_test_as_array_null = 0;
    }
    #[test]
    fn test_as_array_boolean() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let boolean_value = json!(rug_fuzz_0);
        debug_assert!(boolean_value.as_array().is_none());
             }
}
}
}    }
    #[test]
    fn test_as_array_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number_value = json!(rug_fuzz_0);
        debug_assert!(number_value.as_array().is_none());
             }
}
}
}    }
    #[test]
    fn test_as_array_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let string_value = json!(rug_fuzz_0);
        debug_assert!(string_value.as_array().is_none());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_605 {
    use crate::{json, Value};
    #[test]
    fn test_as_array_mut_existing_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v = json!({ "array_key" : ["elem1", "elem2", "elem3"] });
        let array = v[rug_fuzz_0].as_array_mut().unwrap();
        array.push(json!(rug_fuzz_1));
        debug_assert_eq!(
            v, json!({ "array_key" : ["elem1", "elem2", "elem3", "elem4"] })
        );
             }
}
}
}    }
    #[test]
    fn test_as_array_mut_non_existing_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v = json!({ "non_array_key" : "not an array" });
        debug_assert!(v[rug_fuzz_0].as_array_mut().is_none());
             }
}
}
}    }
    #[test]
    fn test_as_array_mut_null() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v = json!({ rug_fuzz_0 : null });
        debug_assert!(v[rug_fuzz_1].as_array_mut().is_none());
             }
}
}
}    }
    #[test]
    fn test_as_array_mut_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v = json!({ "number_key" : 42 });
        debug_assert!(v[rug_fuzz_0].as_array_mut().is_none());
             }
}
}
}    }
    #[test]
    fn test_as_array_mut_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v = json!({ "object_key" : { "inner" : "value" } });
        debug_assert!(v[rug_fuzz_0].as_array_mut().is_none());
             }
}
}
}    }
    #[test]
    fn test_as_array_mut_empty_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v = json!({ "empty_array_key" : [] });
        v[rug_fuzz_0].as_array_mut().unwrap().push(json!(rug_fuzz_1));
        debug_assert_eq!(v, json!({ "empty_array_key" : ["elem1"] }));
             }
}
}
}    }
    #[test]
    fn test_as_array_mut_nested_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v = json!({ "nested" : { "array_key" : ["elem1", "elem2"] } });
        let array = v[rug_fuzz_0][rug_fuzz_1].as_array_mut().unwrap();
        array.push(json!(rug_fuzz_2));
        debug_assert_eq!(
            v, json!({ "nested" : { "array_key" : ["elem1", "elem2", "elem3"] } })
        );
             }
}
}
}    }
    #[test]
    fn test_as_array_mut_array_root() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v = json!([rug_fuzz_0, rug_fuzz_1, rug_fuzz_2]);
        let array = v.as_array_mut().unwrap();
        array.push(json!(rug_fuzz_3));
        debug_assert_eq!(v, json!(["elem1", "elem2", "elem3", "elem4"]));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_606 {
    use crate::{json, Value};
    #[test]
    fn test_as_bool() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16)) = <(&str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, bool, bool, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let obj = json!(
            { "key1" : true, "key2" : false, "key3" : "true", "key4" : "false", "key5" :
            1, "key6" : 0, "key7" : 1.0, "key8" : 0.0, "key9" : "1", "key10" : "0",
            "key11" : null, "key12" : {}, "key13" : [], }
        );
        debug_assert_eq!(obj[rug_fuzz_0].as_bool(), Some(true));
        debug_assert_eq!(obj[rug_fuzz_1].as_bool(), Some(false));
        debug_assert_eq!(obj[rug_fuzz_2].as_bool(), None);
        debug_assert_eq!(obj[rug_fuzz_3].as_bool(), None);
        debug_assert_eq!(obj[rug_fuzz_4].as_bool(), None);
        debug_assert_eq!(obj[rug_fuzz_5].as_bool(), None);
        debug_assert_eq!(obj[rug_fuzz_6].as_bool(), None);
        debug_assert_eq!(obj[rug_fuzz_7].as_bool(), None);
        debug_assert_eq!(obj[rug_fuzz_8].as_bool(), None);
        debug_assert_eq!(obj[rug_fuzz_9].as_bool(), None);
        debug_assert_eq!(obj[rug_fuzz_10].as_bool(), None);
        debug_assert_eq!(obj[rug_fuzz_11].as_bool(), None);
        debug_assert_eq!(obj[rug_fuzz_12].as_bool(), None);
        debug_assert_eq!(Value::Bool(rug_fuzz_13).as_bool(), Some(true));
        debug_assert_eq!(Value::Bool(rug_fuzz_14).as_bool(), Some(false));
        debug_assert_eq!(Value::String(rug_fuzz_15.to_string()).as_bool(), None);
        debug_assert_eq!(Value::Number(rug_fuzz_16.into()).as_bool(), None);
        debug_assert_eq!(Value::Null.as_bool(), None);
        debug_assert_eq!(Value::Object(crate ::Map::new()).as_bool(), None);
        debug_assert_eq!(Value::Array(vec![]).as_bool(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_607 {
    use crate::Value;
    use crate::json;
    #[test]
    fn as_f64_null() {
        let _rug_st_tests_llm_16_607_rrrruuuugggg_as_f64_null = 0;
        let v = json!(null);
        debug_assert_eq!(v.as_f64(), None);
        let _rug_ed_tests_llm_16_607_rrrruuuugggg_as_f64_null = 0;
    }
    #[test]
    fn as_f64_bool() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = json!(rug_fuzz_0);
        debug_assert_eq!(v.as_f64(), None);
             }
}
}
}    }
    #[test]
    fn as_f64_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(f64, f64, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = json!(rug_fuzz_0);
        debug_assert_eq!(v.as_f64(), Some(12.5));
        let v = json!(- rug_fuzz_1);
        debug_assert_eq!(v.as_f64(), Some(- 12.5));
        let v = json!(rug_fuzz_2);
        debug_assert_eq!(v.as_f64(), Some(12.0));
        let v = json!(- rug_fuzz_3);
        debug_assert_eq!(v.as_f64(), Some(- 12.0));
             }
}
}
}    }
    #[test]
    fn as_f64_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = json!(rug_fuzz_0);
        debug_assert_eq!(v.as_f64(), None);
             }
}
}
}    }
    #[test]
    fn as_f64_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = json!([rug_fuzz_0, rug_fuzz_1, rug_fuzz_2]);
        debug_assert_eq!(v.as_f64(), None);
             }
}
}
}    }
    #[test]
    fn as_f64_object() {
        let _rug_st_tests_llm_16_607_rrrruuuugggg_as_f64_object = 0;
        let v = json!({ "a" : 1 });
        debug_assert_eq!(v.as_f64(), None);
        let _rug_ed_tests_llm_16_607_rrrruuuugggg_as_f64_object = 0;
    }
    #[test]
    fn as_f64_i64_max() {
        let _rug_st_tests_llm_16_607_rrrruuuugggg_as_f64_i64_max = 0;
        let v = json!(i64::MAX);
        debug_assert_eq!(v.as_f64(), Some(i64::MAX as f64));
        let _rug_ed_tests_llm_16_607_rrrruuuugggg_as_f64_i64_max = 0;
    }
    #[test]
    fn as_f64_i64_min() {
        let _rug_st_tests_llm_16_607_rrrruuuugggg_as_f64_i64_min = 0;
        let v = json!(i64::MIN);
        debug_assert_eq!(v.as_f64(), Some(i64::MIN as f64));
        let _rug_ed_tests_llm_16_607_rrrruuuugggg_as_f64_i64_min = 0;
    }
    #[test]
    fn as_f64_u64_max() {
        let _rug_st_tests_llm_16_607_rrrruuuugggg_as_f64_u64_max = 0;
        let v = json!(u64::MAX);
        debug_assert_eq!(v.as_f64(), Some(u64::MAX as f64));
        let _rug_ed_tests_llm_16_607_rrrruuuugggg_as_f64_u64_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_608 {
    use crate::{json, Value};
    #[test]
    fn test_as_i64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u64, &str, &str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let big = i64::max_value() as u64 + rug_fuzz_0;
        let v = json!(
            { "integer" : 64, "negative_integer" : - 64, "big_integer" : big, "float" :
            256.0, "string" : "64", "array" : [64], "object" : { "key" : 64 }, "bool" :
            true, "null" : null, }
        );
        debug_assert_eq!(v[rug_fuzz_1].as_i64(), Some(64));
        debug_assert_eq!(v[rug_fuzz_2].as_i64(), Some(- 64));
        debug_assert_eq!(v[rug_fuzz_3].as_i64(), None);
        debug_assert_eq!(v[rug_fuzz_4].as_i64(), None);
        debug_assert_eq!(v[rug_fuzz_5].as_i64(), None);
        debug_assert_eq!(v[rug_fuzz_6].as_i64(), None);
        debug_assert_eq!(v[rug_fuzz_7].as_i64(), None);
        debug_assert_eq!(v[rug_fuzz_8].as_i64(), None);
        debug_assert_eq!(v[rug_fuzz_9].as_i64(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_609 {
    use crate::{json, Value};
    #[test]
    fn value_as_null_null() {
        let _rug_st_tests_llm_16_609_rrrruuuugggg_value_as_null_null = 0;
        let value = Value::Null;
        debug_assert_eq!(value.as_null(), Some(()));
        let _rug_ed_tests_llm_16_609_rrrruuuugggg_value_as_null_null = 0;
    }
    #[test]
    fn value_as_null_boolean() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = json!(rug_fuzz_0);
        debug_assert_eq!(value.as_null(), None);
             }
}
}
}    }
    #[test]
    fn value_as_null_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = json!(rug_fuzz_0);
        debug_assert_eq!(value.as_null(), None);
             }
}
}
}    }
    #[test]
    fn value_as_null_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = json!(rug_fuzz_0);
        debug_assert_eq!(value.as_null(), None);
             }
}
}
}    }
    #[test]
    fn value_as_null_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = json!([rug_fuzz_0]);
        debug_assert_eq!(value.as_null(), None);
             }
}
}
}    }
    #[test]
    fn value_as_null_object() {
        let _rug_st_tests_llm_16_609_rrrruuuugggg_value_as_null_object = 0;
        let value = json!({ "key" : "value" });
        debug_assert_eq!(value.as_null(), None);
        let _rug_ed_tests_llm_16_609_rrrruuuugggg_value_as_null_object = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_610 {
    use super::*;
    use crate::*;
    use crate::{json, Value};
    #[test]
    fn test_as_object_with_object() {
        let _rug_st_tests_llm_16_610_rrrruuuugggg_test_as_object_with_object = 0;
        let obj = json!({ "key1" : "value1", "key2" : "value2" });
        debug_assert!(obj.as_object().is_some());
        debug_assert_eq!(obj.as_object().unwrap().len(), 2);
        let _rug_ed_tests_llm_16_610_rrrruuuugggg_test_as_object_with_object = 0;
    }
    #[test]
    fn test_as_object_with_non_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let arr = json!([rug_fuzz_0, rug_fuzz_1]);
        debug_assert!(arr.as_object().is_none());
             }
}
}
}    }
    #[test]
    fn test_as_object_with_nested_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let nested = json!({ "outer" : { "inner_key" : "inner_value" } });
        debug_assert!(nested[rug_fuzz_0].as_object().is_some());
        debug_assert_eq!(nested[rug_fuzz_1].as_object().unwrap().len(), 1);
             }
}
}
}    }
    #[test]
    fn test_as_object_with_null() {
        let _rug_st_tests_llm_16_610_rrrruuuugggg_test_as_object_with_null = 0;
        let null = Value::Null;
        debug_assert!(null.as_object().is_none());
        let _rug_ed_tests_llm_16_610_rrrruuuugggg_test_as_object_with_null = 0;
    }
    #[test]
    fn test_as_object_with_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let string = Value::String(String::from(rug_fuzz_0));
        debug_assert!(string.as_object().is_none());
             }
}
}
}    }
    #[test]
    fn test_as_object_with_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = Value::Number(Number::from_f64(rug_fuzz_0).unwrap());
        debug_assert!(number.as_object().is_none());
             }
}
}
}    }
    #[test]
    fn test_as_object_with_bool() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let boolean = Value::Bool(rug_fuzz_0);
        debug_assert!(boolean.as_object().is_none());
             }
}
}
}    }
    #[test]
    fn test_as_object_with_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let array = Value::Array(vec![json!(rug_fuzz_0), json!(2), json!(3)]);
        debug_assert!(array.as_object().is_none());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_611 {
    use super::*;
    use crate::*;
    use crate::json;
    use crate::map::Map;
    use crate::Value;
    #[test]
    fn test_as_object_mut_valid_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v = json!({ "a" : 1, "b" : 2 });
        let o = v.as_object_mut();
        debug_assert!(o.is_some());
        let mut o = o.unwrap();
        debug_assert_eq!(o.len(), 2);
        o.insert(rug_fuzz_0.to_string(), json!(rug_fuzz_1));
        debug_assert_eq!(v, json!({ "a" : 1, "b" : 2, "c" : 3 }));
             }
}
}
}    }
    #[test]
    fn test_as_object_mut_null() {
        let _rug_st_tests_llm_16_611_rrrruuuugggg_test_as_object_mut_null = 0;
        let mut v = json!(null);
        let o = v.as_object_mut();
        debug_assert!(o.is_none());
        let _rug_ed_tests_llm_16_611_rrrruuuugggg_test_as_object_mut_null = 0;
    }
    #[test]
    fn test_as_object_mut_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v = json!([rug_fuzz_0, rug_fuzz_1, rug_fuzz_2]);
        let o = v.as_object_mut();
        debug_assert!(o.is_none());
             }
}
}
}    }
    #[test]
    fn test_as_object_mut_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v = json!(rug_fuzz_0);
        let o = v.as_object_mut();
        debug_assert!(o.is_none());
             }
}
}
}    }
    #[test]
    fn test_as_object_mut_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v = json!(rug_fuzz_0);
        let o = v.as_object_mut();
        debug_assert!(o.is_none());
             }
}
}
}    }
    #[test]
    fn test_as_object_mut_bool() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v = json!(rug_fuzz_0);
        let o = v.as_object_mut();
        debug_assert!(o.is_none());
             }
}
}
}    }
    #[test]
    fn test_as_object_mut_clear() {
        let _rug_st_tests_llm_16_611_rrrruuuugggg_test_as_object_mut_clear = 0;
        let mut v = json!({ "a" : 1, "b" : 2 });
        let o = v.as_object_mut();
        debug_assert!(o.is_some());
        let mut o = o.unwrap();
        o.clear();
        debug_assert_eq!(v, json!({}));
        let _rug_ed_tests_llm_16_611_rrrruuuugggg_test_as_object_mut_clear = 0;
    }
    #[test]
    fn test_as_object_mut_modify() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v = json!({ "a" : 1, "b" : 2 });
        let o = v.as_object_mut();
        debug_assert!(o.is_some());
        let mut o = o.unwrap();
        o.insert(rug_fuzz_0.to_string(), json!(rug_fuzz_1));
        debug_assert_eq!(v, json!({ "a" : 10, "b" : 2 }));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_612 {
    use crate::json;
    use crate::value::Value;
    #[test]
    fn as_str_string_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = json!(rug_fuzz_0);
        debug_assert_eq!(v.as_str(), Some("test"));
             }
}
}
}    }
    #[test]
    fn as_str_object_value() {
        let _rug_st_tests_llm_16_612_rrrruuuugggg_as_str_object_value = 0;
        let v = json!({ "key" : "value" });
        debug_assert_eq!(v.as_str(), None);
        let _rug_ed_tests_llm_16_612_rrrruuuugggg_as_str_object_value = 0;
    }
    #[test]
    fn as_str_array_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = json!([rug_fuzz_0]);
        debug_assert_eq!(v.as_str(), None);
             }
}
}
}    }
    #[test]
    fn as_str_null_value() {
        let _rug_st_tests_llm_16_612_rrrruuuugggg_as_str_null_value = 0;
        let v = json!(null);
        debug_assert_eq!(v.as_str(), None);
        let _rug_ed_tests_llm_16_612_rrrruuuugggg_as_str_null_value = 0;
    }
    #[test]
    fn as_str_boolean_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = json!(rug_fuzz_0);
        debug_assert_eq!(v.as_str(), None);
             }
}
}
}    }
    #[test]
    fn as_str_number_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = json!(rug_fuzz_0);
        debug_assert_eq!(v.as_str(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_613 {
    use super::*;
    use crate::*;
    use crate::json;
    #[test]
    fn test_as_u64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(&str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = json!(
            { "a" : 64_u64, "b" : "text", "c" : 256.0, "d" : - 64_i64, "e" : i64::MAX,
            "f" : u64::MAX, "g" : null }
        );
        debug_assert_eq!(v[rug_fuzz_0].as_u64(), Some(64_u64));
        debug_assert_eq!(v[rug_fuzz_1].as_u64(), None);
        debug_assert_eq!(v[rug_fuzz_2].as_u64(), None);
        debug_assert_eq!(v[rug_fuzz_3].as_u64(), None);
        debug_assert_eq!(v[rug_fuzz_4].as_u64(), Some(i64::MAX as u64));
        debug_assert_eq!(v[rug_fuzz_5].as_u64(), Some(u64::MAX));
        debug_assert_eq!(v[rug_fuzz_6].as_u64(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_614 {
    use super::*;
    use crate::*;
    use crate::json;
    #[test]
    fn get_with_string_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let obj = json!({ "key" : "value", "array" : [1, 2, 3] });
        debug_assert_eq!(obj.get(rug_fuzz_0), Some(& json!("value")));
             }
}
}
}    }
    #[test]
    fn get_with_usize_on_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, i32, i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let array = json!([rug_fuzz_0, rug_fuzz_1, rug_fuzz_2]);
        debug_assert_eq!(array.get(rug_fuzz_3), Some(& json!(2)));
             }
}
}
}    }
    #[test]
    fn get_with_string_key_on_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let array = json!([rug_fuzz_0, rug_fuzz_1]);
        debug_assert_eq!(array.get(rug_fuzz_2), None);
             }
}
}
}    }
    #[test]
    fn get_with_index_out_of_bounds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, i32, i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let array = json!([rug_fuzz_0, rug_fuzz_1, rug_fuzz_2]);
        debug_assert_eq!(array.get(rug_fuzz_3), None);
             }
}
}
}    }
    #[test]
    fn get_on_non_object_non_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = json!(rug_fuzz_0);
        debug_assert_eq!(number.get(rug_fuzz_1), None);
        debug_assert_eq!(number.get(rug_fuzz_2), None);
             }
}
}
}    }
    #[test]
    fn get_with_usize_on_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let obj = json!({ "1" : "value1", "2" : "value2" });
        debug_assert_eq!(obj.get(rug_fuzz_0), None);
             }
}
}
}    }
    #[test]
    fn get_on_empty_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let obj = json!({});
        debug_assert_eq!(obj.get(rug_fuzz_0), None);
             }
}
}
}    }
    #[test]
    fn get_on_empty_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let array = json!([]);
        debug_assert_eq!(array.get(rug_fuzz_0), None);
             }
}
}
}    }
    #[test]
    fn get_with_non_existing_string_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let obj = json!({ "key1" : "value1", "key2" : "value2" });
        debug_assert_eq!(obj.get(rug_fuzz_0), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_615 {
    use crate::{json, Value};
    #[test]
    fn test_get_mut_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut object = json!({ "A" : 65, "B" : 66, "C" : 67 });
        let a = object.get_mut(rug_fuzz_0).unwrap();
        *a = json!(rug_fuzz_1);
        debug_assert_eq!(object, json!({ "A" : 69, "B" : 66, "C" : 67 }));
             }
}
}
}    }
    #[test]
    fn test_get_mut_object_nonexistent_key() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut object = json!({ "A" : 65, "B" : 66, "C" : 67 });
        let result = object.get_mut(rug_fuzz_0);
        debug_assert!(result.is_none());
             }
}
}
}    }
    #[test]
    fn test_get_mut_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut array = json!([rug_fuzz_0, rug_fuzz_1, rug_fuzz_2]);
        let c = array.get_mut(rug_fuzz_3).unwrap();
        *c = json!(rug_fuzz_4);
        debug_assert_eq!(array, json!(["A", "B", "D"]));
             }
}
}
}    }
    #[test]
    fn test_get_mut_array_out_of_bounds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut array = json!([rug_fuzz_0, rug_fuzz_1, rug_fuzz_2]);
        let result = array.get_mut(rug_fuzz_3);
        debug_assert!(result.is_none());
             }
}
}
}    }
    #[test]
    fn test_get_mut_wrong_type() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value = json!(rug_fuzz_0);
        let result = value.get_mut(rug_fuzz_1);
        debug_assert!(result.is_none());
             }
}
}
}    }
    #[test]
    fn test_get_mut_null() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value = json!(null);
        let result = value.get_mut(rug_fuzz_0);
        debug_assert!(result.is_none());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_616 {
    use crate::json;
    use crate::value::Value;
    #[test]
    fn test_is_array_on_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let val = json!([rug_fuzz_0, rug_fuzz_1, rug_fuzz_2]);
        debug_assert!(val.is_array());
             }
}
}
}    }
    #[test]
    fn test_is_array_on_empty_array() {
        let _rug_st_tests_llm_16_616_rrrruuuugggg_test_is_array_on_empty_array = 0;
        let val = json!([]);
        debug_assert!(val.is_array());
        let _rug_ed_tests_llm_16_616_rrrruuuugggg_test_is_array_on_empty_array = 0;
    }
    #[test]
    fn test_is_array_on_object() {
        let _rug_st_tests_llm_16_616_rrrruuuugggg_test_is_array_on_object = 0;
        let val = json!({ "foo" : "bar" });
        debug_assert!(! val.is_array());
        let _rug_ed_tests_llm_16_616_rrrruuuugggg_test_is_array_on_object = 0;
    }
    #[test]
    fn test_is_array_on_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let val = json!(rug_fuzz_0);
        debug_assert!(! val.is_array());
             }
}
}
}    }
    #[test]
    fn test_is_array_on_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let val = json!(rug_fuzz_0);
        debug_assert!(! val.is_array());
             }
}
}
}    }
    #[test]
    fn test_is_array_on_boolean() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let val = json!(rug_fuzz_0);
        debug_assert!(! val.is_array());
             }
}
}
}    }
    #[test]
    fn test_is_array_on_null() {
        let _rug_st_tests_llm_16_616_rrrruuuugggg_test_is_array_on_null = 0;
        let val = json!(null);
        debug_assert!(! val.is_array());
        let _rug_ed_tests_llm_16_616_rrrruuuugggg_test_is_array_on_null = 0;
    }
    #[test]
    fn test_is_array_on_nested_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let val = json!([[rug_fuzz_0, rug_fuzz_1]]);
        debug_assert!(val.is_array());
        debug_assert!(val[rug_fuzz_2].is_array());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_617 {
    use crate::{json, Value};
    #[test]
    fn test_is_boolean() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(bool, bool, &str, i32, bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let true_val = json!(rug_fuzz_0);
        debug_assert!(true_val.is_boolean());
        let false_val = json!(rug_fuzz_1);
        debug_assert!(false_val.is_boolean());
        let null_val = Value::Null;
        debug_assert!(! null_val.is_boolean());
        let string_val = json!(rug_fuzz_2);
        debug_assert!(! string_val.is_boolean());
        let number_val = json!(rug_fuzz_3);
        debug_assert!(! number_val.is_boolean());
        let object_val = json!({ "key" : "value" });
        debug_assert!(! object_val.is_boolean());
        let array_val = json!([rug_fuzz_4, rug_fuzz_5]);
        debug_assert!(! array_val.is_boolean());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_618 {
    use crate::value::Value;
    #[test]
    fn test_is_f64_for_f64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let f64_value = Value::from(rug_fuzz_0);
        debug_assert!(f64_value.is_f64());
             }
}
}
}    }
    #[test]
    fn test_is_f64_for_i64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let i64_value = Value::from(rug_fuzz_0);
        debug_assert!(! i64_value.is_f64());
             }
}
}
}    }
    #[test]
    fn test_is_f64_for_u64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let u64_value = Value::from(rug_fuzz_0);
        debug_assert!(! u64_value.is_f64());
             }
}
}
}    }
    #[test]
    fn test_is_f64_for_i64_edge_case() {
        let _rug_st_tests_llm_16_618_rrrruuuugggg_test_is_f64_for_i64_edge_case = 0;
        let max_i64_value = Value::from(i64::MAX);
        debug_assert!(! max_i64_value.is_f64());
        let _rug_ed_tests_llm_16_618_rrrruuuugggg_test_is_f64_for_i64_edge_case = 0;
    }
    #[test]
    fn test_is_f64_for_u64_edge_case() {
        let _rug_st_tests_llm_16_618_rrrruuuugggg_test_is_f64_for_u64_edge_case = 0;
        let max_u64_value = Value::from(u64::MAX);
        debug_assert!(! max_u64_value.is_f64());
        let _rug_ed_tests_llm_16_618_rrrruuuugggg_test_is_f64_for_u64_edge_case = 0;
    }
    #[test]
    fn test_is_f64_for_bool() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bool_value = Value::from(rug_fuzz_0);
        debug_assert!(! bool_value.is_f64());
             }
}
}
}    }
    #[test]
    fn test_is_f64_for_null() {
        let _rug_st_tests_llm_16_618_rrrruuuugggg_test_is_f64_for_null = 0;
        let null_value = Value::Null;
        debug_assert!(! null_value.is_f64());
        let _rug_ed_tests_llm_16_618_rrrruuuugggg_test_is_f64_for_null = 0;
    }
    #[test]
    fn test_is_f64_for_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let string_value = Value::from(rug_fuzz_0.to_string());
        debug_assert!(! string_value.is_f64());
             }
}
}
}    }
    #[test]
    fn test_is_f64_for_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let array_value = Value::from(vec![rug_fuzz_0, 3.14_f64]);
        debug_assert!(! array_value.is_f64());
             }
}
}
}    }
    #[test]
    fn test_is_f64_for_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        use crate::Map;
        let mut object = Map::new();
        object.insert(rug_fuzz_0.to_string(), Value::from(rug_fuzz_1));
        let object_value = Value::from(object);
        debug_assert!(! object_value.is_f64());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_619_llm_16_619 {
    use crate::{number::Number, Map, Value};
    #[test]
    fn test_value_is_i64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, f64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let i64_val = Value::Number(Number::from(rug_fuzz_0));
        debug_assert!(i64_val.is_i64());
        let u64_val = Value::Number(Number::from(u64::MAX));
        debug_assert!(! u64_val.is_i64());
        let f64_val = Value::Number(Number::from_f64(rug_fuzz_1).unwrap());
        debug_assert!(! f64_val.is_i64());
        let neg_i64_val = Value::Number(Number::from(-rug_fuzz_2));
        debug_assert!(neg_i64_val.is_i64());
             }
}
}
}    }
    #[test]
    fn test_value_is_i64_at_bounds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let i64_min = Value::Number(Number::from(i64::MIN));
        debug_assert!(i64_min.is_i64());
        let i64_max = Value::Number(Number::from(i64::MAX));
        debug_assert!(i64_max.is_i64());
        let below_i64_min = Value::Number(
            Number::from_f64((i64::MIN as f64) - rug_fuzz_0).unwrap(),
        );
        debug_assert!(! below_i64_min.is_i64());
        let above_i64_max = Value::Number(
            Number::from_f64((i64::MAX as u64 as f64) + rug_fuzz_1).unwrap(),
        );
        debug_assert!(! above_i64_max.is_i64());
             }
}
}
}    }
    #[test]
    fn test_value_is_i64_with_non_number_types() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(bool, &str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let null_val = Value::Null;
        debug_assert!(! null_val.is_i64());
        let bool_val = Value::Bool(rug_fuzz_0);
        debug_assert!(! bool_val.is_i64());
        let string_val = Value::String(rug_fuzz_1.to_string());
        debug_assert!(! string_val.is_i64());
        let array_val = Value::Array(vec![Value::Number(Number::from(rug_fuzz_2))]);
        debug_assert!(! array_val.is_i64());
        let object_val = Value::Object(Map::new());
        debug_assert!(! object_val.is_i64());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_620 {
    use crate::{json, Value};
    #[test]
    fn test_is_null() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(bool, i32, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let null_value = Value::Null;
        debug_assert!(null_value.is_null());
        let bool_value = json!(rug_fuzz_0);
        debug_assert!(! bool_value.is_null());
        let array_value = json!([]);
        debug_assert!(! array_value.is_null());
        let object_value = json!({});
        debug_assert!(! object_value.is_null());
        let number_value = json!(rug_fuzz_1);
        debug_assert!(! number_value.is_null());
        let string_value = json!(rug_fuzz_2);
        debug_assert!(! string_value.is_null());
        let empty_string_value = json!(rug_fuzz_3);
        debug_assert!(! empty_string_value.is_null());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_621 {
    use crate::json;
    use crate::Value;
    #[test]
    fn test_is_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(&str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = json!(
            { "integer" : 42, "float" : 3.14, "string" : "not a number", "bool" : true,
            "object" : { "key" : "value" }, "array" : [1, 2, 3], "null" : null }
        );
        debug_assert!(v[rug_fuzz_0].is_number());
        debug_assert!(v[rug_fuzz_1].is_number());
        debug_assert!(! v[rug_fuzz_2].is_number());
        debug_assert!(! v[rug_fuzz_3].is_number());
        debug_assert!(! v[rug_fuzz_4].is_number());
        debug_assert!(! v[rug_fuzz_5].is_number());
        debug_assert!(! v[rug_fuzz_6].is_number());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_622 {
    use super::*;
    use crate::*;
    use crate::json;
    #[test]
    fn test_is_object_with_object() {
        let _rug_st_tests_llm_16_622_rrrruuuugggg_test_is_object_with_object = 0;
        let obj = json!({ "a" : 1, "b" : 2 });
        debug_assert!(obj.is_object());
        let _rug_ed_tests_llm_16_622_rrrruuuugggg_test_is_object_with_object = 0;
    }
    #[test]
    fn test_is_object_with_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let arr = json!([rug_fuzz_0, rug_fuzz_1, rug_fuzz_2]);
        debug_assert!(! arr.is_object());
             }
}
}
}    }
    #[test]
    fn test_is_object_with_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let string = json!(rug_fuzz_0);
        debug_assert!(! string.is_object());
             }
}
}
}    }
    #[test]
    fn test_is_object_with_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = json!(rug_fuzz_0);
        debug_assert!(! number.is_object());
             }
}
}
}    }
    #[test]
    fn test_is_object_with_boolean() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let boolean = json!(rug_fuzz_0);
        debug_assert!(! boolean.is_object());
             }
}
}
}    }
    #[test]
    fn test_is_object_with_null() {
        let _rug_st_tests_llm_16_622_rrrruuuugggg_test_is_object_with_null = 0;
        let null = json!(null);
        debug_assert!(! null.is_object());
        let _rug_ed_tests_llm_16_622_rrrruuuugggg_test_is_object_with_null = 0;
    }
    #[test]
    fn test_is_object_with_nested_object() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let nested = json!({ "outer" : { "inner" : "value" } });
        debug_assert!(nested[rug_fuzz_0].is_object());
             }
}
}
}    }
    #[test]
    fn test_is_object_with_nested_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let nested = json!({ "array" : ["a", "b", "c"] });
        debug_assert!(! nested[rug_fuzz_0].is_object());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_623 {
    use crate::{json, Value};
    #[test]
    fn test_is_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(&str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = json!(
            { "string" : "hello", "number" : 42, "object" : { "key" : "value" }, "array"
            : [1, 2, 3], "boolean" : true, "null" : null }
        );
        debug_assert!(v[rug_fuzz_0].is_string());
        debug_assert!(! v[rug_fuzz_1].is_string());
        debug_assert!(! v[rug_fuzz_2].is_string());
        debug_assert!(! v[rug_fuzz_3].is_string());
        debug_assert!(! v[rug_fuzz_4].is_string());
        debug_assert!(! v[rug_fuzz_5].is_string());
        let null_v = Value::Null;
        debug_assert!(! null_v.is_string());
        let string_v = Value::String(rug_fuzz_6.to_owned());
        debug_assert!(string_v.is_string());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_624_llm_16_624 {
    use crate::value::Value;
    use crate::number::Number;
    #[test]
    fn test_is_u64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u64, i64, f64, &str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let u64_val = Value::Number(Number::from(rug_fuzz_0));
        let i64_val = Value::Number(Number::from(-rug_fuzz_1));
        let f64_val = Value::Number(Number::from_f64(rug_fuzz_2).unwrap());
        let string_val = Value::String(rug_fuzz_3.into());
        let object_val = Value::Object(crate::map::Map::new());
        let array_val = Value::Array(vec![]);
        let bool_val = Value::Bool(rug_fuzz_4);
        let null_val = Value::Null;
        debug_assert!(u64_val.is_u64());
        debug_assert!(! i64_val.is_u64());
        debug_assert!(! f64_val.is_u64());
        debug_assert!(! string_val.is_u64());
        debug_assert!(! object_val.is_u64());
        debug_assert!(! array_val.is_u64());
        debug_assert!(! bool_val.is_u64());
        debug_assert!(! null_val.is_u64());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_625 {
    use super::*;
    use crate::*;
    use crate::json;
    #[test]
    fn test_pointer_valid_paths() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(&str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = json!(
            { "x" : { "y" : ["a", "b", "c"], "z" : { "i" : "j" }, }, "a" : "b", "~" :
            "tilde", "" : "empty", }
        );
        debug_assert_eq!(data.pointer(rug_fuzz_0), Some(& json!("a")));
        debug_assert_eq!(data.pointer(rug_fuzz_1).unwrap().as_array().unwrap().len(), 3);
        debug_assert_eq!(data.pointer(rug_fuzz_2), Some(& json!("c")));
        debug_assert_eq!(data.pointer(rug_fuzz_3), Some(& json!("j")));
        debug_assert_eq!(data.pointer(rug_fuzz_4), Some(& json!("b")));
        debug_assert_eq!(data.pointer(rug_fuzz_5), Some(& json!("tilde")));
        debug_assert_eq!(data.pointer(rug_fuzz_6), Some(& json!("empty")));
             }
}
}
}    }
    #[test]
    fn test_pointer_invalid_paths() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(&str, &str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = json!({ "x" : { "y" : ["a", "b", "c"], }, "a" : 1, });
        debug_assert_eq!(data.pointer(rug_fuzz_0), Some(& data));
        debug_assert_eq!(data.pointer(rug_fuzz_1), None);
        debug_assert_eq!(data.pointer(rug_fuzz_2), None);
        debug_assert_eq!(data.pointer(rug_fuzz_3), None);
        debug_assert_eq!(data.pointer(rug_fuzz_4), None);
        debug_assert_eq!(data.pointer(rug_fuzz_5), None);
        debug_assert_eq!(data.pointer(rug_fuzz_6), None);
        debug_assert_eq!(data.pointer(rug_fuzz_7), None);
        debug_assert_eq!(data.pointer(rug_fuzz_8), None);
             }
}
}
}    }
    #[test]
    fn test_pointer_edge_cases() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(&str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = json!(
            { "" : { "" : "nested empty" }, "nested" : { "" : "nested", "arr" : ["empty",
            ""] }, "arr" : [""] }
        );
        debug_assert_eq!(
            data.pointer(rug_fuzz_0), Some(& json!({ "" : "nested empty" }))
        );
        debug_assert_eq!(data.pointer(rug_fuzz_1), Some(& json!("nested empty")));
        debug_assert_eq!(
            data.pointer(rug_fuzz_2), Some(& json!({ "" : "nested", "arr" : ["empty", ""]
            }))
        );
        debug_assert_eq!(data.pointer(rug_fuzz_3), Some(& json!("nested")));
        debug_assert_eq!(data.pointer(rug_fuzz_4), Some(& json!("")));
        debug_assert_eq!(data.pointer(rug_fuzz_5), Some(& json!("")));
        debug_assert_eq!(data.pointer(rug_fuzz_6), Some(& json!("")));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_626 {
    use crate::{json, Value};
    #[test]
    fn test_pointer_mut() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18)) = <(&str, &str, &str, &str, i32, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut data = json!(
            { "name" : "John Doe", "age" : 30, "address" : { "city" : "New York", "zip" :
            "10001" }, "phones" : ["12345", "67890"] }
        );
        debug_assert_eq!(
            data.pointer_mut(rug_fuzz_0).map(| v | * v = json!(rug_fuzz_1)), Some(())
        );
        debug_assert_eq!(data[rug_fuzz_2], json!("Jane Doe"));
        debug_assert_eq!(
            data.pointer_mut(rug_fuzz_3).map(| v | * v = json!(rug_fuzz_4)), Some(())
        );
        debug_assert_eq!(data[rug_fuzz_5], json!(31));
        debug_assert_eq!(
            data.pointer_mut(rug_fuzz_6).map(| v | * v = json!(rug_fuzz_7)), Some(())
        );
        debug_assert_eq!(data[rug_fuzz_8] [rug_fuzz_9], json!("Boston"));
        debug_assert_eq!(
            data.pointer_mut(rug_fuzz_10).map(| v | * v = json!(rug_fuzz_11)), Some(())
        );
        debug_assert_eq!(data[rug_fuzz_12], json!(["54321", "67890"]));
        debug_assert!(
            data.pointer_mut(rug_fuzz_13).is_none(),
            "Should be none as the index 3 doesn't exist"
        );
        debug_assert_eq!(
            data.pointer_mut(rug_fuzz_14).map(| v | * v = json!(rug_fuzz_15)), Some(())
        );
        debug_assert_eq!(data, json!("override the root"));
        debug_assert!(
            data.pointer_mut(rug_fuzz_16).is_none(), "Should not accept an empty string"
        );
        debug_assert_eq!(
            data.pointer_mut(rug_fuzz_17).map(| _ | ()), None,
            "Should be none for non-existent key"
        );
        debug_assert_eq!(
            data.pointer_mut(rug_fuzz_18).map(| _ | ()), None,
            "Should be none as 'abc' is not a valid index"
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_627 {
    use crate::Value;
    use crate::json;
    #[test]
    fn test_take_removes_and_replaces_with_null() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v = json!({ "x" : "y", "empty" : {} });
        debug_assert_eq!(v[rug_fuzz_0].take(), json!("y"));
        debug_assert_eq!(v[rug_fuzz_1], json!(null));
        debug_assert_eq!(v[rug_fuzz_2].take(), json!({}));
        debug_assert_eq!(v[rug_fuzz_3], json!(null));
        debug_assert_eq!(v[rug_fuzz_4].take(), json!(null));
        debug_assert_eq!(v[rug_fuzz_5], json!(null));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_798 {
    use crate::{json, to_value, Value};
    use serde::Serialize;
    #[derive(Serialize)]
    struct ExampleStruct {
        id: i32,
        name: String,
    }
    #[test]
    fn to_value_example_struct() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let example = ExampleStruct {
            id: rug_fuzz_0,
            name: rug_fuzz_1.to_owned(),
        };
        let expected = json!({ "id" : 42, "name" : "Serde", });
        let result = to_value(example).unwrap();
        debug_assert_eq!(result, expected);
             }
}
}
}    }
    #[test]
    fn to_value_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number = rug_fuzz_0;
        let expected = json!(rug_fuzz_1);
        let result = to_value(number).unwrap();
        debug_assert_eq!(result, expected);
             }
}
}
}    }
    #[test]
    fn to_value_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let text = rug_fuzz_0.to_owned();
        let expected = json!(rug_fuzz_1);
        let result = to_value(&text).unwrap();
        debug_assert_eq!(result, expected);
             }
}
}
}    }
    #[test]
    fn to_value_map() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = crate::Map::new();
        map.insert(rug_fuzz_0.to_string(), json!(rug_fuzz_1));
        let expected = json!({ "key" : "value", });
        let result = to_value(map).unwrap();
        debug_assert_eq!(result, expected);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "key must be a string")]
    fn to_value_non_string_key_map() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = crate::Map::new();
        let non_string_key = json!(rug_fuzz_0);
        map.insert(non_string_key.to_string(), json!(rug_fuzz_1));
        to_value(map).unwrap();
             }
}
}
}    }
    #[derive(Serialize)]
    enum ExampleEnum {
        VariantA,
        VariantB(i32),
    }
    #[test]
    fn to_value_enum() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let example_variant_a = ExampleEnum::VariantA;
        let expected_a = json!(rug_fuzz_0);
        let result_a = to_value(example_variant_a).unwrap();
        debug_assert_eq!(result_a, expected_a);
        let example_variant_b = ExampleEnum::VariantB(rug_fuzz_1);
        let expected_b = json!({ "VariantB" : 42 });
        let result_b = to_value(example_variant_b).unwrap();
        debug_assert_eq!(result_b, expected_b);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_147 {
    use super::*;
    #[test]
    fn test_parse_index() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &str = rug_fuzz_0;
        debug_assert_eq!(crate ::value::parse_index(p0), Some(123));
        let p0: &str = rug_fuzz_1;
        debug_assert_eq!(crate ::value::parse_index(p0), None);
        let p0: &str = rug_fuzz_2;
        debug_assert_eq!(crate ::value::parse_index(p0), None);
        let p0: &str = rug_fuzz_3;
        debug_assert_eq!(crate ::value::parse_index(p0), Some(0));
        let p0: &str = rug_fuzz_4;
        debug_assert_eq!(crate ::value::parse_index(p0), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_148 {
    use crate::{Error, value::from_value, Value};
    use serde::de::DeserializeOwned;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Value = Value::Bool(rug_fuzz_0);
        let _result: Result<bool, Error> = from_value(p0);
             }
}
}
}    }
}

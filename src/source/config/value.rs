use std::fmt;
use std::convert::TryInto;
use std::fmt::Write;
use serde_core::de::{Deserialize, Deserializer, Visitor};
use crate::source::config::{Result, Map};
use crate::source::config::error::{ConfigError, Unexpected};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ValueKind{
    #[default]
    Nil,
    Boolean(bool),
    I64(i64),
    I128(i128),
    U64(u64),
    U128(u128),
    Float(f64),
    String(String),
    Table(Table),
    Array(Array),
}

pub type Array = Vec<Value>;
pub type Table = Map<String, Value>;

impl<T> From<Option<T>> for ValueKind
where T: Into<Self>
{
    fn from(v: Option<T>) -> Self {
        match v {
            Some(value) => value.into(),
            None => Self::Nil,
        }
    }
}

impl From<String> for ValueKind {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl<'a> From<&'a str> for ValueKind {
    fn from(s: &'a str) -> Self {
        Self::String(s.into())
    }
}

impl From<i8> for ValueKind {
    fn from(i: i8) -> Self {
        Self::I64(i.into())
    }
}

impl From<i16> for ValueKind {
    fn from(i: i16) -> Self {
        Self::I64(i.into())
    }
}

impl From<i32> for ValueKind {
    fn from(i: i32) -> Self {
        Self::I64(i.into())
    }
}

impl From<i64> for ValueKind {
    fn from(i: i64) -> Self {
        Self::I64(i)
    }
}

impl From<i128> for ValueKind {
    fn from(i: i128) -> Self {
        Self::I128(i)
    }
}

impl From<u8> for ValueKind {
    fn from(i: u8) -> Self {
        Self::U64(i.into())
    }
}

impl From<u16> for ValueKind {
    fn from(i: u16) -> Self {
        Self::U64(i.into())
    }
}

impl From<u32> for ValueKind {
    fn from(i: u32) -> Self {
        Self::U64(i.into())
    }
}

impl From<u64> for ValueKind {
    fn from(i: u64) -> Self {
        Self::U64(i)
    }
}

impl From<u128> for ValueKind {
    fn from(i: u128) -> Self {
        Self::U128(i)
    }
}

impl From<bool> for ValueKind {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

impl From<f32> for ValueKind {
    fn from(f: f32) -> Self {
        Self::Float(f.into())
    }
}

impl From<f64> for ValueKind {
    fn from(f: f64) -> Self {
        Self::Float(f)
    }
}

impl<T> From<Map<String, T>> for ValueKind
where T: Into<Value>
{
    fn from(m: Map<String, T>) -> Self {
        let t = m.into_iter().map(|(k, v)|(k, v.into())).collect();
        Self::Table(t)
    }
}

impl<T> From<Vec<T>> for ValueKind
where T: Into<Value>
{
    fn from(v: Vec<T>) -> Self {
        Self::Array(v.into_iter().map(T::into).collect())
    }
}

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Nil => write!(f, "nil"),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::I64(i) => write!(f, "{i}"),
            Self::I128(i) => write!(f, "{i}"),
            Self::U64(i) => write!(f, "{i}"),
            Self::U128(i) => write!(f, "{i}"),
            Self::Float(float) => write!(f, "{float}"),
            Self::String(ref string) => write!(f, "{string}"),
            Self::Table(ref table) => {
                let mut s = String::new();
                for (k, v) in table.iter() {
                    write!(s, "{k} => {v}")?;
                };
                write!(f, "{{ {s} }}")
            },
            Self::Array(ref array) => {
                let mut s = String::new();
                for v in array.iter() {
                    write!(s, "{v}")?;
                };
                write!(f, "[{s}]")
            },
        }
    }
}


#[derive(Default, Debug, Clone, PartialEq)]
pub struct Value {
    origin: Option<String>,
    pub kind: ValueKind,
}

impl Value {
    pub fn new<V>(origin: Option<&String>, kind: V) -> Self
    where V: Into<ValueKind>
    {
        Self {
            origin: origin.cloned(),
            kind: kind.into(),
        }
    }

    pub fn origin(&self) -> Option<&str> {
        self.origin.as_ref().map(AsRef::as_ref)
    }

    // pub fn try_deserialize<'de, T: Deserialize<'de>>(self) -> Result<T> {
    //     T::deserialize(self)
    // }

    pub fn into_bool(self) -> Result<bool> {
        match self.kind {
            ValueKind::Boolean(b) => Ok(b),
            ValueKind::I64(i) => Ok(i != 0),
            ValueKind::I128(i) => Ok(i != 0),
            ValueKind::U64(i) => Ok(i != 0),
            ValueKind::U128(i) => Ok(i != 0),
            ValueKind::Float(f) => Ok(f != 0.0),
            ValueKind::String(ref s) => {
                match s.to_lowercase().as_ref() {
                    "1" | "true" | "on" | "yes" => Ok(true),
                    "0" | "false" | "off" | "no" => Ok(false),
                    other => Err(ConfigError::invalid_type(
                        self.origin.clone(),
                        Unexpected::Str(other.into()),
                        "a boolean"
                    )),
                }
            },
            ValueKind::Nil => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Unit,
                "a boolean"
            )),
            ValueKind::Table(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Map,
                "a boolean"
            )),
            ValueKind::Array(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Seq,
                "a boolean"
            ))
        }
    }

    pub fn into_int(self) -> Result<i64> {
        match self.kind {
            ValueKind::I64(i) => Ok(i),
            ValueKind::I128(i) => i.try_into().map_err(|_| {
                ConfigError::invalid_type(
                    self.origin,
                    Unexpected::I128(i),
                    "an signed 64 bit or less integer"
                )
            }),
            ValueKind::U64(i) => i.try_into().map_err(|_|{
                ConfigError::invalid_type(
                    self.origin,
                    Unexpected::U64(i),
                    "an signed 64 bit or less integer"
                )
            }),
            ValueKind::U128(i) => i.try_into().map_err(|_|{
                ConfigError::invalid_type(
                    self.origin,
                    Unexpected::U128(i),
                    "an signed 64 bit or less integer"
                )
            }),
            ValueKind::String(ref s) => {
                match s.to_lowercase().as_ref() {
                    "true" | "on" | "yes" => Ok(1),
                    "false" | "off" | "no" => Ok(0),
                    s => {
                        s.parse().map_err(|_| {
                            ConfigError::invalid_type(
                                self.origin.clone(),
                                Unexpected::Str(s.into()),
                                "an signed 64 bit or less integer"
                            )
                        })
                    }
                }
            },
            ValueKind::Boolean(b) => Ok(b as i64),
            ValueKind::Float(f) => Ok(f.round() as i64),
            ValueKind::Nil => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Unit,
                "an integer"
            )),
            ValueKind::Table(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Map,
                "an integer"
            )),
            ValueKind::Array(_) => Err(ConfigError::invalid_type(
               self.origin,
               Unexpected::Seq,
               "an integer"
            )),
        }
    }

    pub fn into_int128(self) -> Result<i128> {
        match self.kind {
            ValueKind::I64(i) => Ok(i.into()),
            ValueKind::I128(i) => Ok(i),
            ValueKind::U64(i) => Ok(i.into()),
            ValueKind::U128(i) => i.try_into().map_err(|_|{
                ConfigError::invalid_type(
                    self.origin,
                    Unexpected::U128(i),
                    "an signed 128 bit integer"
                )
            }),
            ValueKind::String(ref s) => {
                match s.to_lowercase().as_ref() {
                    "true" | "on" | "yes" => Ok(1),
                    "false" | "off" | "no" => Ok(0),
                    s => Err(ConfigError::invalid_type(
                        self.origin.clone(),
                        Unexpected::Str(s.into()),
                        "an signed 128 bit integer"
                    ))
                }
            },
            ValueKind::Boolean(b) => Ok(b.into()),
            ValueKind::Float(f) => Ok(f.round() as i128),
            ValueKind::Nil => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Unit,
                "an integer"
            )),
            ValueKind::Table(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Map,
                "an integer"
            )),
            ValueKind::Array(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Seq,
                "an integer"
            ))
        }
    }

    pub fn into_uint(self) -> Result<u64> {
        match self.kind {
            ValueKind::I64(i) => i.try_into().map_err(|_| {
                ConfigError::invalid_type(
                    self.origin,
                    Unexpected::I64(i),
                    "an unsigned 64 bit integer"
                )
            }),
            ValueKind::I128(i) => i.try_into().map_err(|_|{
                ConfigError::invalid_type(
                    self.origin,
                    Unexpected::I128(i),
                    "an unsigned 64 bit integer"
                )
            }),
            ValueKind::U64(i) => Ok(i),
            ValueKind::U128(i) => i.try_into().map_err(|_|{
                ConfigError::invalid_type(
                    self.origin,
                    Unexpected::U128(i),
                    "an unsigned 64 bit integer"
                )
            }),
            ValueKind::Boolean(b) => Ok(b.into()),
            ValueKind::String(ref s) => {
                match s.to_lowercase().as_ref() {
                    "true" | "on" | "yes" => Ok(1),
                    "false" | "off" | "no" => Ok(0),
                    s => {
                        s.parse().map_err(|_| {
                            ConfigError::invalid_type(
                                self.origin.clone(),
                                Unexpected::Str(s.into()),
                                "an unsigned 64 bit integer"
                            )
                        })
                    }
                }
            },
            ValueKind::Float(f) => Ok(f.round() as u64),
            ValueKind::Nil => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Unit,
                "an integer"
            )),
            ValueKind::Table(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Map,
                "an integer"
            )),
            ValueKind::Array(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Seq,
                "an integer"
            ))
        }
    }

    pub fn into_uint128(self) -> Result<u128> {
        match self.kind {
            ValueKind::U64(i) => Ok(i.into()),
            ValueKind::U128(i) => Ok(i),
            ValueKind::I64(i) => {
                i.try_into().map_err(|_|{
                    ConfigError::invalid_type(
                        self.origin,
                        Unexpected::I64(i),
                        "an unsigned 128 bit integer"
                    )
                })
            },
            ValueKind::I128(i) => {
                i.try_into().map_err(|_|{
                    ConfigError::invalid_type(
                        self.origin,
                        Unexpected::I128(i),
                        "an unsigned 128 bit integer"
                    )
                })
            },
            ValueKind::String(ref s) => {
                match s.to_lowercase().as_ref() {
                    "true" | "on" | "yes" => Ok(1),
                    "false" | "off" | "no" => Ok(0),
                    s => s.parse().map_err(|_|{
                        ConfigError::invalid_type(
                            self.origin.clone(),
                            Unexpected::Str(s.into()),
                            "an unsigned 128 bit integer"
                        )
                    })
                }
            },
            ValueKind::Boolean(b) => Ok(b.into()),
            ValueKind::Float(f) => Ok(f.round() as u128),
            ValueKind::Nil => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Unit,
                "an integer"
            )),
            ValueKind::Table(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Map,
                "an integer"
            )),
            ValueKind::Array(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Seq,
                "an integer"
            ))
        }
    }

    pub fn into_float(self) -> Result<f64> {
        match self.kind {
            ValueKind::I64(i) => Ok(i as f64),
            ValueKind::I128(i) => Ok(i as f64),
            ValueKind::U64(i) => Ok(i as f64),
            ValueKind::U128(i) => Ok(i as f64),
            ValueKind::Float(f) => Ok(f),
            ValueKind::Boolean(b) => Ok(if b { 1.0 } else { 0.0 }),
            ValueKind::Nil => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Unit,
                "an floating point"
            )),
            ValueKind::Table(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Map,
                "an floating point"
            )),
            ValueKind::Array(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Seq,
                "an floating point"
            )),
            ValueKind::String(ref s) => {
                match s.to_lowercase().as_ref() {
                    "true" | "on" | "yes" => Ok(1.0),
                    "false" | "off" | "no" => Ok(0.0),
                    s => Err(ConfigError::invalid_type(
                        self.origin.clone(),
                        Unexpected::Str(s.into()),
                        "an floating point"
                    ))
                }
            }
        }
    }

    pub fn into_string(self) -> Result<String> {
        match self.kind {
            ValueKind::String(s) => Ok(s),
            ValueKind::Boolean(b) => Ok(b.to_string()),
            ValueKind::I64(i) => Ok(i.to_string()),
            ValueKind::I128(i) => Ok(i.to_string()),
            ValueKind::U64(i) => Ok(i.to_string()),
            ValueKind::U128(i) => Ok(i.to_string()),
            ValueKind::Float(f) => Ok(f.to_string()),
            ValueKind::Nil => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Unit,
                "s string"
            )),
            ValueKind::Table(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Map,
                "s string"
            )),
            ValueKind::Array(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Seq,
                "s string"
            ))
        }
    }

    pub fn into_array(self) -> Result<Array> {
        match self.kind {
            ValueKind::Array(a) => Ok(a),
            ValueKind::Table(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Seq,
                "a array"
            )),
            ValueKind::Nil => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Unit,
                "a array"
            )),
            ValueKind::Float(f) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Float(f),
                "a array"
            )),
            ValueKind::String(ref s) => Err(ConfigError::invalid_type(
                self.origin.clone(),
                Unexpected::Str(s.into()),
                "a array"
            )),
            ValueKind::Boolean(b) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Bool(b),
                "a array"
            )),
            ValueKind::I64(i) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::I64(i),
                "a array"
            )),
            ValueKind::I128(i) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::I128(i),
                "a array"
            )),
            ValueKind::U64(u) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::U64(u),
                "a array"
            )),
            ValueKind::U128(u) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::U128(u),
                "a array"
            ))
        }
    }

    pub fn into_table(self) -> Result<Table> {
        match self.kind {
            ValueKind::Table(t) => Ok(t),
            ValueKind::Nil => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Unit,
                "a table"
            )),
            ValueKind::String(ref s) => Err(ConfigError::invalid_type(
                self.origin.clone(),
                Unexpected::Str(s.into()),
                "a table"
            )),
            ValueKind::Boolean(b) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Bool(b),
                "a table"
            )),
            ValueKind::Float(f) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Float(f),
                "a table"
            )),
            ValueKind::I64(i) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::I64(i),
                "a table"
            )),
            ValueKind::I128(i) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::I128(i),
                "a table"
            )),
            ValueKind::U64(u) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::U64(u),
                "a table"
            )),
            ValueKind::U128(u) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::U128(u),
                "a table"
            )),
            ValueKind::Array(_a) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Seq,
                "a table"
            ))
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    #[inline]
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("any valid configuration value")
            }

            #[inline]
            fn visit_bool<E>(self, value: bool) -> ::std::result::Result<Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_i8<E>(self, value: i8) -> ::std::result::Result<Value, E> {
                Ok((i64::from(value)).into())
            }

            #[inline]
            fn visit_i16<E>(self, value: i16) -> ::std::result::Result<Value, E> {
                Ok((i64::from(value)).into())
            }

            #[inline]
            fn visit_i32<E>(self, value: i32) -> ::std::result::Result<Value, E> {
                Ok((i64::from(value)).into())
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> ::std::result::Result<Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_i128<E>(self, value: i128) -> ::std::result::Result<Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_u8<E>(self, value: u8) -> ::std::result::Result<Value, E> {
                Ok((i64::from(value)).into())
            }

            #[inline]
            fn visit_u16<E>(self, value: u16) -> ::std::result::Result<Value, E> {
                Ok((i64::from(value)).into())
            }

            #[inline]
            fn visit_u32<E>(self, value: u32) -> ::std::result::Result<Value, E> {
                Ok((i64::from(value)).into())
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> ::std::result::Result<Value, E>
            where
                E: ::serde_core::de::Error,
            {
                let num: i64 = value.try_into().map_err(|_| {
                    E::invalid_type(::serde_core::de::Unexpected::Unsigned(value), &self)
                })?;
                Ok(num.into())
            }

            #[inline]
            fn visit_u128<E>(self, value: u128) -> ::std::result::Result<Value, E>
            where
                E: ::serde_core::de::Error,
            {
                let num: i128 = value.try_into().map_err(|_| {
                    E::invalid_type(
                        ::serde_core::de::Unexpected::Other(
                            format!("integer `{value}` as u128").as_str(),
                        ),
                        &self,
                    )
                })?;
                Ok(num.into())
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> ::std::result::Result<Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_str<E>(self, value: &str) -> ::std::result::Result<Value, E>
            where
                E: ::serde_core::de::Error,
            {
                self.visit_string(String::from(value))
            }

            #[inline]
            fn visit_string<E>(self, value: String) -> ::std::result::Result<Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_none<E>(self) -> ::std::result::Result<Value, E> {
                Ok(Value::new(None, ValueKind::Nil))
            }

            #[inline]
            fn visit_some<D>(self, deserializer: D) -> ::std::result::Result<Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }

            #[inline]
            fn visit_unit<E>(self) -> ::std::result::Result<Value, E> {
                Ok(Value::new(None, ValueKind::Nil))
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> ::std::result::Result<Value, V::Error>
            where
                V: ::serde_core::de::SeqAccess<'de>,
            {
                let mut vec = Array::new();

                while let Some(elem) = visitor.next_element()? {
                    vec.push(elem);
                }

                Ok(vec.into())
            }

            fn visit_map<V>(self, mut visitor: V) -> ::std::result::Result<Value, V::Error>
            where
                V: ::serde_core::de::MapAccess<'de>,
            {
                let mut values = Table::new();

                while let Some((key, value)) = visitor.next_entry()? {
                    values.insert(key, value);
                }

                Ok(values.into())
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

impl<T> From<T> for Value
where
    T: Into<ValueKind>,
{
    fn from(value: T) -> Self {
        Self {
            origin: None,
            kind: value.into(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}




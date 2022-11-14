use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use rocket::request::FromParam;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};
use uuid::{Uuid};

#[derive(Clone, Copy, PartialOrd, PartialEq, Ord, Eq)]
pub struct Guid {
    val: Uuid,
}

impl Debug for Guid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.val.to_string().as_str())
    }
}


impl Guid {
    pub fn new() -> Guid {
        Guid {
            val: Uuid::new_v4()
        }
    }

    pub fn from_str(val: &str) -> Result<Guid, String> {
        let uuid = Uuid::from_str(val);
        match uuid {
            Ok(val) => {
                Ok(Guid {
                    val
                })
            }
            Err(_) => { Err("Could not parse id".to_string()) }
        }
    }

    pub fn from_string(val: &String) -> Result<Guid, String> {
        let uuid = Uuid::from_str(val.as_str());
        match uuid {
            Ok(val) => {
                Ok(Guid {
                    val
                })
            }
            Err(_) => { Err("Could not parse id".to_string()) }
        }
    }

    pub fn to_string(&self) -> String {
        self.val.to_string()
    }
    
}

impl Display for Guid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl Serialize for Guid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        self.val.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Guid {
    fn deserialize<'d, D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let val: String = Deserialize::deserialize(deserializer)?;
        let guid = Guid::from_string(&val).unwrap();
        Ok(guid)
        // const FIELDS: &'static [&'static str] = &["val"];
        // deserializer.deserialize_struct("Guid", FIELDS, GuidVisitor)
    }
}

struct GuidVisitor;

impl<'de> Visitor<'de> for GuidVisitor {
    type Value = Guid;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a valid guid")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        Guid::from_str(v).map_err(|x| Error::custom(x))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> where E: Error {
        Guid::from_str(v).map_err(|x| Error::custom(x))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: Error {
        Guid::from_string(&v).map_err(|x| Error::custom(x))
    }
}

impl FromParam<'_> for Guid {
    type Error = String;

    fn from_param(param: &str) -> Result<Self, Self::Error> {
        Guid::from_str(param)
    }
}

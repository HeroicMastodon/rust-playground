use std::fmt::{Display, Formatter};
use std::str::FromStr;
use rocket::request::FromParam;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::{Uuid};

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug)]
pub struct Guid {
    val: Uuid,
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
        let val: &str = Deserialize::deserialize(deserializer)?;
        let guid = Guid::from_str(val).unwrap();
        Ok(guid)
    }
}

impl FromParam<'_> for Guid {
    type Error = String;

    fn from_param(param: &str) -> Result<Self, Self::Error> {
        Guid::from_str(param)
    }
}

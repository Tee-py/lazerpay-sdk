extern crate serde;
extern crate serde_json;

use std::fmt;
// use serde::{Deserialize, Serialize};
use serde::de::{self, value, Deserialize, Deserializer, Visitor, SeqAccess};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    #[serde(rename = "statusCode")]
    pub status_code: i16,
    #[serde(deserialize_with = "string_or_vec")]
    pub message: Vec<String>,
    pub error: Option<String>,
    pub status: Option<String>
}

fn string_or_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where D: Deserializer<'de>
{
    struct StringOrVec;

    impl<'de> Visitor<'de> for StringOrVec {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where E: de::Error
        {
            Ok(vec![s.to_owned()])
        }

        fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
            where S: SeqAccess<'de>
        {
            Deserialize::deserialize(value::SeqAccessDeserializer::new(seq))
        }
    }

    deserializer.deserialize_any(StringOrVec)
}

#[derive(Debug)]
pub enum Error {
    RequestError(ApiError),
    ClientError(reqwest::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::RequestError(error) => write!(f, "Request Error {:?}", error)?,
            Error::ClientError(error) => write!(f, "Client Error {:?}", error)?,
        };
        Ok(())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::ClientError(err)
    }
}

impl std::error::Error for Error {}

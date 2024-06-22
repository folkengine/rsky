use anyhow::Result;
use base64ct::{Base64, Encoding};
use chrono::offset::Utc as UtcOffset;
use chrono::DateTime;
use indexmap::IndexMap;
use rand::{distributions::Alphanumeric, Rng};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rsky_identity::did::atproto_data::VerificationMaterial;
use rsky_identity::types::DidDocument;
use serde::Serialize;
use serde_json::Value;
use std::time::SystemTime;
use thiserror::Error;
use urlencoding::encode;

pub const RFC3339_VARIANT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

#[derive(Error, Debug)]
pub enum BadContentTypeError {
    #[error("BadType: `{0}`")]
    BadType(String),
    #[error("Content-Type header is missing")]
    MissingType,
}

#[derive(Clone)]
pub struct ContentType {
    pub name: String,
}

/// Used mainly as a way to parse out content-type from request
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ContentType {
    type Error = BadContentTypeError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.content_type() {
            None => Outcome::Error((
                Status::UnsupportedMediaType,
                BadContentTypeError::MissingType,
            )),
            Some(content_type) => Outcome::Success(ContentType {
                name: content_type.to_string(),
            }),
        }
    }
}

pub fn now() -> String {
    let system_time = SystemTime::now();
    let dt: DateTime<UtcOffset> = system_time.into();
    format!("{}", dt.format(RFC3339_VARIANT))
}

pub fn get_random_str() -> String {
    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    token
}

pub fn struct_to_cbor<T: Serialize>(obj: T) -> Result<Vec<u8>> {
    // Encode object to json before dag-cbor because serde_ipld_dagcbor doesn't properly
    // sort by keys
    let json = serde_json::to_string(&obj)?;
    // Deserialize to IndexMap with preserve key order enabled. serde_ipld_dagcbor does not sort nested
    // objects properly by keys
    let map: IndexMap<String, Value> = serde_json::from_str(&json)?;
    let cbor_bytes = serde_ipld_dagcbor::to_vec(&map)?;

    Ok(cbor_bytes)
}

pub fn json_to_b64url<T: Serialize>(obj: &T) -> Result<String> {
    Ok(Base64::encode_string((&serde_json::to_string(obj)?).as_ref()).replace("=", ""))
}

pub fn encode_uri_component(input: &String) -> String {
    encode(input).to_string()
}

// based on did-doc.ts
pub fn get_did(doc: &DidDocument) -> String {
    doc.id.clone()
}

pub fn get_handle(doc: &DidDocument) -> Option<String> {
    match &doc.also_known_as {
        None => None,
        Some(aka) => {
            let found = aka.into_iter().find(|name| name.starts_with("at://"));
            match found {
                None => None,
                // strip off at:// prefix
                Some(found) => Some(found[5..].to_string()),
            }
        }
    }
}

pub fn get_verification_material(
    doc: &DidDocument,
    key_id: &String,
) -> Option<VerificationMaterial> {
    let did = get_did(doc);
    let keys = &doc.verification_method;
    if let Some(keys) = keys {
        let found = keys
            .into_iter()
            .find(|key| key.id == format!("#{key_id}") || key.id == format!("{did}#{key_id}"));
        match found {
            Some(found) if found.public_key_multibase.is_some() => {
                let found = found.clone();
                Some(VerificationMaterial {
                    r#type: found.r#type,
                    public_key_multibase: found.public_key_multibase.unwrap(),
                })
            }
            _ => None,
        }
    } else {
        None
    }
}

pub mod env;
pub mod ipld;
pub mod sign;
pub mod tid;
pub mod time;
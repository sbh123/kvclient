#[macro_use]
extern crate serde_derive;

#[macro_use]
pub mod macros;

extern crate serde;
extern crate serde_json;

extern crate uuid;

pub mod common;
pub mod client;
pub mod rpc;

use uuid::Uuid;
// use std::fmt;
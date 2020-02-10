#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![deny(intra_doc_link_resolution_failure)]
#![forbid(unsafe_code)]

//! Implementation of the client protocol for [MPD](https://musicpd.org). Supports binary responses
//! and command lists, provided they are initiated with the `command_list_ok_begin` command.
//!
//! Consists of a parser for MPD responses ([`parser`](parser/index.html)), and an implementation
//! of [Tokio](https://tokio.rs)'s
//! [codec](https://docs.rs/tokio-util/0.2.0/tokio_util/codec/index.html) subsystem to facilitate
//! asynchronous clients ([`codec`](codec/index.html)).
//!
//! Also provided are utilities for constructing [commands](command/index.html) and [filter
//! expressions](filter/index.html), as a special case of argument to commands.

pub mod codec;
pub mod command;
pub mod filter;
pub mod parser;
pub mod response;

pub use codec::{MpdCodec, MpdCodecError};
pub use command::Command;
pub use filter::Filter;
pub use parser::{greeting as parse_greeting, response as parse_response};
pub use response::Response;

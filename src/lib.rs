#![feature(unsafe_destructor, old_orphan_check)]
#![crate_name="openssl"]
#![crate_type="rlib"]
#![crate_type="dylib"]
#![doc(html_root_url="https://sfackler.github.io/doc/openssl")]

extern crate libc;
#[cfg(test)]
extern crate serialize;

extern crate "openssl-sys" as ffi;

mod macros;

pub mod asn1;
pub mod bn;
pub mod bio;
pub mod crypto;
pub mod ssl;
pub mod x509;

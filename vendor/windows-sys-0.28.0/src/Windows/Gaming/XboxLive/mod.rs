#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Gaming_XboxLive_Storage")]
pub mod Storage;
#[link(name = "windows")]
extern "system" {}

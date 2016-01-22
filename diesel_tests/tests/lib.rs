#![cfg_attr(not(feature = "syntex"), feature(custom_derive, plugin, custom_attribute, time2))]
#![cfg_attr(not(feature = "syntex"), plugin(diesel_codegen, dotenv_macros))]

extern crate quickcheck;
#[macro_use] extern crate diesel;

#[cfg(not(feature = "syntex"))]
include!("lib.in.rs");

#[cfg(feature = "syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));

mod associations;
mod expressions;
mod filter;
mod filter_operators;
mod find;
mod internal_details;
mod joins;
mod macros;
mod order;
mod perf_details;
mod select;
mod transactions;
mod types;
mod types_roundtrip;
mod debug;

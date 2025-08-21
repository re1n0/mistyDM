pub mod config;
mod consts;
pub mod utils;

pub use consts::*;

#[macro_use]
extern crate rust_i18n;

i18n!(
    "locales",
    fallback = "en",
    minify_key = true,
    minify_key_len = 12,
    minify_key_prefix = "t_"
);

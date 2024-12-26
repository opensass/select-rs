#![doc(
    html_logo_url = "https://raw.githubusercontent.com/opensass/select-rs/refs/heads/main/assets/logo.webp",
    html_favicon_url = "https://github.com/opensass/select-rs/blob/main/assets/favicon.png"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "yew")]
pub mod yew;

#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod errors;
mod modules;
mod types;
mod interfaces;
mod structures;

pub use crate::errors::{Error, Result};
pub use crate::modules::{DocumentModule};
pub use crate::types::{DocumentType};
pub use crate::interfaces::{DocumentInterface};
pub use crate::structures::{DocumentStructure};



#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod errors;

mod pages;
mod traits;

pub use crate::{
    errors::{DocumentError, DocumentErrorKind, DocumentResult},
    pages::{
        interfaces::DocumentInterface,
        modules::DocumentModule,
        packages::{PackageContext, PackagePage, PackageQuery},
        structures::DocumentStructure,
        types::DocumentType,
        PagesManager,
    },
    traits::PagedElement,
};

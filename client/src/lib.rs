//
// Copyright (c) Cryptic Dot
//
// Modification based on Orca Whirlpools (https://github.com/orca-so/whirlpools),
// originally licensed under the Apache License, Version 2.0, prior to February 26, 2025.
//
// Modifications licensed under FusionAMM SDK Source-Available License v1.0
// See the LICENSE file in the project root for license information.
//

#[rustfmt::skip]
mod generated;

mod consts;
mod pda;

#[cfg(feature = "fetch")]
mod gpa;

#[cfg(feature = "core-types")]
mod core_types;

pub use generated::accounts::*;
pub use generated::errors::*;
pub use generated::instructions::*;
pub use generated::programs::FUSIONAMM_ID as ID;
pub use generated::programs::*;
pub use generated::types::*;

#[cfg(feature = "fetch")]
pub use generated::shared::*;

#[cfg(feature = "fetch")]
pub(crate) use generated::*;

pub use consts::*;
pub use pda::*;

#[cfg(feature = "fetch")]
pub use gpa::*;

//
// Copyright (c) Cryptic Dot
//
// Modification based on Orca Whirlpools (https://github.com/orca-so/whirlpools),
// originally licensed under the Apache License, Version 2.0, prior to February 26, 2025.
//
// Modifications licensed under FusionAMM SDK Source-Available License v1.0
// See the LICENSE file in the project root for license information.
//

// FIXME: disable std for non-test builds to decrease wasm binary size.
// There is currently something in tsify that prevents this:
// https://github.com/madonoharu/tsify/issues/56
// #![cfg_attr(not(test), no_std)]
#![allow(clippy::useless_conversion)]

mod constants;
mod math;
mod quote;
mod types;

pub use constants::*;
pub use math::*;
pub use quote::*;
pub use types::*;

//
// Copyright (c) Cryptic Dot
//
// Modification based on Orca Whirlpools (https://github.com/orca-so/whirlpools),
// originally licensed under the Apache License, Version 2.0, prior to February 26, 2025.
//
// Modifications licensed under FusionAMM SDK Source-Available License v1.0
// See the LICENSE file in the project root for license information.
//

#![allow(non_snake_case)]

#[cfg(feature = "wasm")]
use fusionamm_macros::wasm_expose;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "wasm", wasm_expose)]
pub struct PositionRatio {
    pub ratio_a: u128,
    pub ratio_b: u128,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", wasm_expose)]
pub enum PositionStatus {
    PriceInRange,
    PriceBelowRange,
    PriceAboveRange,
    Invalid,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "wasm", wasm_expose)]
pub struct PositionFacade {
    pub liquidity: u128,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub fee_growth_checkpoint_a: u128,
    pub fee_owed_a: u64,
    pub fee_growth_checkpoint_b: u128,
    pub fee_owed_b: u64,
}

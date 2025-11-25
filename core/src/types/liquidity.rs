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
pub struct DecreaseLiquidityQuote {
    pub liquidity_delta: u128,
    pub token_est_a: u64,
    pub token_est_b: u64,
    pub token_min_a: u64,
    pub token_min_b: u64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "wasm", wasm_expose)]
pub struct IncreaseLiquidityQuote {
    pub liquidity_delta: u128,
    pub token_est_a: u64,
    pub token_est_b: u64,
    pub token_max_a: u64,
    pub token_max_b: u64,
}

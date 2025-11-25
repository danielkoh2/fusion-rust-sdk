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
pub struct ExactInSwapQuote {
    pub token_in: u64,
    pub token_est_out: u64,
    pub token_min_out: u64,
    pub trade_fee: u64,
    pub next_sqrt_price: u128,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "wasm", wasm_expose)]
pub struct ExactOutSwapQuote {
    pub token_out: u64,
    pub token_est_in: u64,
    pub token_max_in: u64,
    pub trade_fee: u64,
    pub next_sqrt_price: u128,
}

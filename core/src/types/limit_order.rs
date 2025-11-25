//
// Copyright (c) Cryptic Dot
//
// Licensed under FusionAMM SDK Source-Available License v1.0
// See the LICENSE file in the project root for license information.
//

#[cfg(feature = "wasm")]
use fusionamm_macros::wasm_expose;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "wasm", wasm_expose)]
pub struct LimitOrderFacade {
    pub tick_index: i32,
    pub amount: u64,
    pub a_to_b: bool,
    pub age: u64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "wasm", wasm_expose)]
pub struct LimitOrderDecreaseQuote {
    pub amount_out_a: u64,
    pub amount_out_b: u64,
    pub reward_a: u64,
    pub reward_b: u64,
}

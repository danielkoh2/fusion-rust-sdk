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
pub struct TransferFee {
    pub fee_bps: u16,
    pub max_fee: u64,
}

impl TransferFee {
    pub fn new(fee_bps: u16) -> Self {
        Self { fee_bps, max_fee: u64::MAX }
    }

    pub fn new_with_max(fee_bps: u16, max_fee: u64) -> Self {
        Self { fee_bps, max_fee }
    }
}

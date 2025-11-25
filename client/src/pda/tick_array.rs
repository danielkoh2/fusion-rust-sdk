//
// Copyright (c) Cryptic Dot
//
// Modification based on Orca Whirlpools (https://github.com/orca-so/whirlpools),
// originally licensed under the Apache License, Version 2.0, prior to February 26, 2025.
//
// Modifications licensed under FusionAMM SDK Source-Available License v1.0
// See the LICENSE file in the project root for license information.
//

use crate::generated::programs::FUSIONAMM_ID;
use solana_program::program_error::ProgramError;
use solana_pubkey::Pubkey;

pub fn get_tick_array_address(fusion_pool: &Pubkey, start_tick_index: i32) -> Result<(Pubkey, u8), ProgramError> {
    let start_tick_index_str = start_tick_index.to_string();
    let seeds = &[b"tick_array", fusion_pool.as_ref(), start_tick_index_str.as_bytes()];
    Pubkey::try_find_program_address(seeds, &FUSIONAMM_ID).ok_or(ProgramError::InvalidSeeds)
}

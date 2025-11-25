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

pub fn get_fusion_pool_address(token_mint_a: &Pubkey, token_mint_b: &Pubkey, tick_spacing: u16) -> Result<(Pubkey, u8), ProgramError> {
    let tick_spacing_bytes = tick_spacing.to_le_bytes();
    let seeds = &[b"fusion_pool", token_mint_a.as_ref(), token_mint_b.as_ref(), tick_spacing_bytes.as_ref()];
    Pubkey::try_find_program_address(seeds, &FUSIONAMM_ID).ok_or(ProgramError::InvalidSeeds)
}

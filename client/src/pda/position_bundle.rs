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

pub fn get_position_bundle_address(position_mint: &Pubkey) -> Result<(Pubkey, u8), ProgramError> {
    let seeds = &[b"position_bundle", position_mint.as_ref()];
    Pubkey::try_find_program_address(seeds, &FUSIONAMM_ID).ok_or(ProgramError::InvalidSeeds)
}

pub fn get_bundled_position_address(position_bundle_address: &Pubkey, bundle_index: u8) -> Result<(Pubkey, u8), ProgramError> {
    let bundle_index_str = bundle_index.to_string();
    let seeds = &[b"bundled_position", position_bundle_address.as_ref(), bundle_index_str.as_bytes()];
    Pubkey::try_find_program_address(seeds, &FUSIONAMM_ID).ok_or(ProgramError::InvalidSeeds)
}

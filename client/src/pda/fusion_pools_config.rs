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

pub fn get_fusion_pools_config_address() -> Result<(Pubkey, u8), ProgramError> {
    let seeds = &[b"config".as_ref()];
    Pubkey::try_find_program_address(seeds, &FUSIONAMM_ID).ok_or(ProgramError::InvalidSeeds)
}

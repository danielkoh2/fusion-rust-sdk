//
// Copyright (c) Cryptic Dot
//
// Modification based on Orca Whirlpools (https://github.com/orca-so/whirlpools),
// originally licensed under the Apache License, Version 2.0, prior to February 26, 2025.
//
// Modifications licensed under FusionAMM SDK Source-Available License v1.0
// See the LICENSE file in the project root for license information.
//

use fusionamm_core::PositionFacade;

use crate::Position;

impl From<Position> for PositionFacade {
    fn from(val: Position) -> Self {
        PositionFacade {
            liquidity: val.liquidity,
            tick_lower_index: val.tick_lower_index,
            tick_upper_index: val.tick_upper_index,
            fee_growth_checkpoint_a: val.fee_growth_checkpoint_a,
            fee_growth_checkpoint_b: val.fee_growth_checkpoint_b,
            fee_owed_a: val.fee_owed_a,
            fee_owed_b: val.fee_owed_b,
        }
    }
}

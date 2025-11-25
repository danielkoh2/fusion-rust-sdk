//
// Copyright (c) Cryptic Dot
//
// Modification based on Orca Whirlpools (https://github.com/orca-so/whirlpools),
// originally licensed under the Apache License, Version 2.0, prior to February 26, 2025.
//
// Modifications licensed under FusionAMM SDK Source-Available License v1.0
// See the LICENSE file in the project root for license information.
//

use fusionamm_core::TickRange;

use crate::Position;

impl From<Position> for TickRange {
    fn from(val: Position) -> Self {
        TickRange {
            tick_lower_index: val.tick_lower_index,
            tick_upper_index: val.tick_upper_index,
        }
    }
}

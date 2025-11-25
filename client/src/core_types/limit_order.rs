//
// Copyright (c) Cryptic Dot
//
// Modification based on Orca Whirlpools (https://github.com/orca-so/whirlpools),
// originally licensed under the Apache License, Version 2.0, prior to February 26, 2025.
//
// Modifications licensed under FusionAMM SDK Source-Available License v1.0
// See the LICENSE file in the project root for license information.
//

use fusionamm_core::LimitOrderFacade;

use crate::LimitOrder;

impl From<LimitOrder> for LimitOrderFacade {
    fn from(val: LimitOrder) -> Self {
        LimitOrderFacade {
            tick_index: val.tick_index,
            amount: val.amount,
            a_to_b: val.a_to_b,
            age: val.age,
        }
    }
}

//
// Copyright (c) Cryptic Dot
//
// Modification based on Orca Whirlpools (https://github.com/orca-so/whirlpools),
// originally licensed under the Apache License, Version 2.0, prior to February 26, 2025.
//
// Modifications licensed under FusionAMM SDK Source-Available License v1.0
// See the LICENSE file in the project root for license information.
//

use fusionamm_core::{TickArrayFacade, TickFacade};

use crate::{Tick, TickArray};

impl From<TickArray> for TickArrayFacade {
    fn from(val: TickArray) -> Self {
        TickArrayFacade {
            start_tick_index: val.start_tick_index,
            ticks: val.ticks.map(|tick| tick.into()),
        }
    }
}

impl From<Tick> for TickFacade {
    fn from(val: Tick) -> Self {
        TickFacade {
            liquidity_net: val.liquidity_net,
            liquidity_gross: val.liquidity_gross,
            initialized: val.initialized,
            fee_growth_outside_a: val.fee_growth_outside_a,
            fee_growth_outside_b: val.fee_growth_outside_b,
            age: val.age,
            open_orders_input: val.open_orders_input,
            part_filled_orders_input: val.part_filled_orders_input,
            part_filled_orders_remaining_input: val.part_filled_orders_remaining_input,
            fulfilled_a_to_b_orders_input: val.fulfilled_a_to_b_orders_input,
            fulfilled_b_to_a_orders_input: val.fulfilled_b_to_a_orders_input,
        }
    }
}

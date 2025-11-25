//
// Copyright (c) Cryptic Dot
//
// Licensed under FusionAMM SDK Source-Available License v1.0
// See the LICENSE file in the project root for license information.
//

use crate::{div_by_sqrt_price_squared, mul_by_sqrt_price_squared, CoreError};

/// Computes the limit order output amount by input amount.
/// ### Parameters
/// - `input_amount` - Input amount.
/// - `a_to_b_order` - The limit order direction.
/// - `sqrt_price` - Square root price
/// - `round_up` - Round up if true
pub fn get_limit_order_output_amount(input_amount: u64, a_to_b_order: bool, sqrt_price: u128, round_up: bool) -> Result<u64, CoreError> {
    let output_amount = if a_to_b_order {
        mul_by_sqrt_price_squared(input_amount, sqrt_price, round_up)?
    } else {
        div_by_sqrt_price_squared(input_amount, sqrt_price, round_up)?
    };
    Ok(output_amount)
}

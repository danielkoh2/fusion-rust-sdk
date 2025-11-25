//
// Copyright (c) Cryptic Dot
//
// Modification based on Orca Whirlpools (https://github.com/orca-so/whirlpools),
// originally licensed under the Apache License, Version 2.0, prior to February 26, 2025.
//
// Modifications licensed under FusionAMM SDK Source-Available License v1.0
// See the LICENSE file in the project root for license information.
//

use fusionamm_client::{fetch_all_fusion_pool_with_filter, get_fusion_pool_address, DecodedAccount, FusionPool, FusionPoolFilter};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use std::error::Error;

use crate::order_mints;

#[cfg(not(doctest))]
/// Fetches the details of a specific Concentrated Liquidity Pool.
///
/// This function retrieves information about a pool for the specified tick spacing.
/// It determines whether the pool is initialized or not and returns the corresponding details.
///
/// # Arguments
///
/// * `rpc` - A reference to the Solana RPC client.
/// * `token_1` - The public key of the first token mint in the pool.
/// * `token_2` - The public key of the second token mint in the pool.
/// * `tick_spacing` - The tick spacing of the pool.
///
/// # Returns
///
/// A `Result` containing `PoolInfo`:
/// * `PoolInfo::Initialized` if the pool is initialized, including the pool's state and price.
/// * `PoolInfo::Uninitialized` if the pool is not yet initialized, including configuration details.
///
/// # Errors
///
/// This function will return an error if:
/// - Any required account or mint information cannot be fetched.
/// - The pool or its configuration details are invalid.
pub async fn fetch_fusion_pool_by_token_pair_and_tick_spacing(
    rpc: &RpcClient,
    token_1: Pubkey,
    token_2: Pubkey,
    tick_spacing: u16,
) -> Result<DecodedAccount<FusionPool>, Box<dyn Error>> {
    let [token_a, token_b] = order_mints(token_1, token_2);
    let fusion_pool_address = get_fusion_pool_address(&token_a, &token_b, tick_spacing)?.0;

    let fusion_pool_account = rpc.get_account(&fusion_pool_address).await?;
    let fusion_pool = FusionPool::from_bytes(&fusion_pool_account.data)?;

    Ok(DecodedAccount {
        address: fusion_pool_address,
        account: fusion_pool_account,
        data: fusion_pool,
    })
}

#[cfg(not(doctest))]
/// Fetches all possible liquidity pools between two token mints in fusion pools.
///
/// This function retrieves information about all pools between the specified token mints,
/// including both initialized and uninitialized pools. If a pool does not exist, it creates
/// a placeholder account for the uninitialized pool with default configuration details.
///
/// # Arguments
///
/// * `rpc` - A reference to the Solana RPC client.
/// * `token_1` - The public key of the first token mint in the pool.
/// * `token_2` - The public key of the second token mint in the pool.
///
/// # Returns
///
/// A `Result` containing a `Vec<PoolInfo>`:
/// * `PoolInfo::Initialized` for initialized pools, including pool state and price.
/// * `PoolInfo::Uninitialized` for uninitialized pools, including configuration details.
///
/// # Errors
///
/// This function will return an error if:
/// - Any required account or mint information cannot be fetched.
/// - The pool or its configuration details are invalid.
///
pub async fn fetch_fusion_pools_by_token_pair(
    rpc: &RpcClient,
    token_1: Pubkey,
    token_2: Pubkey,
) -> Result<Vec<DecodedAccount<FusionPool>>, Box<dyn Error>> {
    let [token_a, token_b] = order_mints(token_1, token_2);

    let account_infos = rpc.get_multiple_accounts(&[token_a, token_b]).await?;

    account_infos[0].as_ref().ok_or(format!("Mint {} not found", token_a))?;
    account_infos[1].as_ref().ok_or(format!("Mint {} not found", token_b))?;

    let fusion_pools =
        fetch_all_fusion_pool_with_filter(rpc, vec![FusionPoolFilter::TokenMintA(token_a), FusionPoolFilter::TokenMintB(token_b)]).await?;

    Ok(fusion_pools)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{setup_ata_with_amount, setup_fusion_pool, setup_mint_with_decimals, RpcContext};
    use serial_test::serial;
    use solana_program_test::tokio;

    struct TestContext {
        ctx: RpcContext,
        mint_a: Pubkey,
        mint_b: Pubkey,
        concentrated_pool: Pubkey,
    }

    impl TestContext {
        async fn new() -> Result<Self, Box<dyn Error>> {
            let ctx = RpcContext::new().await;
            let mint_a = setup_mint_with_decimals(&ctx, 9).await?;
            let mint_b = setup_mint_with_decimals(&ctx, 9).await?;

            setup_ata_with_amount(&ctx, mint_a, 500_000_000_000).await?;
            setup_ata_with_amount(&ctx, mint_b, 500_000_000_000).await?;

            // Setup all pools
            let concentrated_pool = setup_fusion_pool(&ctx, mint_a, mint_b, 64, 300).await?;

            Ok(Self {
                ctx,
                mint_a,
                mint_b,
                concentrated_pool,
            })
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_fetch_concentrated_liquidity_pool() {
        let test_ctx = TestContext::new().await.unwrap();

        let pool = fetch_fusion_pool_by_token_pair_and_tick_spacing(&test_ctx.ctx.rpc, test_ctx.mint_a, test_ctx.mint_b, 64)
            .await
            .unwrap();

        assert_eq!(pool.data.liquidity, 0);
        assert_eq!(pool.data.tick_spacing, 64);
        assert_eq!(pool.address, test_ctx.concentrated_pool);
        assert_eq!(pool.data.token_mint_a, test_ctx.mint_a);
        assert_eq!(pool.data.token_mint_b, test_ctx.mint_b);
        assert_eq!(pool.data.fee_rate, 300);
        assert_eq!(pool.data.protocol_fee_rate, 0);
    }
}

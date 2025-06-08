use orca_whirlpools::{
    fetch_oracle, set_whirlpools_config_address, swap_instructions, SwapType, WhirlpoolsConfigInput,
};
use orca_whirlpools_client::{get_oracle_address, Oracle, Whirlpool};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use std::str::FromStr;

#[tokio::test]
async fn test_oracle() {
    let rpc = RpcClient::new("https://solana-rpc.publicnode.com".to_string());
    let whirlpool_address =
        Pubkey::from_str("FwewVm8u6tFPGewAyHmWAqad9hmF7mvqxK4mJ7iNqqGC").unwrap();
    let whirlpool_info = rpc.get_account(&whirlpool_address).await.unwrap();
    let whirlpool = Whirlpool::from_bytes(&whirlpool_info.data).unwrap();
    let (address, _) = get_oracle_address(&whirlpool_address).unwrap();
    let oracle = fetch_oracle(&rpc, address, &whirlpool).await;
    println!("{:#?}", oracle);
}

#[tokio::test]
async fn main() {
    set_whirlpools_config_address(WhirlpoolsConfigInput::SolanaMainnet).unwrap();
    let rpc = RpcClient::new("https://solana-rpc.publicnode.com".to_string());
    let wallet = Keypair::new();
    let whirlpool_address =
        Pubkey::from_str("FwewVm8u6tFPGewAyHmWAqad9hmF7mvqxK4mJ7iNqqGC").unwrap();
    let mint_address = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
    let input_amount = 10_u64.pow(9);

    let result = swap_instructions(
        &rpc,
        whirlpool_address,
        input_amount,
        mint_address,
        SwapType::ExactIn,
        None,
        Some(wallet.pubkey()),
    )
    .await
    .unwrap();

    println!("Quote estimated token out: {:#?}", result.quote);
}

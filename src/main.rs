use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoin::Network;
use anyhow::Result;
use bitcoincore_rpc::json::{GetBlockTemplateModes, GetBlockTemplateRules, GetBlockTemplateCapabilities};

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to Bitcoin Core RPC in regtest mode
    let rpc = Client::new(
        "http://localhost:18443",  // Regtest RPC port
        Auth::UserPass("paul".to_string(), "changeme".to_string()),
    )?;

    // Get blockchain info
    let blockchain_info = rpc.get_blockchain_info()?;
    println!("Blockchain Info: {:#?}", blockchain_info);

    // Get network info
    let network_info = rpc.get_network_info()?;
    println!("Network Info: {:#?}", network_info);

    // Get mining info
    let mining_info = rpc.get_mining_info()?;
    println!("Mining Info: {:#?}", mining_info);

    // Get mempool info
    let mempool_info = rpc.get_mempool_info()?;
    println!("Mempool Info: {:#?}", mempool_info);

    // Get the best block hash
    let best_block_hash = rpc.get_best_block_hash()?;
    println!("Best Block Hash: {}", best_block_hash);

    // Get block count
    let block_count = rpc.get_block_count()?;
    println!("Block Count: {}", block_count);

    // Get difficulty
    let difficulty = rpc.get_difficulty()?;
    println!("Difficulty: {}", difficulty);

    // Get connection count
    let connection_count = rpc.get_connection_count()?;
    println!("Connection Count: {}", connection_count);

    // Get peer info
    let peer_info = rpc.get_peer_info()?;
    println!("Peer Info: {:#?}", peer_info);

    // Get raw mempool
    let raw_mempool = rpc.get_raw_mempool()?;
    println!("Raw Mempool: {:#?}", raw_mempool);

    // Get mempool entry for a specific transaction
    if let Some(txid) = raw_mempool.first() {
        let mempool_entry = rpc.get_mempool_entry(txid)?;
        println!("Mempool Entry: {:#?}", mempool_entry);
    }

    // Get block template with required parameters
    let block_template = rpc.get_block_template(
        GetBlockTemplateModes::Template,
        &[GetBlockTemplateRules::SegWit],
        &[], // Empty capabilities array
    )?;
    println!("Block Template: {:#?}", block_template);

    // Get estimated fee
    let estimated_fee = rpc.estimate_smart_fee(6, None)?;
    println!("Estimated Fee: {:#?}", estimated_fee);

    // Get wallet info
    let wallet_info = rpc.get_wallet_info()?;
    println!("Wallet Info: {:#?}", wallet_info);

    Ok(())
}

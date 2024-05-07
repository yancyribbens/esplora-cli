//! Esplora Command Line Interface
//!
//! This binary provides A command line interface
//! for rust-esplora-client.

use bitcoin::Txid;
use clap::{Parser, Subcommand};
use esplora_client::Builder;

use bitcoin::Script;
use std::str::FromStr;

use bitcoin::consensus::encode::deserialize;
use bitcoin::Transaction;
use hex::test_hex_unwrap as hex;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(default_value = "https://blockstream.info/api")]
    network: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Get transaction option by id
    GetTx { id: String },
    /// Get transaction at block index
    GetTxAtBlockIndex { block_hash: String, index: String },
    /// Get transaction status by id
    GetTxStatus { id: String },
    /// Get block header by block hash
    GetHeaderByHash { block_hash: String },
    /// Get block status by block hash
    GetBlockStatus { block_hash: String },
    /// Get block by block hash
    GetBlockByHash { block_hash: String },
    /// Get transaction merkle proof by tx id
    GetMerkleProof { id: String },
    /// Get transaction merkle block inclusion proof by id
    GetMerkleBlock { id: String },
    /// Get output spending status by tx id and output index
    GetOutputStatus { id: String, index: String },
    /// Broadcast transaction.
    Broadcast { transaction_hex: String },
    /// Get blockchain tip height
    GetHeight {},
    /// Get current blockchain tip block hash
    GetTipHash {},
    /// Get block hash at height
    GetBlockHash { height: String },
    /// Get a fee estimate by confirmation target in sat/vB
    GetFeeEstimates {},
    /// Get confirmed transaction history for the specified address/scripthash sorted by date
    GetScriptHashTransactions {
        script_bytes: String,
        last_seen: Option<String>,
    },
    /// Get recent block summaries at the tip or at height if provided (max summaries is backend
    /// dependant).
    GetBlocks { height: Option<String> },
}

fn main() {
    let cli = Cli::parse();
    let builder = Builder::new(&cli.network);
    let blocking_client = builder.build_blocking();

    match &cli.command {
        Commands::GetTx { id } => {
            let tx_id: bitcoin::Txid = id.parse().unwrap();
            let r = blocking_client.get_tx(&tx_id).unwrap().unwrap();
            println!("{:#?}", r);
        }
        Commands::GetTxAtBlockIndex { block_hash, index } => {
            let hash: bitcoin::BlockHash = block_hash.parse().unwrap();
            let i: usize = index.parse().unwrap();
            let r = blocking_client
                .get_txid_at_block_index(&hash, i)
                .unwrap()
                .unwrap();
            println!("{:#?}", r);
        }
        Commands::GetTxStatus { id } => {
            let tx_id: bitcoin::Txid = id.parse().unwrap();
            let r = blocking_client.get_tx_status(&tx_id).unwrap();
            println!("{:#?}", r);
        }
        Commands::GetHeaderByHash { block_hash } => {
            let hash: bitcoin::BlockHash = block_hash.parse().unwrap();
            let r = blocking_client.get_header_by_hash(&hash).unwrap();
            println!("{:#?}", r);
        }
        Commands::GetBlockStatus { block_hash } => {
            let hash: bitcoin::BlockHash = block_hash.parse().unwrap();
            let r = blocking_client.get_block_status(&hash).unwrap();
            println!("{:#?}", r);
        }
        Commands::GetBlockByHash { block_hash } => {
            let hash: bitcoin::BlockHash = block_hash.parse().unwrap();
            let r = blocking_client.get_block_by_hash(&hash).unwrap();
            println!("{:#?}", r);
        }
        Commands::GetMerkleProof { id } => {
            let tx_id: bitcoin::Txid = id.parse().unwrap();
            let r = blocking_client.get_merkle_proof(&tx_id).unwrap().unwrap();
            println!("{:#?}", r);
        }
        Commands::GetMerkleBlock { id } => {
            let tx_id: bitcoin::Txid = id.parse().unwrap();
            let r = blocking_client.get_merkle_block(&tx_id).unwrap();
            println!("{:#?}", r);
        }
        Commands::GetOutputStatus { id, index } => {
            let tx_id: bitcoin::Txid = id.parse().unwrap();
            let i: u64 = index.parse().unwrap();
            let r = blocking_client
                .get_output_status(&tx_id, i)
                .unwrap()
                .unwrap();
            println!("{:#?}", r);
        }
        Commands::Broadcast { transaction_hex } => {
            let tx_bytes = hex!(transaction_hex);
            let tx: Transaction = deserialize(&tx_bytes).unwrap();
            blocking_client.broadcast(&tx).unwrap();
        }
        Commands::GetHeight {} => {
            let r = blocking_client.get_height().unwrap();
            println!("{:#?}", r);
        }
        Commands::GetTipHash {} => {
            let r = blocking_client.get_tip_hash().unwrap();
            println!("{:#?}", r);
        }
        Commands::GetBlockHash { height } => {
            let h: u32 = height.parse().unwrap();
            let r = blocking_client.get_block_hash(h).unwrap();
            println!("{:#?}", r);
        }
        Commands::GetFeeEstimates {} => {
            let r = blocking_client.get_fee_estimates().unwrap();
            println!("{:#?}", r);
        }
        Commands::GetScriptHashTransactions {
            script_bytes,
            last_seen,
        } => {
            let mut txid: Option<Txid> = None;
            if let Some(t) = last_seen {
                txid = Some(Txid::from_str(t).unwrap());
            }
            let b = script_bytes.as_bytes();
            let s = Script::from_bytes(b);
            let r = blocking_client.scripthash_txs(s, txid).unwrap();
            println!("{:#?}", r);
        }
        Commands::GetBlocks { height } => {
            let mut bh = None;
            if let Some(h) = height {
                bh = Some(h.parse().unwrap());
            }
            let r = blocking_client.get_blocks(bh).unwrap();
            println!("{:#?}", r);
        }
    }
}

mod abi;
mod pb;
use hex_literal::hex;
use pb::eth::rocketpool::v1 as rocketpool;

use abi::rocketpool::events as rocketpool_events;

use substreams::errors::Error;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

use substreams_ethereum::pb::eth::v2 as eth;

// Bored Ape Club Contract
const TRACKED_CONTRACT: [u8; 20] = hex!("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48");

substreams_ethereum::init!();

/// Extracts transfers events from the contract
#[substreams::handlers::map]
fn map_transfer(blk: eth::Block) -> Result<rocketpool::Transfers, substreams::errors::Error> {
    Ok(rocketpool::Transfers {
        transfers: blk
            .events::<rocketpool_events::Transfer>(&[&TRACKED_CONTRACT])
            .map(|(transfer, log)| {
                substreams::log::info!("NFT Transfer seen");

                rocketpool::Transfer {
                    trx_hash: log.receipt.transaction.hash.clone(),
                    from: transfer.from,
                    to: transfer.to,
                    value: transfer.value.to_u64(),
                    ordinal: log.block_index() as u64,
                }
            })
            .collect(),
    })
}

#[substreams::handlers::map]
pub fn graph_out(transfers: rocketpool::Transfers) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    for transfer in &transfers.transfers {
        let row = tables.create_row("Transfer");

        row.set("from", &transfer.from);
        row.set("to", &transfer.to);
        row.set("txHash", transfer.trx_hash);
        row.set("ordinal", transfer.ordinal);
    }
}

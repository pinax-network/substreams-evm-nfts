use common::logs_with_caller;
use proto::pb::evm::erc721::v1::{Events, Transfer};
use substreams_abis::evm::tokens::cryptopunks::events as cryptopunks;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<Events, substreams::errors::Error> {
    let mut events = Events::default();

    for trx in block.transactions() {
        for (log, caller) in logs_with_caller(&block, trx) {
            // -- Transfer --
            if let Some(event) = cryptopunks::PunkTransfer::match_and_decode(log) {
                events.transfers.push(Transfer {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),

                    // -- call --
                    caller: caller.clone(),

                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),

                    // -- event --
                    from: event.from.to_vec(),
                    to: event.to.to_vec(),
                    token_id: event.punk_index.to_string(),
                });
            }
        }
    }

    Ok(events)
}

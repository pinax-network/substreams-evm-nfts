use common::{logs_with_caller, Address, NULL_ADDRESS};
use proto::pb::evm::erc721::v1::{Events, Transfer};
use substreams_abis::evm::tokens::cryptopunks::events as cryptopunks;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<Events, substreams::errors::Error> {
    let mut events = Events::default();

    for trx in block.transactions() {
        let mut pending_to: Option<Address> = None;

        for (log, caller) in logs_with_caller(&block, trx) {
            // Small bug in the original CryptoPunks contract, the local variable bid is wiped just before the
            // PunkBought event is fired―so the event ends up logging a zero address (and value = 0) even though the
            // transfer itself happened correctly.
            // https://github.com/pinax-network/substreams-evm-nfts/issues/1
            if let Some(event) = cryptopunks::Transfer::match_and_decode(log) {
                pending_to = Some(event.to);
            };

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

            // -- Transfer (Assign) --
            if let Some(event) = cryptopunks::Assign::match_and_decode(log) {
                events.transfers.push(Transfer {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),

                    // -- call --
                    caller: caller.clone(),

                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),

                    // -- event --
                    from: NULL_ADDRESS.to_vec(),
                    to: event.to.to_vec(),
                    token_id: event.punk_index.to_string(),
                });
            }

            // -- Transfer (PunkBought) --
            if let Some(event) = cryptopunks::PunkBought::match_and_decode(log) {
                events.transfers.push(Transfer {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),

                    // -- call --
                    caller: caller.clone(),

                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),

                    // -- event --
                    from: event.from_address.to_vec(),
                    to: pending_to.clone().unwrap_or(event.to_address), // https://github.com/pinax-network/substreams-evm-nfts/issues/1
                    token_id: event.punk_index.to_string(),
                });
            }
        }
    }

    Ok(events)
}

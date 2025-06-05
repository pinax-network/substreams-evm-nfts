use common::{logs_with_caller, Address};
use proto::pb::evm::cryptopunks;
use substreams_abis::evm::tokens::cryptopunks::events;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::{Event, NULL_ADDRESS};

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<cryptopunks::v1::Events, substreams::errors::Error> {
    let mut events = cryptopunks::v1::Events::default();

    for trx in block.transactions() {
        let mut pending_to: Option<Address> = None;

        for (log, caller) in logs_with_caller(&block, trx) {
            // Small bug in the original CryptoPunks contract, the local variable bid is wiped just before the
            // PunkBought event is fired―so the event ends up logging a zero address (and value = 0) even though the
            // transfer itself happened correctly.
            // https://github.com/pinax-network/substreams-evm-nfts/issues/1
            if let Some(event) = events::Transfer::match_and_decode(log) {
                pending_to = Some(event.to);
            };

            // -- PunkTransfer --
            if let Some(event) = events::PunkTransfer::match_and_decode(log) {
                events.punk_transfers.push(cryptopunks::v1::PunkTransfer {
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
                    punk_index: event.punk_index.to_string(),
                });
            }

            // -- Assign --
            if let Some(event) = events::Assign::match_and_decode(log) {
                events.assigns.push(cryptopunks::v1::Assign {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),

                    // -- call --
                    caller: caller.clone(),

                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),

                    // -- event --
                    to: event.to.to_vec(),
                    punk_index: event.punk_index.to_string(),
                });
            }

            // -- PunkBought --
            if let Some(event) = events::PunkBought::match_and_decode(log) {
                // Bug in the original CryptoPunks contract, the local variable `bid` is wiped just before the
                // PunkBought event is fired―so the event ends up logging a zero address (and value = 0) even though the
                // transfer itself happened correctly.
                // This is a workaround to avoid the zero address in the event, but it means that the value will be
                // `None` if the transfer was not a purchase (e.g. a gift).
                // https://github.com/pinax-network/substreams-evm-nfts/issues/1
                let value = match event.to_address == NULL_ADDRESS {
                    true => None,
                    false => Some(event.value.to_string()),
                };
                events.punk_boughts.push(cryptopunks::v1::PunkBought {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),

                    // -- call --
                    caller: caller.clone(),

                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),

                    // -- event --
                    from_address: event.from_address.to_vec(),
                    to_address: pending_to.clone().unwrap_or(event.to_address), // https://github.com/pinax-network/substreams-evm-nfts/issues/1
                    punk_index: event.punk_index.to_string(),
                    value,
                });
            }

            // -- PunkOffered --
            if let Some(event) = events::PunkOffered::match_and_decode(log) {
                events.punk_offereds.push(cryptopunks::v1::PunkOffered {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),

                    // -- call --
                    caller: caller.clone(),

                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),

                    // -- event --
                    punk_index: event.punk_index.to_string(),
                    min_value: event.min_value.to_string(),
                    to_address: event.to_address.to_vec(),
                });
            }
            // -- PunkBidEntered --
            if let Some(event) = events::PunkBidEntered::match_and_decode(log) {
                events.punk_bid_entereds.push(cryptopunks::v1::PunkBidEntered {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),

                    // -- call --
                    caller: caller.clone(),

                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),

                    // -- event --
                    punk_index: event.punk_index.to_string(),
                    value: event.value.to_string(),
                    from_address: event.from_address.to_vec(),
                });
            }
            // -- PunkBidWithdrawn --
            if let Some(event) = events::PunkBidWithdrawn::match_and_decode(log) {
                events.punk_bid_withdrawns.push(cryptopunks::v1::PunkBidWithdrawn {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),

                    // -- call --
                    caller: caller.clone(),

                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),

                    // -- event --
                    punk_index: event.punk_index.to_string(),
                    from_address: event.from_address.to_vec(),
                    value: event.value.to_string(),
                });
            }
            // -- PunkNoLongerForSale --
            if let Some(event) = events::PunkNoLongerForSale::match_and_decode(log) {
                events.punk_no_longer_for_sales.push(cryptopunks::v1::PunkNoLongerForSale {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),

                    // -- call --
                    caller: caller.clone(),

                    // -- log --
                    ordinal: log.ordinal,
                    contract: log.address.to_vec(),

                    // -- event --
                    punk_index: event.punk_index.to_string(),
                });
            }
        }
    }

    Ok(events)
}

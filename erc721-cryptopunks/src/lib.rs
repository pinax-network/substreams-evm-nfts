use proto::pb::evm::cryptopunks::v1::Events as CryptoPunksEvents;
use proto::pb::evm::erc721::v1::{Events, Transfer};
use substreams_ethereum::NULL_ADDRESS;

#[substreams::handlers::map]
fn map_events(cryptopunks: CryptoPunksEvents) -> Result<Events, substreams::errors::Error> {
    let mut events = Events::default();

    for event in cryptopunks.punk_transfers {
        events.transfers.push(Transfer {
            // -- transaction --
            tx_hash: event.tx_hash.to_vec(),

            // -- call --
            caller: event.caller.clone(),

            // -- log --
            ordinal: event.ordinal,
            contract: event.contract.to_vec(),

            // -- event --
            from: event.from.to_vec(),
            to: event.to.to_vec(),
            token_id: event.punk_index.to_string(),
        });
    }

    for event in cryptopunks.assigns {
        events.transfers.push(Transfer {
            // -- transaction --
            tx_hash: event.tx_hash.to_vec(),

            // -- call --
            caller: event.caller.clone(),

            // -- log --
            ordinal: event.ordinal,
            contract: event.contract.to_vec(),

            // -- event --
            from: NULL_ADDRESS.to_vec(),
            to: event.to.to_vec(),
            token_id: event.punk_index.to_string(),
        });
    }

    for event in cryptopunks.punk_boughts {
        events.transfers.push(Transfer {
            // -- transaction --
            tx_hash: event.tx_hash.to_vec(),

            // -- call --
            caller: event.caller.clone(),

            // -- log --
            ordinal: event.ordinal,
            contract: event.contract.to_vec(),

            // -- event --
            from: event.from_address.to_vec(),
            to: event.to_address.to_vec(),
            token_id: event.punk_index.to_string(),
        });
    }

    Ok(events)
}

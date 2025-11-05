use prettytable::{Cell, Row, Table};
use base64::{Engine as _, engine::general_purpose};

use crate::{api, Error};

pub fn run(api_client: &api::Client) -> Result<(), Error> {
    let agents = api_client.list_agents()?;

    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("Agent ID"),
        Cell::new("Created At"),
        Cell::new("Last Seen At"),
        Cell::new("Identity Public Key"),
        Cell::new("Public Prekey"),
    ]));

    for agent in agents {
        let identity_public_key_base64 = general_purpose::STANDARD.encode(agent.identity_public_key);
        let public_prekey = general_purpose::STANDARD.encode(agent.public_prekey);
        table.add_row(Row::new(vec![
            Cell::new(&agent.id.to_string()),
            Cell::new(&agent.created_at.to_string()),
            Cell::new(&agent.last_seen_at.to_string()),
            Cell::new(&identity_public_key_base64),
            Cell::new(&public_prekey),
        ]));
    }

    table.printstd();

    Ok(())
}

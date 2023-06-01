use anyhow::{Context, Result};
use mysql::prelude::*;
use mysql::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Setting {
    port_1: u8,
    port_2: u8,
    port_3: u8,
    port_4: u8,
}

pub fn load_dbsettings(uri: &str) -> Result<Setting> {
    let pool =
        Pool::new(uri).with_context(|| format!("Failed to create connetion pool from {uri}."))?;
    let mut conn = pool.get_conn().context("Failed to connect to database.")?;

    let port_list = conn
        .query_map(
            "SELECT port_1, port_2, port_3, port_4 FROM config_port_speed",
            |(port_1, port_2, port_3, port_4)| Setting {
                port_1,
                port_2,
                port_3,
                port_4,
            },
        )
        .context("Failed to execute query")?;

    let val = port_list.get(0).unwrap().to_owned();

    Ok(val)
}

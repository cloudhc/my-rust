extern crate mysql;

use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Setting {
    port_1: u8,
    port_2: u8,
    port_3: u8,
    port_4: u8,
}

pub fn load_dbsettings(uri: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let pool = Pool::new(uri)?;
    let mut conn = pool.get_conn()?;

    let fetched_dbsettings = conn
        .query_map(
            "SELECT port_1, port_2, port_3, port_4 FROM config_port_speed",
            |(port_1, port_2, port_3, port_4)| {
               Setting { port_1, port_2, port_3, port_4 }
            },
        )?;

    println!("{:?}", fetched_dbsettings);

    Ok(())
}


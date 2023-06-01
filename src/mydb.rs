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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Vulnerability {
    src_ip: String,
    src_port: u16,
    dest_ip: String,
    dest_port: u16,
    proto: String,
    signature_id: String,
    signature: String,
    category: String,
    timestamp: u32,
    microsec: u32,
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

pub fn fetch_vulnerabilities(uri: &str) -> Result<()> {
    let pool =
        Pool::new(uri).with_context(|| format!("Failed to create connetion pool from {uri}."))?;
    let mut conn = pool.get_conn().context("Failed to connect to database.")?;

    let vulnerabilities_list = conn
        .query_map(
            "SELECT src_ip, src_port, dest_ip, dest_port, proto, signature_id, signature, category, timestamp, microsec FROM inst_cresult LIMIT 1",
            |(src_ip, src_port, dest_ip, dest_port, proto, signature_id, signature, category, timestamp, microsec)| Vulnerability {
                src_ip,
                src_port,
                dest_ip,
                dest_port,
                proto,
                signature_id,
                signature,
                category,
                timestamp,
                microsec,
            },
        )
        .context("Failed to execute query")?;

    for vul in vulnerabilities_list.iter() {
        println!("{:?}", vul);
    }

    Ok(())
}
use anyhow::{anyhow, Context, Result};
use futures::StreamExt;
use std::{path::PathBuf, collections::HashMap};
use structopt::StructOpt;

use multi_party_ecdsa::protocols::multi_party_ecdsa::gg_2020::state_machine::refresh::KeyRefresh;
use round_based::async_runtime::AsyncProtocol;

mod gg20_sm_client;
use gg20_sm_client::join_computation;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long, default_value = "http://localhost:8000/")]
    address: surf::Url,
    #[structopt(short, long, default_value = "default-refresh")]
    room: String,
    #[structopt(short, long)]
    output: PathBuf,
    #[structopt(short, long, default_value = "none")]
    local_share: String,

    #[structopt(short, long)]
    change: u16,
    #[structopt(short, long)]
    threshold: u16,
    #[structopt(short, long)]
    number_of_parties: u16,
}


#[tokio::main]
async fn main() -> Result<()> {
    let args: Cli = Cli::from_args();
    let mut output_file = tokio::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(args.output)
        .await
        .context("cannot create output file")?;

    let local_share = match Some(&args.local_share as &str) {
        Some("none") => {
            None
        }
        Some(path_str) => {
            serde_json::from_str(path_str).context("parse local share")?
        }
        None => None
    };

    let mut old_to_new_map: HashMap<u16, u16> = HashMap::new();
    for i in 1..args.number_of_parties + 1 {
        if i != args.change {
            old_to_new_map.insert(i, i);
        }
    }

    let new_party_index_option = match local_share {
        None => Some(args.change),
        _ => None,
    };

    let (_i, incoming, outgoing) = join_computation(args.address, &args.room)
        .await
        .context("join computation")?;

    let incoming = incoming.fuse();
    tokio::pin!(incoming);
    tokio::pin!(outgoing);

    let refresh = KeyRefresh::new(local_share, new_party_index_option, &old_to_new_map, args.threshold, args.number_of_parties)?;
    let output = AsyncProtocol::new(refresh, incoming, outgoing)
        .run()
        .await
        .map_err(|e| anyhow!("protocol execution terminated with error: {}", e))?;
    let output = serde_json::to_vec(&output).context("serialize output")?;
    tokio::io::copy(&mut output.as_slice(), &mut output_file)
        .await
        .context("save output to file")?;

    Ok(())
}

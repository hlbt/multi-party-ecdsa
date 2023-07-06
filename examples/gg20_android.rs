use anyhow::{anyhow, Context, Result};
use futures::StreamExt;
use std::path::PathBuf;
use structopt::StructOpt;

use multi_party_ecdsa::protocols::multi_party_ecdsa::gg_2020::state_machine::keygen::Keygen;
use round_based::async_runtime::AsyncProtocol;

mod gg20_sm_client;
use gg20_sm_client::join_computation;


// #[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring};

    #[no_mangle]
    pub unsafe extern fn Java_com_bxyz_mpc_Native_create(env: JNIEnv, _: JClass, jOutput: JString, jAddress: JString, jRoom: JString) -> jstring {

        let output = env.get_string(jOutput).expect("invalid jOutput string").as_ptr();
        let address = env.get_string(jAddress).expect("invalid jAddress string").as_ptr();
        let room = env.get_string(jRoom).expect("invalid jRoom string").as_ptr();
        let index = 1;
        let threshold = 1;
        let number_of_parties = 3;

        let mut output_file = tokio::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(output)
            .await
            .context("cannot create output file")?;

        let (_i, incoming, outgoing) = join_computation(address, &room)
            .await
            .context("join computation")?;

        let incoming = incoming.fuse();
        tokio::pin!(incoming);
        tokio::pin!(outgoing);

        let keygen = Keygen::new(index, threshold, number_of_parties)?;
        let output = AsyncProtocol::new(keygen, incoming, outgoing)
            .run()
            .await
            .map_err(|e| anyhow!("protocol execution terminated with error: {}", e))?;
        let output = serde_json::to_vec_pretty(&output).context("serialize output")?;
        tokio::io::copy(&mut output.as_slice(), &mut output_file)
            .await
            .context("save output to file")?;

        Ok(())
    }
}

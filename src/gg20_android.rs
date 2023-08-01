use anyhow::{anyhow, Context, Result};
use futures::StreamExt;
use std::path::PathBuf;
use structopt::StructOpt;

use multi_party_ecdsa::protocols::multi_party_ecdsa::gg_2020::state_machine::keygen::Keygen;
use round_based::async_runtime::AsyncProtocol;

use crate::gg20_sm_client::join_computation;


#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring, jint};

    #[no_mangle]
    pub unsafe extern fn Java_com_bxyz_mpc_Native_createKey(env: JNIEnv, _: JClass, index: jint, jAddress: JString, jRoom: JString) -> jstring {
        let address = env.get_string(jAddress).expect("invalid jAddress string").as_ptr();
        let room = env.get_string(jRoom).expect("invalid jRoom string").as_ptr();
        let index = 1;
        let threshold = 1;
        let number_of_parties = 3;


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

        output
    }
}

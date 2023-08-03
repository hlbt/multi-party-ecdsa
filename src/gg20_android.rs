
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    use anyhow::{anyhow, Context, Result};
    use std::path::PathBuf;
    use structopt::StructOpt;
    use surf::Url;
    
    use crate::protocols::multi_party_ecdsa::gg_2020::state_machine::keygen::Keygen;
    use round_based::async_runtime::AsyncProtocol;
    
    use crate::gg20_sm_client::join_computation;

    use jni::JNIEnv;
    use jni::JavaVM;
    use jni::objects::{JClass, JString, JObject, GlobalRef, JValueGen, JIntArray, JObjectArray, AsJArrayRaw, ReleaseMode};
    use jni::sys::{jstring, jint, jintArray, jobjectArray};

    use futures::{SinkExt, StreamExt, TryStreamExt};

    use curv::arithmetic::Converter;
    use curv::BigInt;

    use crate::protocols::multi_party_ecdsa::gg_2020::state_machine::sign::{
        OfflineStage, SignManual,
    };
    use round_based::Msg;

    use log::{debug};


    #[no_mangle]
    pub async fn create_key_async(address: surf::Url, room: String, index: u16, threshold: u16, number_of_parties: u16) -> Result<String> {
        debug!("PPYang create_key_async start");
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

        let output = String::from_utf8(output).expect("Found invalid UTF-8");

        Ok(output)
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_bxyz_mpc_Native_createKey(mut env: JNIEnv, _: JClass, index: jint, jAddress: JString, jRoom: JString) -> jstring {
        android_log::init("multi-party-ecdsa").unwrap();
        debug!("PPYang Java_com_bxyz_mpc_Native_createKey start");
        let address_binding = env.get_string(&jAddress).expect("invalid address string");
        let address = address_binding.to_string_lossy();

        let room_binding = env.get_string(&jRoom).expect("invalid jRoom string");
        let room = room_binding.to_string_lossy();

        let address = Url::parse(&address).expect("invalid address string");
        let index = index as u16;
        let threshold = 1;
        let number_of_parties = 3;


        debug!("PPYang Java_com_bxyz_mpc_Native_createKey call create_key_async address：{} room:{} index:{} threshold:{} number_of_parties:{}", address, room, index, threshold, number_of_parties);
        let task = create_key_async(address, room.to_string(), index, threshold, number_of_parties);
        let result = tokio::runtime::Runtime::new().unwrap().block_on(task);
        // debug!("PPYang result:{:?}", result);

        let result_java_string = env.new_string(result.unwrap()).expect("result");

        **result_java_string
    }

    pub async fn sign_data_async(address: surf::Url, room: String, parties: Vec<u16>, data_to_sign: String, local_share: String) -> Result<String> {
        debug!("PPYang sign_data_async start");
        let local_share = serde_json::from_str(&local_share).context("parse local share")?;
        let number_of_parties = parties.len();

        let (i, incoming, outgoing) =
            join_computation(address.clone(), &format!("{}-offline", room))
                .await
                .context("join offline computation")?;
    
        let incoming = incoming.fuse();
        tokio::pin!(incoming);
        tokio::pin!(outgoing);
    
        let signing = OfflineStage::new(i, parties, local_share)?;
        let completed_offline_stage = AsyncProtocol::new(signing, incoming, outgoing)
            .run()
            .await
            .map_err(|e| anyhow!("protocol execution terminated with error: {}", e))?;
    
        let (i, incoming, outgoing) = join_computation(address, &format!("{}-online", room))
            .await
            .context("join online computation")?;
    
        tokio::pin!(incoming);
        tokio::pin!(outgoing);
    
        let (signing, partial_signature) = SignManual::new(
            BigInt::from_bytes(data_to_sign.as_bytes()),
            completed_offline_stage,
        )?;
    
        outgoing
            .send(Msg {
                sender: i,
                receiver: None,
                body: partial_signature,
            })
            .await?;
    
        let partial_signatures: Vec<_> = incoming
            .take(number_of_parties - 1)
            .map_ok(|msg| msg.body)
            .try_collect()
            .await?;
        let signature = signing
            .complete(&partial_signatures)
            .context("online stage failed")?;
        let signature = serde_json::to_string(&signature).context("serialize signature")?;
    
        Ok(signature)
    }


    #[no_mangle]
    pub unsafe extern fn Java_com_bxyz_mpc_Native_signData(mut env: JNIEnv, _: JClass, jAddress: JString, jRoom: JString, parties: JIntArray, data_to_sign: JString, local_share: JString) -> jstring {
        debug!("PPYang Java_com_bxyz_mpc_Native_signData start");

        let address_binding = env.get_string(&jAddress).expect("invalid address string");
        let address = address_binding.to_string_lossy();
        let address = Url::parse(&address).expect("invalid address string");

        let room_binding = env.get_string(&jRoom).expect("invalid jRoom string");
        let room = room_binding.to_string_lossy().to_string();

        let len = env.get_array_length(&parties).expect("Can't get array elements");
        let elements = env.get_array_elements(&parties, ReleaseMode::NoCopyBack).expect("Can't get array elements");
        let parties: Vec<u16> = elements.iter().map(|int| *int as u16).collect();

        let data_to_sign_binding = env.get_string(&data_to_sign).expect("invalid data_to_sign string");
        let data_to_sign = data_to_sign_binding.to_string_lossy().to_string();

        let local_share_binding = env.get_string(&local_share).expect("invalid local_share string");
        let local_share = local_share_binding.to_string_lossy().to_string();


        let task = sign_data_async(address, room, parties, data_to_sign, local_share);
        let result = tokio::runtime::Runtime::new().unwrap().block_on(task);
        // debug!("PPYang result:{:?}", result);

        let result_java_string = env.new_string(result.unwrap()).expect("result");

        **result_java_string
    }
}

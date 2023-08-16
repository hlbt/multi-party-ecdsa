// use anyhow::{anyhow, Context, Result};

// use crate::protocols::multi_party_ecdsa::gg_2020::state_machine::keygen::Keygen;
// use round_based::async_runtime::AsyncProtocol;

// use crate::gg20_sm_client::join_computation;

// use futures::{SinkExt, StreamExt, TryStreamExt};

// use curv::arithmetic::Converter;
// use curv::BigInt;

// use crate::protocols::multi_party_ecdsa::gg_2020::state_machine::sign::{
//     OfflineStage, SignManual,
// };
// use round_based::Msg;

// use log::{debug};


// #[no_mangle]
// pub async fn create_key_async(address: surf::Url, room: String, index: u16, threshold: u16, number_of_parties: u16) -> Result<String> {
//     debug!("create_key_async start");
//     let (_i, incoming, outgoing) = join_computation(address, &room)
//         .await
//         .context("join computation")?;

//     let incoming = incoming.fuse();
//     tokio::pin!(incoming);
//     tokio::pin!(outgoing);

//     debug!("create_key_async start AsyncProtocol");
//     let keygen = Keygen::new(index, threshold, number_of_parties)?;
//     let output = AsyncProtocol::new(keygen, incoming, outgoing)
//         .run()
//         .await
//         .map_err(|e| anyhow!("protocol execution terminated with error: {}", e))?;
    
//     debug!("create_key_async has output");
//     let output = serde_json::to_vec_pretty(&output).context("serialize output")?;

//     let output = String::from_utf8(output).expect("Found invalid UTF-8");
//     debug!("create_key_async end output");

//     Ok(output)
// }

// #[no_mangle]
// pub async fn sign_data_async(address: surf::Url, room: String, parties: Vec<u16>, data_to_sign: String, local_share: String) -> Result<String> {
//     // debug!("PPYang sign_data_async start");
//     let local_share = serde_json::from_str(&local_share).context("parse local share")?;
//     let number_of_parties = parties.len();

//     let (i, incoming, outgoing) =
//         join_computation(address.clone(), &format!("{}-offline", room))
//             .await
//             .context("join offline computation")?;

//     let incoming = incoming.fuse();
//     tokio::pin!(incoming);
//     tokio::pin!(outgoing);

//     let signing = OfflineStage::new(i, parties, local_share)?;
//     let completed_offline_stage = AsyncProtocol::new(signing, incoming, outgoing)
//         .run()
//         .await
//         .map_err(|e| anyhow!("protocol execution terminated with error: {}", e))?;

//     let (i, incoming, outgoing) = join_computation(address, &format!("{}-online", room))
//         .await
//         .context("join online computation")?;

//     tokio::pin!(incoming);
//     tokio::pin!(outgoing);


//     let message = match hex::decode(data_to_sign.clone()) {
//         Ok(x) => x,
//         Err(_e) => data_to_sign.as_bytes().to_vec(),
//       };

//     let message = &message[..];

//     let (signing, partial_signature) = SignManual::new(
//         BigInt::from_bytes(message),
//         completed_offline_stage,
//     )?;


//     outgoing
//         .send(Msg {
//             sender: i,
//             receiver: None,
//             body: partial_signature,
//         })
//         .await?;

//     let partial_signatures: Vec<_> = incoming
//         .take(number_of_parties - 1)
//         .map_ok(|msg| msg.body)
//         .try_collect()
//         .await?;
//     let signature = signing
//         .complete(&partial_signatures)
//         .context("online stage failed")?;
//     let signature = serde_json::to_string(&signature).context("serialize signature")?;

//     Ok(signature)
// }


#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    use jni::JNIEnv;
    use jni::JavaVM;
    use jni::objects::*;
    use jni::sys::*;

    use log::*;
    use crate::gg20_keygen::*;
    use crate::gg20_refresh::*;
    use crate::gg20_signing::*;
    use crate::protocols::multi_party_ecdsa::gg_2020::state_machine::keygen::Keygen;
    use crate::protocols::multi_party_ecdsa::gg_2020::state_machine::refresh::KeyRefresh;
    use crate::protocols::multi_party_ecdsa::gg_2020::state_machine::sign::OfflineStage;

    #[no_mangle]
    pub unsafe extern fn Java_com_bxyz_mpc_Native_showLog(_env: JNIEnv, _: JClass) {
        android_log::init("multi-party-ecdsa").unwrap();
        debug!("start show log");
    } 

    #[no_mangle]
    pub unsafe extern fn Java_com_bxyz_mpc_Native_createKeygen(_env: JNIEnv, _: JClass, index: jint) -> jlong {
        let index = index as u16;
        crate::gg20_keygen::create_keygen(index, 1, 3) as jlong
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_bxyz_mpc_Native_createRefresh(_env: JNIEnv, _: JClass, key_json: jstring, new_party_index: jint) -> jlong {
        let json_key = match Some(key_json) {
            Some(json) => {
                let jkey_json = _env.get_string(&key_json).expect("invalid msg_json string");
                let str_key_json = jkey_json.to_string_lossy();
                Some(str_key_json.to_string())
            },
            None => None
        };
        let new_party_index = new_party_index as u16;
        crate::gg20_refresh::create_refresh(json_key, new_party_index, 1, 3) as jlong
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_bxyz_mpc_Native_createSigning(_env: JNIEnv, _: JClass, ms_index: jint, parties: JIntArray, key_json: jstring) -> jlong {
        let ms_index = ms_index as u16;

        let len = env.get_array_length(&parties).expect("Can't get array elements");
        let elements = env.get_array_elements(&parties, ReleaseMode::NoCopyBack).expect("Can't get array elements");
        let parties: Vec<u16> = elements.iter().map(|int| *int as u16).collect();

        let jkey_json = _env.get_string(&key_json).expect("invalid msg_json string");
        let str_key_json = jkey_json.to_string_lossy();

        crate::gg20_signing::create_signing(ms_index, parties, str_key_json.to_string()) as jlong
    }


    #[no_mangle]
    pub unsafe extern fn Java_com_bxyz_mpc_Native_free(_env: JNIEnv, _: JClass, call_type: jint, mpc_ptr: jlong) {
        let call_type = call_type as u16;
        match call_type {
            1 => crate::gg20_keygen::free_keygen(mpc_ptr as *mut Keygen),
            2 => crate::gg20_refresh::free_refresh(mpc_ptr as *mut KeyRefresh),
            3 => crate::gg20_signing::free_signing(mpc_ptr as *mut OfflineStage),
        };
    }
    
    #[no_mangle]
    pub unsafe extern fn Java_com_bxyz_mpc_Native_keygenHandleIncoming(mut _env: JNIEnv, _: JClass, call_type: jint, mpc_ptr: jlong, msg_json: JString) {
        let jmsg_json = _env.get_string(&msg_json).expect("invalid msg_json string");
        let str_msg_json = jmsg_json.to_string_lossy();

        let call_type = call_type as u16;
        match call_type {
            1 => crate::gg20_keygen::handle_incoming(mpc_ptr as *mut Keygen, str_msg_json.to_string()),
            2 => crate::gg20_refresh::handle_incoming(mpc_ptr as *mut KeyRefresh, str_msg_json.to_string()),
            3 => crate::gg20_signing::handle_incoming(mpc_ptr as *mut OfflineStage, str_msg_json.to_string()),
        };
    }
    
    #[no_mangle]
    pub unsafe extern fn Java_com_bxyz_mpc_Native_keygenWantsToProceed(_env: JNIEnv, _: JClass, call_type: jint, mpc_ptr: jlong) -> jboolean {
        let call_type = call_type as u16;
        let result = match call_type {
            1 => crate::gg20_keygen::wants_to_proceed(mpc_ptr as *mut Keygen),
            2 => crate::gg20_refresh::wants_to_proceed(mpc_ptr as *mut KeyRefresh),
            3 => crate::gg20_signing::wants_to_proceed(mpc_ptr as *mut OfflineStage),
        };
        result as jboolean
    }
    
    #[no_mangle]
    pub unsafe extern fn Java_com_bxyz_mpc_Native_keygenProceed(_env: JNIEnv, _: JClass, call_type: jint, mpc_ptr: jlong) {
        let call_type = call_type as u16;
        let result = match call_type {
            1 => crate::gg20_keygen::proceed(mpc_ptr as *mut Keygen),
            2 => crate::gg20_refresh::proceed(mpc_ptr as *mut KeyRefresh),
            3 => crate::gg20_signing::proceed(mpc_ptr as *mut OfflineStage),
        };
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_bxyz_mpc_Native_keygenMessageQueue(_env: JNIEnv, _: JClass, call_type: jint, mpc_ptr: jlong) -> jstring {
        let call_type = call_type as u16;
        let str_json = match call_type {
            1 => crate::gg20_keygen::message_queue(mpc_ptr as *mut Keygen),
            2 => crate::gg20_refresh::message_queue(mpc_ptr as *mut KeyRefresh),
            3 => crate::gg20_signing::message_queue(mpc_ptr as *mut OfflineStage),
        };
        let result_java_string = _env.new_string(str_json).expect("result");
        **result_java_string
    }
    
    #[no_mangle]
    pub unsafe extern fn Java_com_bxyz_mpc_Native_keygenIsFinished(_env: JNIEnv, _: JClass, call_type: jint, mpc_ptr: jlong) -> jboolean {
        let call_type = call_type as u16;
        let result = match call_type {
            1 => crate::gg20_keygen::is_finished(mpc_ptr as *mut Keygen),
            2 => crate::gg20_refresh::is_finished(mpc_ptr as *mut KeyRefresh),
            3 => crate::gg20_signing::is_finished(mpc_ptr as *mut OfflineStage),
        };
        result as jboolean
    }
    
    #[no_mangle]
    pub unsafe extern fn Java_com_bxyz_mpc_Native_keygenPickOutput(_env: JNIEnv, _: JClass, call_type: jint, mpc_ptr: jlong) -> jstring {
        let call_type = call_type as u16;
        let str_json = match call_type {
            1 => crate::gg20_keygen::pick_output(mpc_ptr as *mut Keygen),
            2 => crate::gg20_refresh::pick_output(mpc_ptr as *mut KeyRefresh),
            3 => crate::gg20_signing::pick_output(mpc_ptr as *mut OfflineStage),
        };
        let result_java_string = _env.new_string(str_json).expect("result");
        **result_java_string
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_bxyz_mpc_Native_keygenCurrentRound(_env: JNIEnv, _: JClass, call_type: jint, mpc_ptr: jlong) -> jint {
        let call_type = call_type as u16;
        let result = match call_type {
            1 => crate::gg20_keygen::current_round(mpc_ptr as *mut Keygen),
            2 => crate::gg20_refresh::current_round(mpc_ptr as *mut KeyRefresh),
            3 => crate::gg20_signing::current_round(mpc_ptr as *mut OfflineStage),
        };
        result as jint
    }
    

    // #[no_mangle]
    // pub unsafe extern fn Java_com_bxyz_mpc_Native_createKey(mut env: JNIEnv, _: JClass, index: jint, jAddress: JString, jRoom: JString) -> jstring {
    //     debug!("PPYang Java_com_bxyz_mpc_Native_createKey start");
    //     let address_binding = env.get_string(&jAddress).expect("invalid address string");
    //     let address = address_binding.to_string_lossy();

    //     let room_binding = env.get_string(&jRoom).expect("invalid jRoom string");
    //     let room = room_binding.to_string_lossy();

    //     let address = surf::Url::parse(&address).expect("invalid address string");
    //     let index = index as u16;
    //     let threshold = 1;
    //     let number_of_parties = 3;


    //     debug!("PPYang Java_com_bxyz_mpc_Native_createKey call create_key_async address：{} room:{} index:{} threshold:{} number_of_parties:{}", address, room, index, threshold, number_of_parties);
    //     let task = create_key_async(address, room.to_string(), index, threshold, number_of_parties);
    //     let result = tokio::runtime::Runtime::new().unwrap().block_on(task);
    //     debug!("PPYang Java_com_bxyz_mpc_Native_createKey result:{:?}", result);

    //     let result_java_string = env.new_string(result.unwrap()).expect("result");

    //     debug!("PPYang Java_com_bxyz_mpc_Native_createKey return result");
    //     **result_java_string
    // }


    // #[no_mangle]
    // pub unsafe extern fn Java_com_bxyz_mpc_Native_signData(mut env: JNIEnv, _: JClass, jAddress: JString, jRoom: JString, parties: JIntArray, data_to_sign: JString, local_share: JString) -> jstring {
    //     debug!("PPYang Java_com_bxyz_mpc_Native_signData start");

    //     let address_binding = env.get_string(&jAddress).expect("invalid address string");
    //     let address = address_binding.to_string_lossy();
    //     let address = surf::Url::parse(&address).expect("invalid address string");

    //     let room_binding = env.get_string(&jRoom).expect("invalid jRoom string");
    //     let room = room_binding.to_string_lossy().to_string();

    //     let len = env.get_array_length(&parties).expect("Can't get array elements");
    //     let elements = env.get_array_elements(&parties, ReleaseMode::NoCopyBack).expect("Can't get array elements");
    //     let parties: Vec<u16> = elements.iter().map(|int| *int as u16).collect();

    //     let data_to_sign_binding = env.get_string(&data_to_sign).expect("invalid data_to_sign string");
    //     let data_to_sign = data_to_sign_binding.to_string_lossy().to_string();

    //     let local_share_binding = env.get_string(&local_share).expect("invalid local_share string");
    //     let local_share = local_share_binding.to_string_lossy().to_string();


    //     let task = sign_data_async(address, room, parties, data_to_sign, local_share);
    //     let result = tokio::runtime::Runtime::new().unwrap().block_on(task);
    //     // debug!("PPYang result:{:?}", result);

    //     let result_java_string = env.new_string(result.unwrap()).expect("result");

    //     **result_java_string
    // }
}

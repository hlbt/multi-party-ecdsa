
use crate::protocols::multi_party_ecdsa::gg_2020::state_machine::refresh::{KeyRefresh, ProtocolMessage};
use crate::protocols::multi_party_ecdsa::gg_2020::state_machine::keygen::LocalKey;
use curv::elliptic::curves::secp256_k1::Secp256k1;
use std::collections::HashMap;
use round_based::Msg;
use round_based::StateMachine;

pub fn create_refresh(json_key: Option<String>, new_party_index: u16, threshold: u16, number_of_parties: u16) -> *mut KeyRefresh {
    let mut old_to_new_map: HashMap<u16, u16> = HashMap::new();
    for i in 1..number_of_parties + 1 {
        if i != new_party_index {
            old_to_new_map.insert(i, i);
        }
    }

    let local_share = match json_key {
        None => None,
        Some(json) => {
            Some(serde_json::from_str::<LocalKey<Secp256k1>>(&json).expect("deserialize message"))
        }
    };

    let new_party_index_option = match local_share {
        None => Some(new_party_index),
        _ => None,
    };

    let key_refresh = KeyRefresh::new(local_share, new_party_index_option, &old_to_new_map, threshold, number_of_parties).expect("expected keygen");
    Box::into_raw(Box::new(key_refresh))
}

pub fn free_refresh(key_refresh_ptr: *mut KeyRefresh) {
    if key_refresh_ptr.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(key_refresh_ptr));
    }
}

pub fn handle_incoming(key_refresh_ptr: *mut KeyRefresh, msg_json: String) {
    let key_refresh = unsafe {
        assert!(!key_refresh_ptr.is_null());
        &mut *key_refresh_ptr
    };
    let msg = serde_json::from_str::<Msg<ProtocolMessage>>(&msg_json).expect("deserialize message");
    key_refresh.handle_incoming(msg).unwrap();
}

pub fn wants_to_proceed(key_refresh_ptr: *mut KeyRefresh) -> bool {
    let key_refresh = unsafe {
        assert!(!key_refresh_ptr.is_null());
        &mut *key_refresh_ptr
    };
    key_refresh.wants_to_proceed()
}

pub fn proceed(key_refresh_ptr: *mut KeyRefresh) {
    let key_refresh = unsafe {
        assert!(!key_refresh_ptr.is_null());
        &mut *key_refresh_ptr
    };
    key_refresh.proceed().unwrap();
}

pub fn message_queue(key_refresh_ptr: *mut KeyRefresh) -> String {
    let key_refresh = unsafe {
        assert!(!key_refresh_ptr.is_null());
        &mut *key_refresh_ptr
    };
    let to_json = serde_json::to_string(&key_refresh.message_queue());
    key_refresh.message_queue().drain(..);
    to_json.unwrap()
}

pub fn is_finished(key_refresh_ptr: *mut KeyRefresh) -> bool {
    let key_refresh = unsafe {
        assert!(!key_refresh_ptr.is_null());
        &mut *key_refresh_ptr
    };
    key_refresh.is_finished()
}

pub fn pick_output(key_refresh_ptr: *mut KeyRefresh) -> String {
    let key_refresh = unsafe {
        assert!(!key_refresh_ptr.is_null());
        &mut *key_refresh_ptr
    };
    let result = key_refresh.pick_output().unwrap().expect("output");
    serde_json::to_string(&result).unwrap()
}

pub fn current_round(key_refresh_ptr: *mut KeyRefresh) -> u16 {
    let key_refresh = unsafe {
        assert!(!key_refresh_ptr.is_null());
        &mut *key_refresh_ptr
    };
    key_refresh.current_round()
}
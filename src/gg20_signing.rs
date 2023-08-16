use crate::protocols::multi_party_ecdsa::gg_2020::state_machine::sign::{OfflineStage, OfflineProtocolMessage};
use crate::protocols::multi_party_ecdsa::gg_2020::state_machine::keygen::LocalKey;
use curv::elliptic::curves::secp256_k1::Secp256k1;
use round_based::Msg;
use round_based::StateMachine;
use log::*;

pub fn create_signing(mg_index: u16, s_l: Vec<u16>, json_key: String) -> *mut OfflineStage {
    let key = serde_json::from_str::<LocalKey<Secp256k1>>(&json_key).expect("deserialize message");
    let signing = OfflineStage::new(mg_index, s_l, key).expect("expected keygen");
    Box::into_raw(Box::new(signing))
}

pub fn free_signing(signing_ptr: *mut OfflineStage) {
    if signing_ptr.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(signing_ptr));
    }
}

pub fn handle_incoming(signing_ptr: *mut OfflineStage, msg_json: String) {
    let signing = unsafe {
        assert!(!signing_ptr.is_null());
        &mut *signing_ptr
    };
    let msg = serde_json::from_str::<Msg<OfflineProtocolMessage>>(&msg_json).expect("deserialize message");
    signing.handle_incoming(msg).unwrap();
}

pub fn wants_to_proceed(signing_ptr: *mut OfflineStage) -> bool {
    let signing = unsafe {
        assert!(!signing_ptr.is_null());
        &mut *signing_ptr
    };
    signing.wants_to_proceed()
}

pub fn proceed(signing_ptr: *mut OfflineStage) {
    let signing = unsafe {
        assert!(!signing_ptr.is_null());
        &mut *signing_ptr
    };
    signing.proceed().unwrap();
}

pub fn message_queue(signing_ptr: *mut OfflineStage) -> String {
    let signing = unsafe {
        assert!(!signing_ptr.is_null());
        &mut *signing_ptr
    };
    let to_json = serde_json::to_string(&signing.message_queue());
    signing.message_queue().drain(..);
    to_json.unwrap()
}

pub fn is_finished(signing_ptr: *mut OfflineStage) -> bool {
    let signing = unsafe {
        assert!(!signing_ptr.is_null());
        &mut *signing_ptr
    };
    signing.is_finished()
}

pub fn pick_output(signing_ptr: *mut OfflineStage) -> String {
    let signing = unsafe {
        assert!(!signing_ptr.is_null());
        &mut *signing_ptr
    };

    let result = signing.pick_output().unwrap().expect("output");
    serde_json::to_string(&result).unwrap()
}

pub fn current_round(signing_ptr: *mut OfflineStage) -> u16 {
    let signing = unsafe {
        assert!(!signing_ptr.is_null());
        &mut *signing_ptr
    };
    signing.current_round()
}
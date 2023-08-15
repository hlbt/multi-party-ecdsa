use crate::protocols::multi_party_ecdsa::gg_2020::state_machine::keygen::{Keygen, ProtocolMessage};
use round_based::Msg;
use round_based::StateMachine;
use log::*;

pub fn create_keygen(index: u16, threshold: u16, number_of_parties: u16) -> *mut Keygen {
    let keygen = Keygen::new(index, threshold, number_of_parties).expect("expected keygen");
    Box::into_raw(Box::new(keygen))
}

pub fn free_keygen(keygen_ptr: *mut Keygen) {
    if keygen_ptr.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(keygen_ptr));
    }
}

pub fn handle_incoming(keygen_ptr: *mut Keygen, msg_json: String) {
    debug!("gg20_keygen handle_incoming start");
    let keygent = unsafe {
        assert!(!keygen_ptr.is_null());
        &mut *keygen_ptr
    };
    debug!("gg20_keygen handle_incoming serde_json start");
    let msg = serde_json::from_str::<Msg<ProtocolMessage>>(&msg_json).expect("deserialize message");
    debug!("gg20_keygen eygent.handle_incoming start");
    keygent.handle_incoming(msg).unwrap();
}

pub fn wants_to_proceed(keygen_ptr: *mut Keygen) -> bool {
    let keygent = unsafe {
        assert!(!keygen_ptr.is_null());
        &mut *keygen_ptr
    };
    keygent.wants_to_proceed()
}

pub fn proceed(keygen_ptr: *mut Keygen) {
    let keygent = unsafe {
        assert!(!keygen_ptr.is_null());
        &mut *keygen_ptr
    };
    keygent.proceed().unwrap();
}

pub fn message_queue(keygen_ptr: *mut Keygen) -> String {
    let keygent = unsafe {
        assert!(!keygen_ptr.is_null());
        &mut *keygen_ptr
    };
    let to_json = serde_json::to_string(&keygent.message_queue());
    keygent.message_queue().drain(..);
    to_json.unwrap()
}

pub fn is_finished(keygen_ptr: *mut Keygen) -> bool {
    let keygent = unsafe {
        assert!(!keygen_ptr.is_null());
        &mut *keygen_ptr
    };
    keygent.is_finished()
}

pub fn pick_output(keygen_ptr: *mut Keygen) -> String {
    let keygent = unsafe {
        assert!(!keygen_ptr.is_null());
        &mut *keygen_ptr
    };

    let result = keygent.pick_output().unwrap().expect("output");
    serde_json::to_string(&result).unwrap()
}

pub fn current_round(keygen_ptr: *mut Keygen) -> u16 {
    let keygent = unsafe {
        assert!(!keygen_ptr.is_null());
        &mut *keygen_ptr
    };
    keygent.current_round()
}
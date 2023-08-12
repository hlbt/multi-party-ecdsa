use crate::protocols::multi_party_ecdsa::gg_2020::state_machine::keygen::{Keygen, ProtocolMessage};
use round_based::Msg;
use round_based::StateMachine;

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
    let keygent = unsafe {
        assert!(!keygen_ptr.is_null());
        &mut *keygen_ptr
    };
    let msg = serde_json::from_str::<Msg<ProtocolMessage>>(&msg_json).expect("deserialize message");
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
    serde_json::to_string(&keygent.message_queue()).unwrap()
}

// use cggmp-threshold-ecdsa::refresh::state_machine::{self, state_machine::KeyRefresh, ProtocolMessage}
// use crate::protocols::multi_party_ecdsa::gg_2020::state_machine::keygen::LocalKey;
// use curv::elliptic::curves::secp256_k1::Secp256k1;


// pub fn create_refresh(json_key: String, new_party_index: u16, old_to_new_map: HashMap<u16, u16>, threshold: u16, number_of_parties: u16) -> *mut KeyRefresh {
//     let key = serde_json::from_str::<LocalKey<Secp256k1>>(&key).expect("deserialize message");
//     let keyRefresh = KeyRefresh::new(Some(key), Some(new_party_index), *old_to_new_map, threshold, number_of_parties).expect("expected keygen");
//     Box::into_raw(Box::new(keyRefresh))
// }

// pub fn free_keygen(keyRefresh_ptr: *mut KeyRefresh) {
//     if keyRefresh_ptr.is_null() {
//         return;
//     }
//     unsafe {
//         drop(Box::from_raw(keyRefresh_ptr));
//     }
// }

// pub fn handle_incoming(keyRefresh_ptr: *mut KeyRefresh, msg_json: String) {
//     let keyRefresh = unsafe {
//         assert!(!keyRefresh_ptr.is_null());
//         &mut *keyRefresh_ptr
//     };
//     let msg = serde_json::from_str::<Msg<ProtocolMessage>>(&msg_json).expect("deserialize message");
//     keyRefresh.handle_incoming(msg).unwrap();
// }

// pub fn wants_to_proceed(keyRefresh_ptr: *mut KeyRefresh) -> bool {
//     let keyRefresh = unsafe {
//         assert!(!keyRefresh_ptr.is_null());
//         &mut *keyRefresh_ptr
//     };
//     keyRefresh.wants_to_proceed()
// }

// pub fn proceed(keyRefresh_ptr: *mut KeyRefresh) {
//     let keyRefresh = unsafe {
//         assert!(!keyRefresh_ptr.is_null());
//         &mut *keyRefresh_ptr
//     };
//     keyRefresh.proceed().unwrap();
// }

// pub fn message_queue(keyRefresh_ptr: *mut KeyRefresh) -> String {
//     let keyRefresh = unsafe {
//         assert!(!keyRefresh_ptr.is_null());
//         &mut *keyRefresh_ptr
//     };
//     let to_json = serde_json::to_string(&keyRefresh.message_queue());
//     keyRefresh.message_queue().drain(..);
//     to_json.unwrap()
// }

// pub fn is_finished(keyRefresh_ptr: *mut KeyRefresh) -> bool {
//     let keyRefresh = unsafe {
//         assert!(!keyRefresh_ptr.is_null());
//         &mut *keyRefresh_ptr
//     };
//     keyRefresh.is_finished()
// }

// pub fn pick_output(keyRefresh_ptr: *mut KeyRefresh) -> String {
//     let keyRefresh = unsafe {
//         assert!(!keyRefresh_ptr.is_null());
//         &mut *keyRefresh_ptr
//     };
//     let result = keyRefresh.pick_output().unwrap().expect("output");
//     serde_json::to_string(&result).unwrap()
// }

// pub fn current_round(keyRefresh_ptr: *mut KeyRefresh) -> u16 {
//     let keyRefresh = unsafe {
//         assert!(!keyRefresh_ptr.is_null());
//         &mut *keyRefresh_ptr
//     };
//     keyRefresh.current_round()
// }
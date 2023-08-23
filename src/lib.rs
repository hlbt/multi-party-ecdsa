/*
    Multi-party ECDSA

    Copyright 2018 by Kzen Networks

    This file is part of Multi-party ECDSA library
    (https://github.com/KZen-networks/multi-party-ecdsa)

    Multi-party ECDSA is free software: you can redistribute
    it and/or modify it under the terms of the GNU General Public
    License as published by the Free Software Foundation, either
    version 3 of the License, or (at your option) any later version.

    @license GPL-3.0+ <https://github.com/KZen-networks/multi-party-ecdsa/blob/master/LICENSE>
*/

#![allow(clippy::many_single_char_names)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

pub mod protocols;
pub mod utilities;
pub mod gg20_keygen;
pub mod gg20_refresh;
pub mod gg20_signing;
pub mod gg20_android;
use std::fmt;

#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub enum Error {
    InvalidKey,
    InvalidSS,
    InvalidCom,
    InvalidSig,
    Phase5BadSum,
    Phase6Error,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match *self {
            InvalidKey => write!(f, "InvalidKey"),
            InvalidSS => write!(f, "InvalidSS"),
            InvalidCom => write!(f, "InvalidCom"),
            InvalidSig => write!(f, "InvalidSig"),
            Phase5BadSum => write!(f, "Phase5BadSum"),
            Phase6Error => write!(f, "Phase6Error"),
        }
    }
}


use std::os::raw::{c_char, c_int, c_long};
use std::ffi::{CString, CStr};
use std::slice;
use crate::protocols::multi_party_ecdsa::gg_2020::state_machine::keygen::Keygen;
use crate::protocols::multi_party_ecdsa::gg_2020::state_machine::refresh::KeyRefresh;
use crate::protocols::multi_party_ecdsa::gg_2020::state_machine::sign::OfflineStage;

// #[no_mangle]
// pub extern fn create_key(address: *const c_char, room: *const c_char, index: c_int, threshold: c_int, number_of_parties: c_int) -> *mut c_char {
//     let c_str_address = unsafe { CStr::from_ptr(address) };
//     let str_address: &str = c_str_address.to_str().unwrap();
//     let string_address: String = str_address.to_owned();
//     let url_address = Url::parse(&string_address).expect("invalid address string");

//     let c_str_room = unsafe { CStr::from_ptr(room) };
//     let str_room: &str = c_str_room.to_str().unwrap();
//     let string_room: String = str_room.to_owned();


//     let u16_index: u16 = index as u16;

//     let u16_threshold: u16 = threshold as u16;

//     let u16_number_of_parties: u16 = number_of_parties as u16;

//     let task = crate::gg20_android::create_key_async(url_address, string_room, u16_index, u16_threshold, u16_number_of_parties);
//     let result = tokio::runtime::Runtime::new().unwrap().block_on(task);


//     CString::new(result.unwrap()).unwrap().into_raw()
// }

// #[no_mangle]
// pub extern fn sign_data(address: *const c_char, room: *const c_char, parties: *const c_int, data_to_sign: *const c_char, local_share: *const c_char) -> *mut c_char {
//     let c_str_address = unsafe { CStr::from_ptr(address) };
//     let str_address: &str = c_str_address.to_str().unwrap();
//     let string_address: String = str_address.to_owned();
//     let url_address = Url::parse(&string_address).expect("invalid address string");

//     let c_str_room = unsafe { CStr::from_ptr(room) };
//     let str_room: &str = c_str_room.to_str().unwrap();
//     let string_room: String = str_room.to_owned();

//     let c_int_parties: &[c_int] = unsafe {
//         let len = 2;
//         slice::from_raw_parts(parties, len)
//     };
//     let u16_vec_parties: Vec<u16> = c_int_parties.iter().map(|&c| c as u16).collect();


//     let c_str_data_to_sign = unsafe { CStr::from_ptr(data_to_sign) };
//     let str_data_to_sign: &str = c_str_data_to_sign.to_str().unwrap();
//     let string_data_to_sign: String = str_data_to_sign.to_owned();

//     let c_str_local_share = unsafe { CStr::from_ptr(local_share) };
//     let str_local_share: &str = c_str_local_share.to_str().unwrap();
//     let string_local_share: String = str_local_share.to_owned();

//     let task = crate::gg20_android::sign_data_async(url_address, string_room, u16_vec_parties, string_data_to_sign, string_local_share);
//     let result = tokio::runtime::Runtime::new().unwrap().block_on(task);
//     CString::new(result.unwrap()).unwrap().into_raw()
// }


#[no_mangle]
pub extern fn create_keygen(index: c_int) -> c_long {
    let u16_index: u16 = index as u16;

    crate::gg20_keygen::create_keygen(u16_index, 1, 3) as c_long
}

#[no_mangle]
pub extern fn create_refresh(json_key: *const c_char, new_party_index: c_int) -> c_long {
    let new_party_index: u16 = new_party_index as u16;

    let c_str_key = unsafe { CStr::from_ptr(json_key) };
    let str_key = c_str_key.to_str().unwrap();
    let string_key = str_key.to_owned();
    let json_key = match string_key.len() {
        0 => None,
        _ => Some(string_key)
    };

    crate::gg20_refresh::create_refresh(json_key, new_party_index, 1, 3) as c_long
}

#[no_mangle]
pub extern fn create_signing(mg_index: c_int, parties: *const c_int, json_key: *const c_char, data_to_sign: *const c_char) -> c_long {
    let mg_index: u16 = mg_index as u16;

    let c_int_parties: &[c_int] = unsafe {
        let len = 2;
        slice::from_raw_parts(parties, len)
    };
    let u16_vec_parties: Vec<u16> = c_int_parties.iter().map(|&c| c as u16).collect();

    let c_str_local_share = unsafe { CStr::from_ptr(json_key) };
    let str_local_share: &str = c_str_local_share.to_str().unwrap();
    let string_local_share: String = str_local_share.to_owned();

    let c_str_data_to_sign = unsafe { CStr::from_ptr(data_to_sign) };
    let str_data_to_sign: &str = c_str_data_to_sign.to_str().unwrap();
    let string_data_to_sign: String = str_data_to_sign.to_owned();

    crate::gg20_signing::create_signing(mg_index, u16_vec_parties, string_local_share, string_data_to_sign) as c_long
}

#[no_mangle]
pub extern fn freeMpc(call_type: c_int, mpc_ptr: c_long) {
    let call_type: u16 = call_type as u16;

    match call_type {
        1 => crate::gg20_keygen::free_keygen(mpc_ptr as *mut Keygen),
        2 => crate::gg20_refresh::free_refresh(mpc_ptr as *mut KeyRefresh),
        3 => crate::gg20_signing::free_signing(mpc_ptr as *mut OfflineStage),
        _ => (),
    };
}

#[no_mangle]
pub unsafe extern fn keygenHandleIncoming(call_type: c_int, mpc_ptr: c_long, msg_json: *const c_char) {
    let c_str_msg_json = unsafe { CStr::from_ptr(msg_json) };
    let str_msg_json: &str = c_str_msg_json.to_str().unwrap();
    let string_msg_json: String = str_msg_json.to_owned();

    let call_type = call_type as u16;
    match call_type {
        1 => crate::gg20_keygen::handle_incoming(mpc_ptr as *mut Keygen, string_msg_json.to_string()),
        2 => crate::gg20_refresh::handle_incoming(mpc_ptr as *mut KeyRefresh, string_msg_json.to_string()),
        3 => crate::gg20_signing::handle_incoming(mpc_ptr as *mut OfflineStage, string_msg_json.to_string()),
        _ => (),
    };
}

#[no_mangle]
pub unsafe extern fn keygenWantsToProceed(call_type: c_int, mpc_ptr: c_long) -> c_int {
    let call_type = call_type as u16;
    let result = match call_type {
        1 => crate::gg20_keygen::wants_to_proceed(mpc_ptr as *mut Keygen),
        2 => crate::gg20_refresh::wants_to_proceed(mpc_ptr as *mut KeyRefresh),
        3 => crate::gg20_signing::wants_to_proceed(mpc_ptr as *mut OfflineStage),
        _ => false,
    };
    let reuslt = match result {
        true => 1,
        false => 0,
    };
    reuslt as c_int
}

#[no_mangle]
pub unsafe extern fn keygenProceed(call_type: c_int, mpc_ptr: c_long) {
    let call_type = call_type as u16;
    match call_type {
        1 => crate::gg20_keygen::proceed(mpc_ptr as *mut Keygen),
        2 => crate::gg20_refresh::proceed(mpc_ptr as *mut KeyRefresh),
        3 => crate::gg20_signing::proceed(mpc_ptr as *mut OfflineStage),
        _ => (),
    };
}

#[no_mangle]
pub unsafe extern fn keygenMessageQueue(call_type: c_int, mpc_ptr: c_long) -> *mut c_char {
    let call_type = call_type as u16;
    let str_json = match call_type {
        1 => crate::gg20_keygen::message_queue(mpc_ptr as *mut Keygen),
        2 => crate::gg20_refresh::message_queue(mpc_ptr as *mut KeyRefresh),
        3 => crate::gg20_signing::message_queue(mpc_ptr as *mut OfflineStage),
        _ => "".to_string(),
    };
    CString::new(str_json).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern fn keygenIsFinished(call_type: c_int, mpc_ptr: c_long) -> c_int {
    let call_type = call_type as u16;
    let result = match call_type {
        1 => crate::gg20_keygen::is_finished(mpc_ptr as *mut Keygen),
        2 => crate::gg20_refresh::is_finished(mpc_ptr as *mut KeyRefresh),
        3 => crate::gg20_signing::is_finished(mpc_ptr as *mut OfflineStage),
        _ => false,
    };
    let reuslt = match result {
        true => 1,
        false => 0,
    };
    reuslt as c_int
}

#[no_mangle]
pub unsafe extern fn keygenPickOutput(call_type: c_int, mpc_ptr: c_long) -> *mut c_char {
    let call_type = call_type as u16;
    let str_json = match call_type {
        1 => crate::gg20_keygen::pick_output(mpc_ptr as *mut Keygen),
        2 => crate::gg20_refresh::pick_output(mpc_ptr as *mut KeyRefresh),
        3 => crate::gg20_signing::pick_output(mpc_ptr as *mut OfflineStage),
        _ => "".to_string(),
    };
    CString::new(str_json).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern fn keygenCurrentRound(call_type: c_int, mpc_ptr: c_long) -> c_int {
    let call_type = call_type as u16;
    let result = match call_type {
        1 => crate::gg20_keygen::current_round(mpc_ptr as *mut Keygen),
        2 => crate::gg20_refresh::current_round(mpc_ptr as *mut KeyRefresh),
        3 => crate::gg20_signing::current_round(mpc_ptr as *mut OfflineStage),
        _ => 0,
    };
    result as c_int
}

// #[no_mangle]
// pub unsafe extern fn dataToSign(data_to_sign: *const c_char, offline_stage_json: *const c_char) -> *mut c_char {
//     let c_str_data_to_sign = unsafe { CStr::from_ptr(data_to_sign) };
//     let str_data_to_sign: &str = c_str_data_to_sign.to_str().unwrap();
//     let string_data_to_sign: String = str_data_to_sign.to_owned();

//     let c_str_offline_stage_json = unsafe { CStr::from_ptr(offline_stage_json) };
//     let str_offline_stage_json: &str = c_str_offline_stage_json.to_str().unwrap();
//     let string_offline_stage_json: String = str_offline_stage_json.to_owned();

//     let str_json = crate::gg20_signing::data_to_sign(string_data_to_sign, string_offline_stage_json);

//     CString::new(str_json).unwrap().into_raw()
// }

// #[no_mangle]
// pub unsafe extern fn completeSignature( data_to_sign: *const c_char, offline_stage_json: *const c_char, partial_signatures_json: *const c_char) -> *mut c_char {
//     let c_str_data_to_sign = unsafe { CStr::from_ptr(data_to_sign) };
//     let str_data_to_sign: &str = c_str_data_to_sign.to_str().unwrap();
//     let string_data_to_sign: String = str_data_to_sign.to_owned();

//     let c_str_offline_stage_json = unsafe { CStr::from_ptr(offline_stage_json) };
//     let str_offline_stage_json: &str = c_str_offline_stage_json.to_str().unwrap();
//     let string_offline_stage_json: String = str_offline_stage_json.to_owned();

//     let c_str_partial_signatures_json = unsafe { CStr::from_ptr(partial_signatures_json) };
//     let str_partial_signatures_json: &str = c_str_partial_signatures_json.to_str().unwrap();
//     let string_partial_signatures_json: String = str_partial_signatures_json.to_owned();

//     let str_json = crate::gg20_signing::complete_signature(string_data_to_sign, string_offline_stage_json, string_partial_signatures_json);

//     CString::new(str_json).unwrap().into_raw()
// }

#[no_mangle]
pub extern fn rust_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}
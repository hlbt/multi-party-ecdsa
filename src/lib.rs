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


use std::os::raw::{c_char, c_int};
use std::ffi::{CString, CStr};
use std::slice;
// use surf::Url;

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
pub extern fn rust_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}
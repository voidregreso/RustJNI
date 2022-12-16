#![allow(non_snake_case)]

extern crate core;

use jni::objects::JObject;
use jni::sys::{jboolean, jbyteArray, JNI_TRUE};
use jni::JNIEnv;

fn rc4(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut state = [0u8; 256];
    let mut x = 0;
    let mut y = 0;
    let mut key_len = key.len();

    for i in 0..256 {
        state[i] = i as u8;
    }

    for i in 0..256 {
        x = (x + state[i] as usize + key[i % key_len] as usize) % 256;
        state.swap(i, x);
    }

    let mut out = Vec::with_capacity(data.len());

    for d in data {
        x = (x + 1) % 256;
        y = (y + state[x] as usize) % 256;
        state.swap(x, y);
        let s_x = state[x];
        let s_y = state[y];
        out.push(d ^ s_x ^ s_y);
    }
    out
}

#[no_mangle]
pub extern "system" fn Java_com_ernesto_RustFeature_cryptRC4 (
    env: JNIEnv,
    _obj: JObject,
    str: jbyteArray,
    key: jbyteArray,
    reverse: jboolean
) -> jbyteArray {
    let b1: Vec<u8> = env.convert_byte_array(str).unwrap();
    let b2: Vec<u8> = env.convert_byte_array(key).unwrap();
    let mut res: Vec<u8> = rc4(b2.as_slice().try_into().unwrap(), b1.as_slice().try_into().unwrap());
    if reverse == JNI_TRUE {
        res.reverse();
    }
    let new_array = env.byte_array_from_slice(&res).unwrap();
    new_array.into()
}

use core::convert::TryFrom;
use das_types::convert;
use das_types::schemas::basic::*;
use hex;
use molecule::prelude::Entity;

#[test]
fn should_support_u8() {
    let num: u8 = u8::MAX;
    let data = Uint8::from(num);
    let reader = data.as_reader();
    // println!("{:?}", data);

    assert_eq!(num, u8::from(reader));
    assert_eq!(num, u8::from(data));
}

#[test]
fn should_support_u16() {
    let num: u16 = u16::MAX;
    let data = Uint16::from(num);
    let reader = data.as_reader();
    // println!("{:?}", data);

    assert_eq!(num, u16::from(reader));
    assert_eq!(num, u16::from(data));
}

#[test]
fn should_support_u32() {
    let num: u32 = u32::MAX;
    let data = Uint32::from(num);
    let reader = data.as_reader();
    // println!("{:?}", data);

    assert_eq!(num, u32::from(reader));
    assert_eq!(num, u32::from(data));
}

#[test]
fn should_support_u64() {
    let num: u64 = u64::MAX;
    let data = Uint64::from(num);
    let reader = data.as_reader();
    // println!("{:?}", data);

    assert_eq!(num, u64::from(reader));
    assert_eq!(num, u64::from(data));
}

#[test]
fn should_support_u128() {
    let num: u128 = u128::MAX;
    let data = Uint128::from(num);
    let reader = data.as_reader();
    // println!("{:?}", data);

    assert_eq!(num, u128::from(reader));
    assert_eq!(num, u128::from(data));
}

#[test]
fn should_support_bytes() {
    // Convert from Bytes between Vec<u8>
    let text_in_vec = Vec::from(text);
    let data = Bytes::from(text_in_vec.clone());

    assert_eq!(Vec::from(data), text_in_vec);

    // Convert from Bytes to String
    let text = "hello world";
    let data = Bytes::from(text.as_bytes().to_vec());

    assert_eq!(String::try_from(data), Ok(String::from(text)));
}

#[test]
fn should_support_hash() {
    // Convert from Hash between Vec
    let text = "a0ec1714a64139b5f0e46d1d1de4f2e7b32735ffedaab34285c49f2e5269abda";
    let mut buf = [0u8; 32];
    hex::decode_to_slice(text, &mut buf);
    let data = Hash::try_from(buf.to_vec()).unwrap();

    assert_eq!(Vec::from(data), buf.to_vec());
}

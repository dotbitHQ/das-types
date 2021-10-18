use super::schemas::packed::*;
use alloc::string::FromUtf8Error;
use ckb_std::ckb_types::{bytes, packed as no_std_packed};
use core::convert::TryFrom;
use molecule::{error::VerificationError, prelude::*};
use std::prelude::v1::*;

#[cfg(feature = "std")]
use ckb_types::packed as std_packed;

/// Implement convert between primitive type and molecule types
macro_rules! impl_uint_convert {
    ($uint_type:ty, $mol_type:ty, $mol_reader_typer:ident, $length: expr) => {
        impl From<$uint_type> for $mol_type {
            fn from(v: $uint_type) -> Self {
                Self::new_unchecked(bytes::Bytes::from(v.to_le_bytes().to_vec()))
            }
        }

        impl From<$mol_type> for $uint_type {
            fn from(v: $mol_type) -> Self {
                let mut buf = [0u8; $length];
                buf.copy_from_slice(v.raw_data().as_ref());
                <$uint_type>::from_le_bytes(buf)
            }
        }

        impl From<$mol_reader_typer<'_>> for $uint_type {
            fn from(v: $mol_reader_typer<'_>) -> Self {
                let mut buf = [0u8; $length];
                buf.copy_from_slice(v.raw_data());
                <$uint_type>::from_le_bytes(buf)
            }
        }
    };
}

impl_uint_convert!(u8, Uint8, Uint8Reader, 1);
impl_uint_convert!(u32, Uint32, Uint32Reader, 4);
impl_uint_convert!(u64, Uint64, Uint64Reader, 8);

impl_uint_convert!(u64, Timestamp, TimestampReader, 8);

/// Convert &[u8] to schemas::basic::Bytes
///
/// The difference with from_slice is that it does not require a dynvec header.
impl From<&[u8]> for Bytes {
    fn from(v: &[u8]) -> Self {
        Bytes::new_builder()
            .set(v.to_owned().into_iter().map(Byte::new).collect())
            .build()
    }
}

/// Convert Vec<u8> to schemas::basic::Bytes
///
/// The difference with from_slice is that it does not require a dynvec header.
impl From<Vec<u8>> for Bytes {
    fn from(v: Vec<u8>) -> Self {
        Bytes::from(v.as_slice())
    }
}

/// Convert bytes::Bytes to schemas::basic::Bytes
impl From<bytes::Bytes> for Bytes {
    fn from(v: bytes::Bytes) -> Self {
        Bytes::from(v.as_ref())
    }
}

impl From<no_std_packed::Bytes> for Bytes {
    fn from(v: no_std_packed::Bytes) -> Self {
        Bytes::new_unchecked(v.as_bytes().into())
    }
}

impl Into<no_std_packed::Bytes> for Bytes {
    fn into(self) -> no_std_packed::Bytes {
        no_std_packed::Bytes::new_unchecked(self.as_bytes().into())
    }
}

/// Convert schemas::basic::Bytes to Vec<u8>
///
/// The main thing here is to remove the Header from the Molecule data.
impl From<Bytes> for Vec<u8> {
    fn from(v: Bytes) -> Self {
        v.as_reader().raw_data().to_vec()
    }
}

/// Convert schemas::basic::Bytes to String
///
/// The main thing here is to remove the Header from the Molecule data.
impl TryFrom<Bytes> for String {
    type Error = FromUtf8Error;
    fn try_from(v: Bytes) -> Result<Self, FromUtf8Error> {
        let bytes = v.as_reader().raw_data().to_vec();
        String::from_utf8(bytes).map(|v| String::from(v))
    }
}

impl From<Script> for ScriptOpt {
    fn from(v: Script) -> Self {
        ScriptOpt::new_builder().set(Some(v)).build()
    }
}

/// Convert &[u8] to schemas::basic::Hash
///
/// The difference with from_slice is that it does not require a dynvec header.
impl TryFrom<&[u8]> for Hash {
    type Error = VerificationError;
    fn try_from(v: &[u8]) -> Result<Self, VerificationError> {
        if v.len() != 32 {
            return Err(VerificationError::TotalSizeNotMatch("Hash".to_owned(), 32, v.len()));
        }
        let mut inner = [Byte::new(0); 32];
        let v = v.to_owned().into_iter().map(Byte::new).collect::<Vec<_>>();
        inner.copy_from_slice(&v);
        Ok(Self::new_builder().set(inner).build())
    }
}

impl TryFrom<Vec<u8>> for Hash {
    type Error = VerificationError;
    fn try_from(v: Vec<u8>) -> Result<Self, VerificationError> {
        Hash::try_from(v.as_slice())
    }
}

impl From<[u8; 32]> for Hash {
    fn from(v: [u8; 32]) -> Self {
        let mut inner = [Byte::new(0); 32];
        for (i, item) in v.iter().enumerate() {
            inner[i] = Byte::new(*item);
        }
        Self::new_builder().set(inner).build()
    }
}

/// Convert between schemas::Hash and ckb_std::ckb_types::packed::Byte32
impl From<no_std_packed::Byte32> for Hash {
    fn from(v: no_std_packed::Byte32) -> Self {
        Hash::new_unchecked(v.as_bytes().into())
    }
}

impl<'r> From<no_std_packed::Byte32Reader<'r>> for HashReader<'r> {
    fn from(v: no_std_packed::Byte32Reader<'r>) -> Self {
        HashReader::new_unchecked(v.as_slice())
    }
}

impl Into<no_std_packed::Byte32> for Hash {
    fn into(self) -> no_std_packed::Byte32 {
        no_std_packed::Byte32::new_unchecked(self.as_bytes().into())
    }
}

impl<'r> Into<no_std_packed::Byte32Reader<'r>> for HashReader<'r> {
    fn into(self) -> no_std_packed::Byte32Reader<'r> {
        no_std_packed::Byte32Reader::new_unchecked(self.as_slice())
    }
}

#[cfg(feature = "std")]
impl From<std_packed::Byte32> for Hash {
    fn from(v: std_packed::Byte32) -> Self {
        Hash::new_unchecked(v.as_bytes().into())
    }
}

#[cfg(feature = "std")]
impl<'r> From<std_packed::Byte32Reader<'r>> for HashReader<'r> {
    fn from(v: std_packed::Byte32Reader<'r>) -> Self {
        HashReader::new_unchecked(v.as_slice())
    }
}

#[cfg(feature = "std")]
impl Into<std_packed::Byte32> for Hash {
    fn into(self) -> std_packed::Byte32 {
        std_packed::Byte32::new_unchecked(self.as_bytes().into())
    }
}

#[cfg(feature = "std")]
impl<'r> Into<std_packed::Byte32Reader<'r>> for HashReader<'r> {
    fn into(self) -> std_packed::Byte32Reader<'r> {
        std_packed::Byte32Reader::new_unchecked(self.as_slice())
    }
}

/// Convert schemas::basic::Hash to Vec<u8>
impl From<Hash> for Vec<u8> {
    fn from(v: Hash) -> Self {
        v.as_slice().to_vec()
    }
}

/// Convert between schemas::Script and ckb_std::ckb_types::packed::Script
impl From<no_std_packed::Script> for Script {
    fn from(v: no_std_packed::Script) -> Self {
        Script::new_unchecked(v.as_bytes().into())
    }
}

impl<'r> From<no_std_packed::ScriptReader<'r>> for ScriptReader<'r> {
    fn from(v: no_std_packed::ScriptReader<'r>) -> Self {
        ScriptReader::new_unchecked(v.as_slice())
    }
}

impl Into<no_std_packed::Script> for Script {
    fn into(self) -> no_std_packed::Script {
        no_std_packed::Script::new_unchecked(self.as_bytes().into())
    }
}

impl<'r> Into<no_std_packed::ScriptReader<'r>> for ScriptReader<'r> {
    fn into(self) -> no_std_packed::ScriptReader<'r> {
        no_std_packed::ScriptReader::new_unchecked(self.as_slice())
    }
}

#[cfg(feature = "std")]
impl From<std_packed::Script> for Script {
    fn from(v: std_packed::Script) -> Self {
        Script::new_unchecked(v.as_bytes().into())
    }
}

#[cfg(feature = "std")]
impl<'r> From<std_packed::ScriptReader<'r>> for ScriptReader<'r> {
    fn from(v: std_packed::ScriptReader<'r>) -> Self {
        ScriptReader::new_unchecked(v.as_slice())
    }
}

#[cfg(feature = "std")]
impl Into<std_packed::Script> for Script {
    fn into(self) -> std_packed::Script {
        std_packed::Script::new_unchecked(self.as_bytes().into())
    }
}

#[cfg(feature = "std")]
impl<'r> Into<std_packed::ScriptReader<'r>> for ScriptReader<'r> {
    fn into(self) -> std_packed::ScriptReader<'r> {
        std_packed::ScriptReader::new_unchecked(self.as_slice())
    }
}

/// Convert &[u8] to schemas::basic::Hash
///
/// The difference with from_slice is that it does not require a dynvec header.
impl TryFrom<&[u8]> for AccountId {
    type Error = VerificationError;
    fn try_from(v: &[u8]) -> Result<Self, VerificationError> {
        if v.len() != 20 {
            return Err(VerificationError::TotalSizeNotMatch(
                "AccountId".to_owned(),
                10,
                v.len(),
            ));
        }
        let mut inner = [Byte::new(0); 20];
        let v = v.to_owned().into_iter().map(Byte::new).collect::<Vec<_>>();
        inner.copy_from_slice(&v);
        Ok(Self::new_builder().set(inner).build())
    }
}

impl TryFrom<Vec<u8>> for AccountId {
    type Error = VerificationError;
    fn try_from(v: Vec<u8>) -> Result<Self, VerificationError> {
        AccountId::try_from(v.as_slice())
    }
}

/// Convert schemas::basic::AccountId to Vec<u8>
impl From<AccountId> for Vec<u8> {
    fn from(v: AccountId) -> Self {
        v.as_slice().to_vec()
    }
}

/// Convert AccountChars to human-readable string
impl AccountChars {
    pub fn as_readable(&self) -> Vec<u8> {
        let mut ret = Vec::new();
        for reader in self.as_reader().iter() {
            ret.append(&mut reader.bytes().raw_data().to_owned());
        }

        ret
    }
}

/// Convert AccountCharsReader to human-readable string
impl<'r> AccountCharsReader<'r> {
    pub fn as_readable(&self) -> Vec<u8> {
        let mut ret = Vec::new();
        for reader in self.iter() {
            ret.append(&mut reader.bytes().raw_data().to_owned());
        }

        ret
    }
}

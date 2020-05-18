use dislog_hal::Bytes;
use kv_object::kv_object::{KVWrapperT, KvWrapper};
use kv_object::prelude::AttrProxy;
use kv_object::sm2::CertificateSm2;
use kv_object::KVObjectError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quota {
    id: [u8; 32],
    timep: i64,
    face_value: u64,
    issue_cert: CertificateSm2,
    trade_hash: [u8; 32],
}

impl Quota {
    pub fn new(
        id: [u8; 32],
        timep: i64,
        face_value: u64,
        issue_cert: CertificateSm2,
        trade_hash: [u8; 32],
    ) -> Self {
        Self {
            id,
            timep,
            face_value,
            issue_cert,
            trade_hash,
        }
    }
}

pub const QUOTA_LEN: usize = 113;

impl Bytes for Quota {
    type BytesType = Vec<u8>;

    type Error = KVObjectError;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() % QUOTA_LEN != 0 {
            return Err(KVObjectError::DeSerializeError);
        }
        let mut id_ = [0u8; 32];
        let mut timep_ = [0u8; 8];
        let mut face_value_ = [0u8; 8];
        let mut trade_hash_ = [0u8; 32];

        id_.clone_from_slice(&bytes[..32]);
        timep_.clone_from_slice(&bytes[32..40]);
        face_value_.clone_from_slice(&bytes[40..48]);
        trade_hash_.clone_from_slice(&bytes[81..QUOTA_LEN]);

        let issue_cert = CertificateSm2::from_bytes(&bytes[48..81])
            .map_err(|_| KVObjectError::DeSerializeError)?;

        Ok(Self {
            id: id_,
            timep: i64::from_le_bytes(face_value_),
            face_value: u64::from_le_bytes(face_value_),
            issue_cert,
            trade_hash: trade_hash_,
        })
    }

    fn to_bytes(&self) -> Self::BytesType {
        let mut ret = Vec::<u8>::new();

        ret.extend_from_slice(&self.id[..]);
        ret.extend_from_slice(&self.timep.to_le_bytes()[..]);
        ret.extend_from_slice(&self.face_value.to_le_bytes()[..]);
        ret.extend_from_slice(self.issue_cert.to_bytes().as_ref());
        ret.extend_from_slice(&self.trade_hash[..]);

        ret
    }
}

impl AttrProxy for Quota {
    type Byte = Vec<u8>;

    // 根据key读取值
    fn get_key(&self, key: &str) -> Result<Self::Byte, KVObjectError> {
        let mut ret = Vec::<u8>::new();

        let timep_ = self.timep.to_le_bytes();
        let face_value_ = self.face_value.to_le_bytes();
        let issue_cert_ = self.issue_cert.to_bytes();
        ret.extend_from_slice(match key {
            "id" => &self.id[..],
            "timep" => &timep_[..],
            "face_value" => &face_value_[..],
            "issue_cert" => issue_cert_.as_ref(),
            "trade_hash" => &self.trade_hash[..],
            _ => return Err(KVObjectError::KeyIndexError),
        });
        Ok(ret)
    }

    // 根据key写值
    fn set_key(&mut self, _key: &str, _value: &Self::Byte) -> Result<(), KVObjectError> {
        Err(KVObjectError::KeyIndexError)
    }
}

impl KVWrapperT for Quota {}

pub type QuotaWrapper = KvWrapper<Quota>;

use super::quota::{Quota, QUOTA_LEN};
use dislog_hal::Bytes;
use kv_object::kv_object::{KVBody, KVObject};
use kv_object::prelude::AttrProxy;
use kv_object::sm2::CertificateSm2;
use kv_object::KVObjectError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    /// 额度控制位
    quota_info: Quota,
    /// 钱包公钥
    wallet_cert: CertificateSm2,
}

impl Currency {
    pub fn new(quota: Quota, cert: CertificateSm2) -> Self {
        Self {
            quota_info: quota,
            wallet_cert: cert,
        }
    }
}

pub const CURRENCY_LEN: usize = QUOTA_LEN + 33;

impl Bytes for Currency {
    type BytesType = Vec<u8>;

    type Error = KVObjectError;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != CURRENCY_LEN {
            return Err(KVObjectError::DeSerializeError);
        }

        let quota_info =
            Quota::from_bytes(&bytes[..QUOTA_LEN]).map_err(|_| KVObjectError::DeSerializeError)?;
        let wallet_cert = CertificateSm2::from_bytes(&bytes[QUOTA_LEN..CURRENCY_LEN])
            .map_err(|_| KVObjectError::DeSerializeError)?;

        Ok(Self {
            quota_info,
            wallet_cert,
        })
    }

    fn to_bytes(&self) -> Self::BytesType {
        let mut ret = Vec::<u8>::new();

        ret.extend_from_slice(self.quota_info.to_bytes().as_ref());
        ret.extend_from_slice(self.wallet_cert.to_bytes().as_ref());

        ret
    }
}

impl AttrProxy for Currency {
    type Byte = Vec<u8>;

    // 根据key读取值
    fn get_key(&self, key: &str) -> Result<Self::Byte, KVObjectError> {
        let mut ret = Vec::<u8>::new();

        let quota_info_ = self.quota_info.to_bytes();
        let wallet_cert_ = self.wallet_cert.to_bytes();
        ret.extend_from_slice(match key {
            "quota_info" => quota_info_.as_ref(),
            "wallet_cert" => wallet_cert_.as_ref(),
            _ => return Err(KVObjectError::KeyIndexError),
        });
        Ok(ret)
    }

    // 根据key写值
    fn set_key(&mut self, _key: &str, _value: &Self::Byte) -> Result<(), KVObjectError> {
        Err(KVObjectError::KeyIndexError)
    }
}

impl KVBody for Currency {}

pub type CurrencyWrapper = KVObject<Currency>;

#[cfg(test)]
mod tests {

    #[test]
    fn test_issue_quota() {
        use super::super::issue::Issue;
        use super::super::quota::QuotaWrapper;
        use super::{Currency, CurrencyWrapper};
        use asymmetric_crypto::prelude::Keypair;
        use kv_object::kv_object::MsgType;
        use kv_object::prelude::KValueObject;
        use kv_object::sm2::KeyPairSm2;
        use rand::thread_rng;

        // 发行机构
        let mut rng = thread_rng();
        let keypair_sm2: KeyPairSm2 = KeyPairSm2::generate(&mut rng).unwrap();
        let cert = keypair_sm2.get_certificate();

        // 钱包
        let wallet_keypair_sm2: KeyPairSm2 = KeyPairSm2::generate(&mut rng).unwrap();
        let wallet_cert = keypair_sm2.get_certificate();

        let mut currencys = Vec::<(u64, u64)>::new();
        currencys.push((100, 1));
        currencys.push((50, 2));
        currencys.push((10, 5));

        let issue = Issue::new(currencys);
        let quotas = issue.quota_distribution(&cert);

        println!("{:?}", quotas);

        for each_quota in quotas.iter() {
            let mut quota = QuotaWrapper::new(MsgType::Quota, each_quota.clone());

            let sign_bytes = quota.to_bytes(&keypair_sm2).unwrap();

            println!("sigture: {:?}", sign_bytes);

            let serialized = serde_json::to_string(&quota).unwrap();
            println!("serialized = {}", serialized);

            let deserialized: QuotaWrapper = serde_json::from_str(&serialized).unwrap();
            println!("deserialized = {:?}", deserialized);

            //currency
            let currency = CurrencyWrapper::new(
                MsgType::Currency,
                Currency::new(quota.get_body().clone(), wallet_cert.clone()),
            );

            let _sign_bytes = quota.to_bytes(&wallet_keypair_sm2).unwrap();

            println!("currency: {:?}", currency);

            let serialized = serde_json::to_string(&currency).unwrap();
            println!("serialized currency = {}", serialized);

            let deserialized: CurrencyWrapper = serde_json::from_str(&serialized).unwrap();
            println!("deserialized currency = {:?}", deserialized);

            let deserialized_obj: Currency = deserialized.get_body().clone();
            println!("deserialized_obj = {:?}", deserialized_obj);
        }
    }
}

use ecies::{decrypt, encrypt};
use super::result::Result;

pub trait Crypto {
    /// 数据加密
    fn encrypt(receiver_pub_key: &[u8], msg: &[u8]) -> Result<Vec<u8>> {
        Ok(encrypt(receiver_pub_key, msg)?)
    }

    /// 数据解密
    fn decrypt(receiver_secret_key: &[u8], msg: &[u8]) -> Result<Vec<u8>> {
        Ok(decrypt(receiver_secret_key, msg)?)
    }
}
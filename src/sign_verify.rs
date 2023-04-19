use libsecp256k1::{Message, PublicKey, RecoveryId, SecretKey, sign, Signature, verify};
use super::result::Result;

pub trait SignVerify {
    /// 数据签名
    fn sign(secret_key: &[u8], msg: &[u8]) -> Result<(Signature, RecoveryId)> {
        let private_key = SecretKey::parse_slice(secret_key)?;
        Ok(sign(&Message::parse_slice(msg)?, &private_key))
    }

    /// 验证签名
    fn verify(pub_key: &[u8], sign: &Signature, msg: &[u8]) -> Result<bool> {
        let pub_key = PublicKey::parse_slice(pub_key, None)?;
        Ok(verify(&Message::parse_slice(msg)?, sign, &pub_key))
    }
}
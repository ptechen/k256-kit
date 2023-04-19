pub mod sign_verify;
pub mod crypto;
pub mod result;

pub struct Crypto;
impl crypto::Crypto for Crypto{}

pub struct SignVerify;
impl sign_verify::SignVerify for SignVerify{}
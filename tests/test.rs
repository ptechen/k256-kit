use encrypt_decrypt::EncryptDecrypt;
use k256_kit::crypto::Crypto;

pub struct Data;

impl Crypto for Data {}

#[test]
fn test() {
    let seckey = [
        59, 148, 11, 85, 134, 130, 61, 253, 2, 174, 59, 70, 27, 180, 51, 107, 94, 203, 174, 253,
        102, 39, 170, 146, 46, 252, 4, 143, 236, 12, 136, 28,
    ];
    let pubkey = [
        2, 29, 21, 35, 7, 198, 183, 43, 14, 208, 65, 139, 14, 112, 205, 128, 231, 245, 41, 91, 141,
        134, 245, 114, 45, 63, 82, 19, 251, 210, 57, 79, 54,
    ];
    let data = "123";
    let se = Data::encrypt(pubkey.as_slice(), data.as_bytes()).unwrap();
    let d = Data::decrypt(seckey.as_slice(), se.as_slice()).unwrap();
    assert_eq!(data, String::from_utf8(d).unwrap());
}
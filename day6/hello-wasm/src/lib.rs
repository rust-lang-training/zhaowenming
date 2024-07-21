// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
use rsa::{pkcs1::DecodeRsaPrivateKey,pkcs8::DecodePublicKey,Pkcs1v15Encrypt};
use wasm_bindgen::prelude::*;
use base64::prelude::BASE64_STANDARD;
// use rsa::pkcs8::DecodePublicKey;
use base64::Engine;
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
#[test]
#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should hava a body");
    let val = document.create_element("p")?;
    val.set_text_content(Some("Hello from Rust"));
    body.append_child(&val)?;
    Ok(())
}

#[wasm_bindgen]
pub fn encrypt_message(msg: &str) -> String {
let public_key_pem = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAsBcvGrHg6xzS3/j1xc0e
TjTCltJ7XMC+qeg7plBx+6/Pb8ni8Xvo31r7sMfEtyIMSEWxPVhWFOUsUYYWzQb1
miaTCPnjwA8j/UQ+3BTjR8ypekOVF9hesrJep2e3iTyfMNwT3BdrA9448acnarou
ckEggu482VWOVCASKcELwASF4FFOpKr6rUBeHfCkQ/sWygvrTBopvNLw4McKg/fD
RXGIFeVPv7BDmH1gntBR0RVljfO9g0RkWkmrWyzqbQ/kez/yM5yNIjMQ53NlxQY1
05kKhzm6gVy0vo/9VSGvAJ3v4U8m7OextNq14XaPxw/GP2JhoQ3KVUUzdASj+OmC
zQIDAQAB
-----END PUBLIC KEY-----";
let public_key = rsa::RsaPublicKey::from_public_key_pem(public_key_pem).unwrap();
let mut rng = rand::thread_rng();

let enc_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, msg.as_bytes()).unwrap();
 BASE64_STANDARD.encode(enc_data)
}


#[wasm_bindgen]
pub fn decrypt_message(msg: &str) -> String {
let private_key_pem = "-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEAsBcvGrHg6xzS3/j1xc0eTjTCltJ7XMC+qeg7plBx+6/Pb8ni
8Xvo31r7sMfEtyIMSEWxPVhWFOUsUYYWzQb1miaTCPnjwA8j/UQ+3BTjR8ypekOV
F9hesrJep2e3iTyfMNwT3BdrA9448acnarouckEggu482VWOVCASKcELwASF4FFO
pKr6rUBeHfCkQ/sWygvrTBopvNLw4McKg/fDRXGIFeVPv7BDmH1gntBR0RVljfO9
g0RkWkmrWyzqbQ/kez/yM5yNIjMQ53NlxQY105kKhzm6gVy0vo/9VSGvAJ3v4U8m
7OextNq14XaPxw/GP2JhoQ3KVUUzdASj+OmCzQIDAQABAoIBADHyUYUT4UUvBs1K
k4PIe4kZQ/x9tUoIzcvpuEmjVcFbC86ciAeQ6uu3pd5tu2lqeetiATTyQnBo1JpH
G16KU82mlVwdbTFD2iRzYBUSw6jQ711jn/2EfM/Y8iovrAp8LHcyZVuvNMaDGMnK
bM7EAnqb6Hna012d++H2utAnFosfBNsdudAkAd0teDnK3nhfkRyz8smAxdca5sNU
AseAnu7ygbx76Hf4rangNgvINVeyMHaL3vNWIIdK0/uTuTqZiagc9RQMALOy7MoZ
9tF1DY+irRM3WusRchkgfiUJy4BVr1iLcOtHWLZuaQCOJqw3vYg2DdcET0to9vsW
VkqkZkECgYEA1dBAtOCpAwiIH91kLhqS/ZV1osccq6D7u4YBDUxuzpXAS5jyLjii
PogILOSHeplhJyvVnGGJa5gHbBfQeh9katYfBIu4vLhvVD1+r2FlMilhGu93AQtK
gA3aMifCxdQPBheaUdFXu74RT2xcKaKLI2mCncXBubAGUnhGs9cB990CgYEA0tWK
lx+T0nisb3RUGsiwMyW6Bw6TDBv32VDf7u3BTZoCgbtTfcyAunS6/15CYU59HSRu
HyjZZR8kqXbJyamqz4QDZkDnpWUS2RaMWjzZlnXloIufOC53tsFIcYtgXPo/grtj
CV/rvBlPdexj9rKpqFS6173oELRV8Nx9Zi7I/7ECgYEAr5mZivP32sXnBkSNQlAQ
6LMXKdjdcxeDpz8nHgTn/EKA65VxUQSyOyj9jeEiY04UvFcD2KtCcVsqvVvRnHqr
vhKc5t7ZmiTfShA3O1KCCfByD+0bj3/2b7TwvsBo9pByxk4cL5X+t/IhxVj5WDhm
jW7aFzmTIS2wA4tEhe7PbwECgYEAij7FgnSNPFwuTWLM50ci6lbZlVkEKNOKADBc
tdGmh9jtvd10UK+w1RJHr71B74ZE2cASiVICwqDaLFic1iPvweSBSiJTAou1AS4c
1+rAjj0+VbhXrcIic25nrzRB+pWI4ZM3zGTeMVezV9KqdZAPWTY90ctPiOyG0cny
W/dfivECgYAOiX5GkDu5x5kf2ODJ+Pw8eklDmLZq3feZU0v1OAFWxPc45WJRxKRb
AfvxcErodnn8KdFAf0X1BFwv8RDs04it6KXZ2MnFdO1r6zjwBXzSrmkeeFUDzCrv
ngpI4oA31+8L3Guyo/Xr/vVrL+71dAHmPXlQRcfWwJF37/tu0SrZXw==
-----END RSA PRIVATE KEY-----";
let private_key = rsa::RsaPrivateKey::from_pkcs1_pem(private_key_pem).unwrap();
let data = BASE64_STANDARD.decode(msg).unwrap();
let mut rng = rand::thread_rng();

let plain_data = private_key.decrypt(Pkcs1v15Encrypt, &data[..]).unwrap();
String::from_utf8(plain_data).unwrap()
}
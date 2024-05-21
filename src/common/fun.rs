use crypto::md5::Md5;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use gmsm::sm4::{sm4_cbc_decrypt_hex,  sm4_cbc_encrypt_hex};
use rand::Rng;

pub fn sha1_salt(text: String,salt: String) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(&format!("RUST{}PANEL{}CATTLE", text, salt));
    hasher.result_str()
}
pub fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let charset: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let random_string: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
    random_string
}
pub fn md5(text: String) -> String {
    let mut hasher = Md5::new();
    hasher.input_str(&text);
    hasher.result_str()
}
pub fn sm4_decrypt_login(encrypted_text: String, key: String,t:String) -> String {
    // 使用 MD5 哈希函数生成初始化向量 (IV)
    let iv = md5(key.clone());

    // 使用 MD5 哈希函数和附加的 token 生成加密密钥
    let key_with_token = format!("{}{}{}", key, t, iv);
    let encryption_key = md5(key_with_token);

    // 执行解密操作
   sm4_cbc_decrypt_hex(&encrypted_text, &encryption_key, &iv)
}

pub fn sm4_decrypt_file(encrypted_text: String) -> String {
    let binding = md5(String::from("01b394cebf636ef53fe44c46a10abea2f54f60e6"));
    let secret_key = binding.as_str();
    sm4_cbc_decrypt_hex(&encrypted_text, secret_key, secret_key)
}

pub fn sm4_encrypt_file(plain:&str)-> String{
    let binding = md5(String::from("01b394cebf636ef53fe44c46a10abea2f54f60e6"));
    let secret_key = binding.as_str();

    sm4_cbc_encrypt_hex(plain, secret_key, secret_key)
}
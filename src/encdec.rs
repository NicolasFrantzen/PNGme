
use anyhow::{Result, bail};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};


pub fn encrypt_message(key: &str, message: &str) -> String
{
    let mc = new_magic_crypt!(key, 256);
    let base64 = mc.encrypt_str_to_base64(message);

    base64
}


pub fn decrypt_message(key: &str, base64: &str) -> Result<String>
{
    let mc = new_magic_crypt!(key, 256);
    match mc.decrypt_base64_to_string(&base64)
    {
        Ok(message) => Ok(message),
        Err(_) => bail!("Failed to decrypt message. Maybe the key was wrong?")
    }
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_encrypt()
    {
        let key = "key";
        let message = "A very very secret message...";

        let cipher = encrypt_message(key, message);
        assert_eq!(cipher, "A8lJif+V0WdM9nznA0+2r8ThEOQRg9BQuHBeLyRedh0=");
    }

    #[test]
    fn test_decrypt()
    {
        let key = "key";
        let cipher = "LG/4n1/8mnbvADcIpL0Xg1ApMsWcnAe5RPyRw+gmtkM=";

        let message = decrypt_message(key, cipher).unwrap();
        assert_eq!(message, "Another secret message!");
    }

    #[test]
    fn test_encrypt_decrypt()
    {
        let key = "key";
        let cipher = "uyqy4xhNIhIlfVZ/QmCsySxygOEOnej56pdBgzH6Yow=";
        let message = "Super secret message!";

        assert_eq!(cipher, encrypt_message(key, &decrypt_message(key, cipher).unwrap()));
        assert_eq!(message, decrypt_message(key, &encrypt_message(key, message)).unwrap());
    }
}

use magic_crypt::{new_magic_crypt, MagicCryptTrait, MagicCrypt256};

pub struct EncryptedData {
    pub encrypted_data: String,
}

impl EncryptedData {
    pub fn encrypt_data(&mut self) {
        let mcrypt = new_magic_crypt!("magickey", 256);
        let data = mcrypt.encrypt_str_to_base64(&self.encrypted_data); //Encrypts the string and saves it to the 'encrypted_string' variable.
        self.encrypted_data = data;
    }
}

pub struct DecryptedData {
    mcrypt: MagicCrypt256,
}

impl DecryptedData{
    pub fn decrypt_data(&mut self, encrypted_string: String) -> String {
        return self.mcrypt.decrypt_base64_to_string(encrypted_string).unwrap(); //Decrypts the string so we can read it.
    }
}

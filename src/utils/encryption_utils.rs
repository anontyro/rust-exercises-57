pub mod encryption_utils {

  use magic_crypt::MagicCrypt;

  pub fn encrypt_string_base64(password: String) -> String {
    let mut mc: MagicCrypt = new_magic_crypt!("magickey", 256);

    let encrypted_password = mc.encrypt_str_to_base64(password);

    return encrypted_password;
  }

  pub fn decrypt_string_base64(encrypted_password: String) -> String {
    let mut mc: MagicCrypt = new_magic_crypt!("magickey", 256);

    let password = mc.decrypt_base64_to_string(encrypted_password).unwrap();

    return password;
  }
}

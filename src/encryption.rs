use ring::pbkdf2;
use std::num::NonZeroU32;

const CREDENTIAL_LEN: usize = 32;
pub type Credential = [u8; CREDENTIAL_LEN];

const ITERATIONS: NonZeroU32 = unsafe{NonZeroU32::new_unchecked(1000_000)};
pub const SALTS_LENGTH: usize = 16;
pub type Salts = [u8; SALTS_LENGTH];

pub fn encrypt(salts: &Salts, email: &[u8], password: &[u8]) -> Credential {
    let mut salt = Vec::with_capacity(email.len() + SALTS_LENGTH);
    salt.extend(salts);
    salt.extend(email);
    let mut res: Credential = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(pbkdf2::PBKDF2_HMAC_SHA256, ITERATIONS, &salt, password, &mut res);
    res
}
pub fn verify(salts: &Salts, email: &[u8], attempted_password: &[u8], actual_password: &[u8]) -> bool {
    let mut salt = Vec::with_capacity(email.len() + SALTS_LENGTH);
    salt.extend(salts);
    salt.extend(email);
    match pbkdf2::verify(pbkdf2::PBKDF2_HMAC_SHA256, ITERATIONS, &salt, attempted_password, actual_password) {
        Ok(()) => return true,
        Err(_) => return false
    }
}
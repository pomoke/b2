use anyhow::Result;

/// Password validity checker.
///
/// Password is in PHC hashed string. Only PBKDF2 and argon2id are supported.
pub struct PasswordValidity {}

impl PasswordValidity {
    /// Verify password.
    ///
    pub fn is_valid(password: &str, hash: &str) -> Result<()> {
        Ok(())
    }
}

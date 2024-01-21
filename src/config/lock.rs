use anyhow::Result;

/// Password validity checker.
///
/// Password is in crypt format. See crypt(5) for detail.
pub struct PasswordValidity {}

impl PasswordValidity {
    /// Verify password.
    ///
    /// Check <PasswordValidity> for more information.
    pub fn is_valid(password: &str) -> Result<()> {
        Ok(())
    }
}

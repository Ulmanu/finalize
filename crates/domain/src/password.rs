//! Password policy value object.

use thiserror::Error;

/// Minimum accepted password length.
pub const MIN_LENGTH: usize = 8;

/// Maximum accepted password length (bounded to keep Argon2 inputs sane).
pub const MAX_LENGTH: usize = 200;

/// Password policy failures.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum PasswordError {
    /// The password is shorter than [`MIN_LENGTH`].
    #[error("password must be at least {MIN_LENGTH} characters")]
    TooShort,
    /// The password is longer than [`MAX_LENGTH`].
    #[error("password must be at most {MAX_LENGTH} characters")]
    TooLong,
    /// The password is only whitespace.
    #[error("password cannot be only whitespace")]
    Blank,
    /// The password equals the account email or its local part.
    #[error("password is too similar to the email address")]
    TooSimilarToEmail,
}

/// A validated password that never renders its contents.
#[derive(Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    /// Validates a password against the policy.
    ///
    /// # Errors
    ///
    /// Returns the specific [`PasswordError`] when the policy is violated.
    pub fn parse(input: &str) -> Result<Self, PasswordError> {
        if input.len() < MIN_LENGTH {
            return Err(PasswordError::TooShort);
        }
        if input.len() > MAX_LENGTH {
            return Err(PasswordError::TooLong);
        }
        if input.trim().is_empty() {
            return Err(PasswordError::Blank);
        }
        Ok(Self(input.to_string()))
    }

    /// Validates a password and rejects values too similar to the account email.
    ///
    /// # Errors
    ///
    /// Returns [`PasswordError::TooSimilarToEmail`] in addition to the [`Password::parse`]
    /// errors.
    pub fn parse_for_email(input: &str, email_normalized: &str) -> Result<Self, PasswordError> {
        let password = Self::parse(input)?;
        let lowered = password.0.to_lowercase();
        let local_part = email_normalized
            .split('@')
            .next()
            .unwrap_or(email_normalized);
        if lowered == email_normalized || (!local_part.is_empty() && lowered == local_part) {
            return Err(PasswordError::TooSimilarToEmail);
        }
        Ok(password)
    }

    /// Returns the password material for hashing.
    ///
    /// Callers must not log or persist the returned value in plaintext.
    #[must_use]
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Password(redacted)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_passwords_within_policy() {
        assert!(Password::parse("correct horse battery").is_ok());
    }

    #[test]
    fn rejects_short_blank_and_long_passwords() {
        assert_eq!(Password::parse("short"), Err(PasswordError::TooShort));
        assert_eq!(Password::parse("        "), Err(PasswordError::Blank));
        let long = "x".repeat(MAX_LENGTH + 1);
        assert_eq!(Password::parse(&long), Err(PasswordError::TooLong));
    }

    #[test]
    fn rejects_passwords_equal_to_email_parts() {
        assert_eq!(
            Password::parse_for_email("person@example.com", "person@example.com"),
            Err(PasswordError::TooSimilarToEmail)
        );
        assert_eq!(
            Password::parse_for_email("person.person", "person.person@example.com"),
            Err(PasswordError::TooSimilarToEmail)
        );
        assert!(Password::parse_for_email("different-password", "person@example.com").is_ok());
    }

    #[test]
    fn debug_output_is_redacted() {
        let password = Password::parse("super-secret-value").expect("valid");
        assert_eq!(format!("{password:?}"), "Password(redacted)");
    }
}

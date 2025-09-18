use std::fmt::Display;

use serde::{de, Deserialize, Serialize};
use validator::ValidateEmail;

use crate::domain::error::UserError;

#[derive(Debug, Serialize)]
pub struct UserEmail(String);


impl Display for UserEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


impl<'de> Deserialize<'de> for UserEmail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let string = String::deserialize(deserializer)?;
        let email = UserEmail::parse(string).map_err(|e| de::Error::custom(e))?;
        Ok(email)
    }
}

impl UserEmail {
    pub fn parse(s: String) -> Result<UserEmail, UserError> {
        if s.validate_email() {
            Ok(Self(s))
        } else {
            Err(UserError::ParseUserEmailError(s))
        }
    }
}

impl AsRef<str> for UserEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::UserEmail;
    use claims::{assert_err, assert_ok};

    #[test]
    fn test_email_ok() {
        assert_ok!(UserEmail::parse("user@example.com".into()));
    }

    #[test]
    fn test_empty_email_fail() {
        assert_err!(UserEmail::parse("".into()));
    }

    #[test]
    fn test_missing_subject_fail() {
        assert_err!(UserEmail::parse("@gmail.com".into()));
    }

    #[test]
    fn test_missing_at_sign_fail() {
        assert_err!(UserEmail::parse("gmail.com".into()));
    }
}


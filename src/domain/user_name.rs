use crate::domain::error::UserError;
use std::fmt::Display;

use serde::{de, Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;


#[derive(Debug, Serialize)]
pub struct UserName(String);

impl Display for UserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl UserName {
    pub fn parse(s: String) -> Result<UserName, UserError> {
        let is_empty = s.trim().is_empty();

        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_chars = s.chars().any(|g| forbidden_chars.contains(&g));
        
        if is_empty || is_too_long || contains_forbidden_chars {
            Err(UserError::ParseUserNameError(s))
        } else {
            Ok(Self(s))
        }

    }
}

impl<'de> Deserialize<'de> for UserName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let string = String::deserialize(deserializer)?;
        let name = UserName::parse(string).map_err(|e| de::Error::custom(e))?;
        Ok(name)
    }
}

impl AsRef<String> for UserName {
    fn as_ref(&self) -> &String {
        &self.0 
    }
}

#[cfg(test)]
mod tests {
    use super::UserName;
    use claims::{assert_err, assert_ok};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_name_prop(s in "[a-zA-Z]{255,256}") {
            assert_ok!(UserName::parse(s));
        }
    }

    #[test]
    fn test_name_ok() {
        assert_ok!(UserName::parse("Test User".into()));
    }

    #[test]
    fn test_empty_name_fail() {
        assert_err!(UserName::parse("".into()));
    }

    #[test]
    fn test_invalid_name_fail() {
        assert_err!(UserName::parse("\"".into()));
    }

    #[test]
    fn test_long_name_fail() {
        let name = "AaaAAaaAaAaaaAaaaaAAaAAAaaaAaaaaAaaAaAaAAAaaaAAAaaAaaAAaaaAaAAAAAaAAaaaaaaAaaAAAAaaaaaAAaAAAAaAAAaAAAaAAAAAAAAAaAaAaaaaaaaAAAAaAAAaaAaaAABZeoqULfJKySsSwWubOuYhApypnREEVnhQynUKKdPlbmTyUnXLQBDCacYDIeHmXaYlyWPbzfLIerqWnIuvIybYrrGkwuimXVafnYWPFZwbnUPlxclfxedrCc";
        assert_err!(UserName::parse(name.into()));
    }
}


use std::borrow::Cow;

use lazy_static::lazy_static;
use passwords::{analyzer, scorer};
use regex::Regex;
use serde::Serialize;
use std::result;
use validator::ValidationError;

lazy_static! {
    pub static ref CODE_REGEX: Regex = Regex::new(r"\A[A-Z]+[[A-Z]_]*\z").unwrap();
}

#[derive(Serialize, PartialEq, Debug, Eq)]
pub enum PasswordSecurityLevel {
    VeryDangerous = 10,
    Dangerous = 20,
    VeryWeak = 30,
    Weak = 40,
    Good = 50,
    Strong = 60,
    VeryStrong = 70,
    Invulnerable = 80,
}

impl PartialOrd for PasswordSecurityLevel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (*self as isize).partial_cmp(&(*other as isize))
    }
}

impl From<u8> for PasswordSecurityLevel {
    fn from(s: u8) -> Self {
        if s < 20 {
            PasswordSecurityLevel::VeryDangerous
        } else if (20..40).contains(&s) {
            PasswordSecurityLevel::Dangerous
        } else if (40..60).contains(&s) {
            PasswordSecurityLevel::VeryWeak
        } else if (60..80).contains(&s) {
            PasswordSecurityLevel::Weak
        } else if (80..90).contains(&s) {
            PasswordSecurityLevel::Good
        } else if (90..95).contains(&s) {
            PasswordSecurityLevel::Strong
        } else if (95..99).contains(&s) {
            PasswordSecurityLevel::VeryStrong
        } else {
            PasswordSecurityLevel::Invulnerable
        }
    }
}

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    let level = PasswordSecurityLevel::from(scorer::score(&analyzer::analyze(password)) as u8);
    match level {
        PasswordSecurityLevel::Invulnerable
        | PasswordSecurityLevel::VeryStrong
        | PasswordSecurityLevel::Strong
        | PasswordSecurityLevel::Good => Ok(()),
        _ => {
            let mut e = ValidationError::new("password");
            e.add_param(Cow::from("level"), &level);
            result::Result::Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::commons::validators::validate_password;

    mod password_security_level {
        use crate::commons::validators::PasswordSecurityLevel;

        #[test]
        fn test_from_f64_for_password_security_level() {
            let input: Vec<(u8, PasswordSecurityLevel)> = vec![
                (0, PasswordSecurityLevel::VeryDangerous),
                (19, PasswordSecurityLevel::VeryDangerous),
                (20, PasswordSecurityLevel::Dangerous),
                (39, PasswordSecurityLevel::Dangerous),
                (40, PasswordSecurityLevel::VeryWeak),
                (59, PasswordSecurityLevel::VeryWeak),
                (60, PasswordSecurityLevel::Weak),
                (79, PasswordSecurityLevel::Weak),
                (80, PasswordSecurityLevel::Good),
                (89, PasswordSecurityLevel::Good),
                (90, PasswordSecurityLevel::Strong),
                (94, PasswordSecurityLevel::Strong),
                (95, PasswordSecurityLevel::VeryStrong),
                (98, PasswordSecurityLevel::VeryStrong),
                (99, PasswordSecurityLevel::Invulnerable),
                (200, PasswordSecurityLevel::Invulnerable),
            ];

            for (score, level) in input {
                assert_eq!(PasswordSecurityLevel::from(score), level);
            }
        }

        #[test]
        fn test_password_security_level_part_ord() {
            assert!(PasswordSecurityLevel::Invulnerable == PasswordSecurityLevel::Invulnerable);

            assert!(PasswordSecurityLevel::Invulnerable > PasswordSecurityLevel::VeryStrong);
            assert!(PasswordSecurityLevel::VeryStrong > PasswordSecurityLevel::Strong);
            assert!(PasswordSecurityLevel::Strong > PasswordSecurityLevel::Good);
            assert!(PasswordSecurityLevel::Good > PasswordSecurityLevel::Weak);
            assert!(PasswordSecurityLevel::Weak > PasswordSecurityLevel::VeryWeak);
            assert!(PasswordSecurityLevel::VeryWeak > PasswordSecurityLevel::Dangerous);
            assert!(PasswordSecurityLevel::Dangerous > PasswordSecurityLevel::VeryDangerous);
        }
    }

    mod code_regex {
        use crate::commons::validators::CODE_REGEX;

        #[test]
        fn test_code_regex() {
            assert!(CODE_REGEX.is_match("CODE"));
            assert!(CODE_REGEX.is_match("CODE_AAAAAA_BBBBBB_"));

            assert!(!CODE_REGEX.is_match(" CODE"));
            assert!(!CODE_REGEX.is_match("_CODE"));
            assert!(!CODE_REGEX.is_match("_c"));
        }
    }

    #[test]
    fn test_validate_password() {
        let x = validate_password("Euge").err().unwrap();
        println!("{}", x);
    }
}

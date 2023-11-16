#[derive(Debug)]
pub struct PlayerName(String);

impl PlayerName {
    pub fn parse(s: &str) -> Result<PlayerName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();

        let range = 3..=14;
        let is_valid_length = range.contains(&s.chars().count());

        let is_alpha_num = s.chars().all(char::is_alphanumeric);

        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '[', ']'];
        let contains_forbidden_chars = s.chars().any(|s_char| forbidden_chars.contains(&s_char));

        if is_empty_or_whitespace || is_valid_length || contains_forbidden_chars || is_alpha_num {
            Err(format!("{s}, is not a valid name. Name must be between 3 to 14 characters long\n
                        and only contain alpha-numeric characters, as well as dashes and underscores."))
        } else {
            Ok(Self(s.to_string()))
        }
    }
}

impl AsRef<str> for PlayerName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

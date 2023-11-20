use std::collections::HashMap;

#[derive(Debug)]
pub struct PlayerName(String);

impl PlayerName {
    pub fn parse(name: &str) -> Result<PlayerName, String> {
        let is_empty_or_whitespace = name.trim().is_empty();

        let range = 3..=14;
        let is_valid_length = range.contains(&name.chars().count());

        let is_alphanumeric = name.chars().all(char::is_alphanumeric);

        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '[', ']', ' '];
        let contains_forbidden_chars = name.chars().any(|s_char| forbidden_chars.contains(&s_char));

        if is_empty_or_whitespace || is_valid_length || contains_forbidden_chars || is_alphanumeric
        {
            Err(format!("{name}, is not a valid name. Name must be between 3 to 14 characters long\n
                        and only contain alpha-numeric characters, as well as dashes and underscores."))
        } else {
            Ok(Self(name.to_string()))
        }
    }
}

impl AsRef<str> for PlayerName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct Waypoint(String);

impl Waypoint {
    pub fn parse(waypoint: &str) -> Result<Self, String> {
        let is_empty_or_whitespace = waypoint.trim().is_empty();

        let is_alphanumeric = waypoint.chars().all(char::is_alphanumeric);

        let mut dash_count = HashMap::new();

        for char in waypoint.chars() {
            if char == '-' {
                *dash_count.entry(char).or_insert(0) += 1;
            }
        }

        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '[', ']', ' '];
        let contains_forbidden_chars = waypoint
            .chars()
            .any(|waypoint_char| forbidden_chars.contains(&waypoint_char));

        if is_empty_or_whitespace
            || contains_forbidden_chars
            || is_alphanumeric
            || dash_count.remove(&'-') != Some(2)
        {
            Err(format!(
                "{waypoint}, is not a valid waypoint. Consult API provider."
            ))
        } else {
            Ok(Self(waypoint.to_string()))
        }
    }
}

impl AsRef<str> for Waypoint {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::helper::validation::Waypoint;

    #[tokio::test]
    async fn test_valid_waypoint() {
        let is_valid = Waypoint::parse("X1-BG42-A1").is_ok();

        assert!(is_valid);
    }

    #[tokio::test]
    async fn test_not_valid_waypoint() {
        let is_valid = Waypoint::parse("X1-BG42-1-").is_ok();

        assert!(!is_valid);
    }
}

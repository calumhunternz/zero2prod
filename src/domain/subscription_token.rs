use rand::{distributions::Alphanumeric, thread_rng, Rng};

#[derive(serde::Deserialize, Debug)]
pub struct SubscriptionToken(String);

impl SubscriptionToken {
    pub fn parse(s: String) -> Result<SubscriptionToken, String> {
        if s.len() == 25 && s.chars().all(char::is_alphanumeric) {
            Ok(Self(s))
        } else {
            Err(format!("Invalid Token: {}.", s))
        }
    }

    pub fn generate_subscription_token() -> Self {
        let mut rng = thread_rng();
        let token = std::iter::repeat_with(|| rng.sample(Alphanumeric))
            .map(char::from)
            .take(25)
            .collect();
        Self(token)
    }
}

impl AsRef<str> for SubscriptionToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use claims::{assert_err, assert_ok};

    use crate::domain::SubscriptionToken;

    #[test]
    fn generated_token_is_valid() {
        let token = SubscriptionToken::generate_subscription_token();
        assert_ok!(SubscriptionToken::parse(token.as_ref().into()));
    }

    #[test]
    fn token_too_long_is_error() {
        let token = "xxxxxxxxxxxxxxxxxxxxxxxxxx";
        assert_err!(SubscriptionToken::parse(token.into()));
    }
    #[test]
    fn token_too_short_is_error() {
        let token = "x";
        assert_err!(SubscriptionToken::parse(token.into()));
    }

    #[test]
    fn token_with_non_alphanumeric_char_is_error() {
        let token = "xxxxxxxxxxxx^xxxxxxxxxxxx";
        assert_err!(SubscriptionToken::parse(token.into()));
    }
}

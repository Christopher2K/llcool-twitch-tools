pub enum SessionKey {
    OAuthState,
    User
}

impl SessionKey {
    pub fn as_str(&self) -> String {
        match self {
            Self::OAuthState => String::from("oauth_state_session"),
            Self::User => String::from("user_state_session"),
        }
    }
}

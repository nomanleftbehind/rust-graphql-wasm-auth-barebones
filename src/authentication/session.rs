use cookie::Cookie;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct SessionCookie<'a>(pub Option<Cookie<'a>>);

pub struct UserCredential(Option<SessionData>);

impl UserCredential {
    pub fn new(session: Option<SessionData>) -> Self {
        Self(session)
    }
    pub fn is_anonymous(&self) -> bool {
        self.0.is_none()
    }
    pub fn user_id(&self) -> Option<Uuid> {
        Some(self.0.as_ref()?.user_id)
    }
    pub fn session(&self) -> Option<&SessionData> {
        self.0.as_ref()
    }
}

#[derive(Serialize, Deserialize)]
pub struct SessionData {
    pub user_id: Uuid,
    pub secret: String,
}

impl<'a> TryFrom<&Cookie<'a>> for SessionData {
    type Error = String;

    fn try_from(cookie: &Cookie) -> Result<Self, Self::Error> {
        let val = cookie.value();
        serde_json::from_str(val).map_err(|_| "invalid session cookie".to_owned())
    }
}

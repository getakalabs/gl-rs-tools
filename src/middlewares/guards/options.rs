#[derive(Debug, Clone, PartialEq)]
pub struct GuardOptions<T: ToString + Clone + PartialEq> {
    pub token: String,
    pub roles: Option<Vec<T>>,
    pub json_response: bool,
    pub is_optional: bool,
    pub is_refresh_token: bool,
    pub is_web_token: bool,
}

impl<T: ToString + Clone + PartialEq> Default for GuardOptions<T> {
    fn default() -> Self {
        Self {
            token: String::default(),
            roles: None,
            json_response: false,
            is_optional: false,
            is_refresh_token: false,
            is_web_token: false,
        }
    }
}

impl <T: ToString + Clone + PartialEq> GuardOptions<T> {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.clone() == Self::default()
    }
}

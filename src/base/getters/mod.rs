use crate::Base;

impl Base {
    pub fn get_api_url(&self) -> String {
        match self.api_url.clone() {
            None => String::default(),
            Some(data) => data.to_string().unwrap_or(String::default())
        }
    }

    pub fn get_web_url(&self) -> String {
        match self.web_url.clone() {
            None => String::default(),
            Some(data) => data.to_string().unwrap_or(String::default())
        }
    }

    pub fn get_admin_url(&self) -> String {
        match self.admin_url.clone() {
            None => String::default(),
            Some(data) => data.to_string().unwrap_or(String::default())
        }
    }
}
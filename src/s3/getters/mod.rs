use crate::S3;

impl S3 {
    pub fn get_access_key_id(&self) -> String {
        match self.access_key_id.clone() {
            None => String::default(),
            Some(data) => data.to_string().unwrap_or(String::default())
        }
    }

    pub fn get_secret_access_key(&self) -> String {
        match self.secret_access_key.clone() {
            None => String::default(),
            Some(data) => data.to_string().unwrap_or(String::default())
        }
    }

    pub fn get_bucket(&self) -> String {
        match self.bucket.clone() {
            None => String::default(),
            Some(data) => data.to_string().unwrap_or(String::default())
        }
    }

    pub fn get_path(&self) -> String {
        match self.path.clone() {
            None => String::default(),
            Some(data) => data.to_string().unwrap_or(String::default())
        }
    }

    pub fn get_region(&self) -> String {
        match self.region.clone() {
            None => String::default(),
            Some(data) => data.to_string().unwrap_or(String::default())
        }
    }
}
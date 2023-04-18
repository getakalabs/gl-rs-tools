use crate::settings::Settings;

use crate::MongoObjectId;
use crate::MongoDateTime;

impl Settings {
    pub fn set_insertable(&self) -> Self {
        let mut settings = self.clone();

        if settings.id.is_none() {
            settings.id = Some(MongoObjectId::new());
        }

        if settings.created_at.is_none() {
            settings.created_at = Some(MongoDateTime::new());
        }

        if settings.updated_at.is_none() {
            settings.updated_at = Some(MongoDateTime::new());
        }

        settings
    }
}
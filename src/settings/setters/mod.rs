use crate::Base;
use crate::Mailer;
use crate::Module;
use crate::MongoObjectId;
use crate::MongoDateTime;
use crate::Paseto;
use crate::S3;
use crate::Settings;

impl Settings {
    pub fn set_insertable(&self) -> Self {
        let mut settings = self.clone();
        settings.id = Some(MongoObjectId::new());
        settings.created_at = Some(MongoDateTime::new());
        settings.updated_at = Some(MongoDateTime::new());
        settings
    }

    pub fn set_module(&self, value: &Module) -> Self {
        let mut settings = self.clone();
        settings.module = Some(value.clone());
        settings
    }

    pub fn set_base(&self, value: &Base) -> Self {
        let mut settings = self.clone();
        settings.base = Some(value.clone());
        settings.module = Some(Module::Base);
        settings
    }

    pub fn set_mailer(&self, value: &Mailer) -> Self {
        let mut settings = self.clone();
        settings.mailer = Some(value.clone());
        settings.module = Some(Module::Mailer);
        settings
    }

    pub fn set_paseto(&self, value: &Paseto) -> Self {
        let mut settings = self.clone();
        settings.paseto = Some(value.clone());
        settings.module = Some(Module::Paseto);
        settings
    }

    pub fn set_s3(&self, value: &S3) -> Self {
        let mut settings = self.clone();
        settings.s3 = Some(value.clone());
        settings.module = Some(Module::S3);
        settings
    }
}
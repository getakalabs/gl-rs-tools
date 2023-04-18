use crate::Paseto;

impl Paseto {
    pub fn mutate(&mut self, form: &Self) {
        self.app_name = form.app_name.clone();
        self.access_token_key_unit = form.access_token_key_unit.clone();
        self.access_token_key_time = form.access_token_key_time.clone();
        self.access_token_key_signing = form.access_token_key_signing.clone();
        self.refresh_token_key_unit = form.refresh_token_key_unit.clone();
        self.refresh_token_key_time = form.refresh_token_key_time.clone();
        self.refresh_token_key_signing = form.refresh_token_key_signing.clone();
    }
}
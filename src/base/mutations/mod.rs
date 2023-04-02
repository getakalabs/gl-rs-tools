use crate::Base;

impl Base {
    pub fn mutate(&mut self, form: &Self) {
        self.api_url = form.api_url.clone();
        self.web_url = form.web_url.clone();
        self.admin_url = form.admin_url.clone();
    }

    pub fn clear(&mut self) {
        self.api_url = None;
        self.web_url = None;
        self.admin_url = None;
    }
}
use crate::S3;

impl S3 {
    pub fn mutate(&mut self, form: &Self) {
        self.access_key_id = form.access_key_id.clone();
        self.secret_access_key = form.secret_access_key.clone();
        self.bucket = form.bucket.clone();
        self.path = form.path.clone();
        self.region = form.region.clone();
        self.image_small_size = form.image_small_size;
        self.image_medium_size = form.image_medium_size;
        self.image_large_size = form.image_large_size;
        self.image_xl_size = form.image_xl_size
    }

    pub fn clear(&mut self) {
        self.access_key_id = None;
        self.secret_access_key = None;
        self.bucket = None;
        self.path = None;
        self.region = None;
        self.image_small_size = None;
        self.image_medium_size = None;
        self.image_large_size = None;
        self.image_xl_size = None;
    }
}
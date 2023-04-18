use crate::S3;

impl S3 {
    pub fn mutate(&mut self, form: &Self) {
        self.access_key_id = form.access_key_id.clone();
        self.secret_access_key = form.secret_access_key.clone();
        self.bucket = form.bucket.clone();
        self.path = form.path.clone();
        self.region = form.region.clone();
        self.image_thumbnail_small_size = form.image_thumbnail_small_size;
        self.image_thumbnail_medium_size = form.image_thumbnail_medium_size;
        self.image_thumbnail_large_size = form.image_thumbnail_large_size;
        self.image_thumbnail_xl_size = form.image_thumbnail_xl_size;
        self.image_landscape_width_small_size = form.image_landscape_width_small_size;
        self.image_landscape_height_small_size = form.image_landscape_height_small_size;
        self.image_landscape_width_medium_size = form.image_landscape_width_medium_size;
        self.image_landscape_height_medium_size = form.image_landscape_height_medium_size;
        self.image_landscape_width_large_size = form.image_landscape_width_large_size;
        self.image_landscape_height_large_size = form.image_landscape_height_large_size;
        self.image_landscape_width_xl_size = form.image_landscape_width_xl_size;
        self.image_landscape_height_xl_size = form.image_landscape_height_xl_size;
        self.image_landscape_width_xxl_size = form.image_landscape_width_xxl_size;
        self.image_landscape_height_xxl_size = form.image_landscape_height_xxl_size;
        self.image_landscape_width_xxxl_size = form.image_landscape_width_xxxl_size;
        self.image_landscape_height_xxxl_size = form.image_landscape_height_xxxl_size;
    }

    pub fn clear(&mut self) {
        self.access_key_id = None;
        self.secret_access_key = None;
        self.bucket = None;
        self.path = None;
        self.region = None;
        self.image_thumbnail_small_size = None;
        self.image_thumbnail_medium_size = None;
        self.image_thumbnail_large_size = None;
        self.image_thumbnail_xl_size = None;
        self.image_landscape_width_small_size = None;
        self.image_landscape_height_small_size = None;
        self.image_landscape_width_medium_size = None;
        self.image_landscape_height_medium_size = None;
        self.image_landscape_width_large_size = None;
        self.image_landscape_height_large_size = None;
        self.image_landscape_width_xl_size = None;
        self.image_landscape_height_xl_size = None;
        self.image_landscape_width_xxl_size = None;
        self.image_landscape_height_xxl_size = None;
        self.image_landscape_width_xxxl_size = None;
        self.image_landscape_height_xxxl_size = None;
    }
}
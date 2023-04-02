pub mod getters;
pub mod mutations;

use arraygen::Arraygen;
use actix_web::{ http::header, web::{Bytes, BytesMut, Data}, HttpRequest, HttpResponse };
use bstr::ByteSlice;
use futures::TryStreamExt;
use handlebars::Handlebars;
use image::{GenericImageView, ImageFormat, Rgba};
use image::imageops::FilterType;
use infer::Infer;
use reqwest;
use rusoto_core::credential::{StaticProvider};
use rusoto_core::{HttpClient, Region};
use rusoto_s3::{DeleteObjectRequest, GetObjectRequest, PutObjectRequest, S3 as RusotoS3, S3Client};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::default::Default;
use std::fs::File as StdFile;
use std::io::{Cursor, Read};
use std::str::FromStr;
use std::sync::{Arc, RwLock};

use crate::traits::prelude::*;
use crate::catchers;
use crate::parsers;
use crate::Asset;
use crate::Cipher;
use crate::DBClient;
use crate::Errors;
use crate::Module;
use crate::Settings;

// TODO: Use OpenCV to detect faces or content aware and use it as gravity
// use opencv::objdetect::CascadeClassifierTrait;
// use opencv::prelude::MatTraitConstManual;
// use opencv::{
//     imgcodecs,
//     core::{
//         Mat, Size,
//         Point, Rect,
//     },
//     objdetect::CascadeClassifier,
// };

pub const XS_WIDTH_ASPECT_RATIO_16_9: u32 = 640;
pub const XS_HEIGHT_ASPECT_RATIO_16_9: u32 = 360;
pub const SMALL_WIDTH_ASPECT_RATIO_16_9: u32 = 854;
pub const SMALL_HEIGHT_ASPECT_RATIO_16_9: u32 = 480;
pub const MEDIUM_WIDTH_ASPECT_RATIO_16_9: u32 = 960;
pub const MEDIUM_HEIGHT_ASPECT_RATIO_16_9: u32 = 540;
pub const LARGE_WIDTH_ASPECT_RATIO_16_9: u32 = 1136;
pub const LARGE_HEIGHT_ASPECT_RATIO_16_9: u32 = 640;
pub const XL_WIDTH_ASPECT_RATIO_16_9: u32 = 1280;
pub const XL_HEIGHT_ASPECT_RATIO_16_9: u32 = 720;
pub const XXL_WIDTH_ASPECT_RATIO_16_9: u32 = 1920;
pub const XXL_HEIGHT_ASPECT_RATIO_16_9: u32 = 1080;


#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Arraygen)]
#[gen_array(fn get_ciphers: &mut Option<Cipher>)]
pub struct S3 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub access_key_id: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub secret_access_key: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub bucket: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub path: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub region: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_small_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_medium_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_large_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_xl_size: Option<i32>,
}

impl IsEmpty for S3 {
    fn is_empty(&self) -> bool {
        self.clone() == Self::default()
    }
}

impl Decrypt for S3 {
    fn decrypt(&self) -> Self {
        let mut data = self.clone();

        for cipher in data.get_ciphers() {
            *cipher = cipher.clone().and_then(|d| {
                match d.is_empty() {
                    true => None,
                    false => d.decrypt_master()
                }
            });
        }

        data
    }
}

impl Encrypt for S3 {
    fn encrypt(&self) -> Self {
        let mut data = self.clone();

        for cipher in data.get_ciphers() {
            *cipher = cipher.clone().and_then(|d| {
                match d.is_empty() {
                    true => None,
                    false => d.encrypt_master()
                }
            });
        }

        data
    }
}

impl ToBson for S3 {
    fn to_bson(&self) -> Option<Self> {
        let mut data = self.clone();

        for cipher in data.get_ciphers() {
            *cipher = match cipher {
                None => None,
                Some(data) => {
                    match data.is_empty() {
                        true => None,
                        false => data.encrypt_master()
                    }
                }
            };
        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

impl ToJson for S3 {
    fn to_json(&self) -> Option<Self> {
        let mut data = self.clone();

        for cipher in data.get_ciphers() {
            *cipher = match cipher {
                None => None,
                Some(data) => {
                    match data.is_empty() {
                        true => None,
                        false => {
                            let data = data.set_to_string();
                            match data.is_empty() {
                                true => None,
                                false => Some(data)
                            }
                        }
                    }
                }
            };
        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

impl S3 {
    pub fn new(form: &Settings) -> Self {
        Self {
            access_key_id: Cipher::new(form.get_access_key_id()),
            secret_access_key: Cipher::new(form.get_secret_access_key()),
            bucket: Cipher::new(form.get_bucket()),
            path: Cipher::new(form.get_path()),
            region: Cipher::new(form.get_region()),
            image_small_size: form.get_image_small_size(),
            image_medium_size: form.get_image_medium_size(),
            image_large_size: form.get_image_large_size(),
            image_xl_size:  form.get_image_xl_size(),
        }
    }

    pub async fn stage(client: &DBClient) -> Arc<RwLock<S3>> {
        let db = match client.get_db() {
            None => return Arc::new(RwLock::new(S3::default())),
            Some(client) => client
        };

        let settings = match Settings::read_from_module(&db, &Module::S3).await {
            Ok(settings) => settings,
            Err(_) => return Arc::new(RwLock::new(S3::default()))
        };

        let data = settings
            .s3
            .map_or(S3::default(), |d| d.decrypt());

        Arc::new(RwLock::new(data))
    }

    pub fn get_sizes(&self) -> Option<Vec<(u32, u32)>> {
        Some(vec![
            (self.image_small_size.unwrap() as u32, self.image_small_size.unwrap() as u32),
            (self.image_medium_size.unwrap() as u32, self.image_medium_size.unwrap() as u32),
            (self.image_large_size.unwrap() as u32, self.image_large_size.unwrap() as u32),
            (self.image_xl_size.unwrap() as u32, self.image_xl_size.unwrap() as u32),
        ])
    }

    pub fn get_client(&self) -> Option<S3Client> {
        // Client is unavailable
        if self.is_empty() {
            return None;
        }

        // Set access, secret access key & region
        let access_key = self.get_access_key_id();
        let secret_access_key = self.get_secret_access_key();
        let region = Region::from_str(&self.get_region());
        if region.is_err() {
            return None;
        }

        // Unwrap region
        let region = region.unwrap();

        // Set aws credentials
        let credentials = StaticProvider::new_minimal(access_key, secret_access_key);

        // Set client
        let client = S3Client::new_with(
            HttpClient::new().expect("Failed to create request dispatcher"),
            credentials,
            region,
        );

        // Return client
        Some(client)
    }

    /// Author: Deneir Uy
    /// Email: deneir.uy@getakalabs.com
    pub async fn get<T>(&self, hbs: Data<Handlebars<'_>>, filename: T, req: HttpRequest) -> HttpResponse
        where T: ToString
    {
        // Create bindings
        let bindings = filename.to_string();

        // Retrieve client
        let client = match self.get_client() {
            None => return catchers::not_found_page(hbs).await.unwrap(),
            Some(client) => client
        };

        // Retrieve item
        let result = client
            .get_object(GetObjectRequest{
                bucket: self.get_bucket(),
                key: format!("{}/{}", self.get_path().as_str(), bindings),
                ..Default::default()
            })
            .await;

        // Retrieve result
        if result.is_err() {
            return catchers::not_found_page(hbs).await.unwrap();
        }

        // Pass result to object
        let object = result.unwrap();
        let body = object.body;
        if body.is_none() {
            return catchers::not_found_page(hbs).await.unwrap();
        }

        // Set content type
        let content_type = object
            .content_type
            .unwrap_or_else(|| "application/octet-stream".to_owned());

        // Set response for requests containing range header
        if let Some(range_header) = req.headers().get(header::RANGE) {
            if let Ok((start, end)) = parsers::headers::get(range_header) {
                let body_bytes = body
                    .unwrap()
                    .map_ok(|b| BytesMut::from(&b[..]))
                    .try_concat()
                    .await;

                return match body_bytes {
                    Ok(b) => {
                        let accept_ranges = object.accept_ranges.unwrap_or_else(|| "bytes".to_owned());
                        let content_length = b.len();

                        HttpResponse::PartialContent()
                            .content_type(content_type)
                            .append_header((header::ACCEPT_RANGES, accept_ranges))
                            .append_header((header::CONTENT_LENGTH, content_length))
                            .append_header((
                                header::CONTENT_RANGE,
                                format!("bytes  {start}-{end}/{content_length}"),
                            ))
                            .body(Bytes::from(b[start..=end].to_vec()))
                    }
                    Err(_) => HttpResponse::InternalServerError().finish(),
                }
            }
        }

        HttpResponse::Ok()
            .content_type(content_type)
            .streaming(body.unwrap().map_ok(|chunk| chunk))
    }

    pub async fn delete(&self, file: &Asset) -> Result<(), Errors> {
        // Retrieve client
        let client = match self.get_client() {
            None => return Err(Errors::new("S3 client failed to initialize")),
            Some(data) => data
        };

        // Retrieve filename and extension
        let filename = file.filename.clone().unwrap_or(String::default());
        let extension = file.extension.clone().unwrap_or(String::default());

        // Set bucket & path
        let bucket = self.get_bucket();
        let path = self.get_path();

        // Delete original file
        _ = client
            .delete_object(DeleteObjectRequest{
                bucket: bucket.clone(),
                key: format!("{path}/{filename}-original{extension}"),
                ..Default::default()
            })
            .await;

        // Check if sizes exists
        match self.get_sizes() {
            None => {}
            Some(sizes) => {
                // Delete for each sizes
                for (width, height) in sizes {
                    _ = client
                        .delete_object(DeleteObjectRequest{
                            bucket: bucket.clone(),
                            key: format!("{path}/{filename}-{width}x{height}.webp"),
                            ..Default::default()
                        })
                        .await;
                }
            }
        }

        // Return ok
        Ok(())
    }

    pub async fn upload_from_url<U, F>(&self, url: U, filename: F, sizes: Option<Vec<(u32, u32)>>, retain_size: bool) -> Result<Asset, Errors>
        where U: ToString,
              F: ToString
    {
        // Retrieve client
        let client = self.get_client();
        if client.is_none() {
            return Err(Errors::new("S3 client failed to initialize"));
        }

        // Set bindings
        let url_bindings = url.to_string();
        let filename = filename.to_string();

        // Download the image from the URL
        let response = reqwest::get(url_bindings).await;
        if response.is_err() {
            return Err(Errors::new("Unable download file from url"));
        }

        // Check bytes
        let response = response.unwrap().bytes().await;
        if response.is_err() {
            return Err(Errors::new("Unable to download file from url"));
        }

        // Upload original image
        self.upload_original(response.unwrap().as_bytes().to_vec(), &filename, sizes, retain_size).await
    }

    pub async fn upload_original<T>(&self, data: Vec<u8>, filename: T, sizes: Option<Vec<(u32, u32)>>, retain_size: bool) -> Result<Asset, Errors>
        where T: ToString
    {
        // Retrieve client
        let client = self.get_client();
        if client.is_none() {
            return Err(Errors::new("S3 client failed to initialize"));
        }

        // Shadow client
        let client = client.unwrap();

        // Check out mime type
        let info = Infer::new();
        let mime = info
            .get(&data.clone())
            .map_or(String::default(), |t| String::from(t.mime_type()));

        // Set filename
        let legacy_filename = filename.to_string();
        let extension = crate::strings::get_extension_from_mime(mime.clone());
        let filename = crate::strings::change_extension(legacy_filename.clone(), extension.clone());
        let filename = crate::strings::replace_filename(filename, "original");

        // Set metadata
        let mut metadata = HashMap::new();
        metadata.insert(String::from("filename"), filename.clone());

        // Upload original image to s3
        let request = PutObjectRequest {
            metadata: Some(metadata),
            bucket: self.get_bucket(),
            key: format!("{}/{}", self.get_path(), filename.clone()),
            body: Some(data.clone().into()),
            acl: Some("public-read".to_owned()),
            content_type: Some(mime.clone()),
            ..Default::default()
        };

        // Upload file
        let result = client.put_object(request).await;
        if result.is_err() {
            return Err(Errors::new("Unable to upload your file"));
        }

        // Check if data is image
        let mut w = None;
        let mut h = None;
        if Asset::is_image(mime.clone()) {
            // Set file
            let mut file = data.clone();

            // Check if current mime type is image
            if sizes.is_some() && Asset::is_image(mime.clone()) {
                let mut sizes = sizes.unwrap();
                sizes.sort_by(|a, b| b.0.cmp(&a.0));

                for (width, height) in sizes {
                    match self.generate_thumbnail(file.clone(), &legacy_filename, width, height, retain_size).await {
                        Ok(value) => file = value,
                        Err(error) => return Err(error)
                    }
                }
            }

            // Load image from data
            let image = image::load_from_memory(&data);
            if image.is_err() {
                return Err(Errors::new("Unable to load image"));
            }

            // Shadow image
            let image = image.unwrap();

            // Calculate the size of the thumbnail
            let (orig_width, orig_height) = image.dimensions();
            w = Some(orig_width.to_string());
            h = Some(orig_height.to_string());
        }

        // Set file container
        let mut file = Asset::new();
        file.filename = Some(crate::strings::change_extension(legacy_filename.clone(), ""));
        file.extension = Some(extension.clone());
        file.mime_type = Some(mime.clone());
        file.width = w;
        file.height = h;
        file.file_size = Some(Asset::get_file_size(data.clone()));
        file.file_type = Some(Asset::get_file_type(mime.clone()));

        Ok(file)
    }

    pub async fn upload_aspect_ratio_19_6<T>(&self, data: Vec<u8>, filename: T) -> Result<Asset, Errors>
        where T: ToString
    {
        // Retrieve client
        let client = self.get_client();
        if client.is_none() {
            return Err(Errors::new("S3 client failed to initialize"));
        }

        // Shadow client
        let client = client.unwrap();

        // Check out mime type
        let info = Infer::new();
        let mime = info
            .get(&data.clone())
            .map_or(String::default(), |t| String::from(t.mime_type()));

        // Set filename
        let legacy_filename = filename.to_string();
        let extension = crate::strings::get_extension_from_mime(mime.clone());
        let filename = crate::strings::change_extension(legacy_filename.clone(), extension.clone());
        let filename = crate::strings::replace_filename(filename, "original");

        // Set metadata
        let mut metadata = HashMap::new();
        metadata.insert(String::from("filename"), filename.clone());

        // Upload original image to s3
        let request = PutObjectRequest {
            metadata: Some(metadata),
            bucket: self.get_bucket(),
            key: format!("{}/{}", self.get_path(), filename.clone()),
            body: Some(data.clone().into()),
            acl: Some("public-read".to_owned()),
            content_type: Some(mime.clone()),
            ..Default::default()
        };

        // Upload file
        let result = client.put_object(request).await;
        if result.is_err() {
            return Err(Errors::new("Unable to upload your file"));
        }

        // Check if data is image
        let mut w = None;
        let mut h = None;
        if Asset::is_image(mime.clone()) {
            // Set file
            let mut file = data.clone();

            // Create sizes
            let sizes = [
                (XXL_WIDTH_ASPECT_RATIO_16_9, XXL_HEIGHT_ASPECT_RATIO_16_9),
                (XL_WIDTH_ASPECT_RATIO_16_9, XL_HEIGHT_ASPECT_RATIO_16_9),
                (LARGE_WIDTH_ASPECT_RATIO_16_9, LARGE_HEIGHT_ASPECT_RATIO_16_9),
                (MEDIUM_WIDTH_ASPECT_RATIO_16_9, MEDIUM_HEIGHT_ASPECT_RATIO_16_9),
                (SMALL_WIDTH_ASPECT_RATIO_16_9, SMALL_HEIGHT_ASPECT_RATIO_16_9),
                (XS_WIDTH_ASPECT_RATIO_16_9, XS_HEIGHT_ASPECT_RATIO_16_9),
            ];

            for (width, height) in sizes {
                match self.generate_aspect_ratio_19_6(file.clone(), &legacy_filename, width, height).await {
                    Ok(value) => file = value,
                    Err(error) => return Err(error)
                }
            }

            // Load image from data
            let image = image::load_from_memory(&data);
            if image.is_err() {
                return Err(Errors::new("Unable to load image"));
            }

            // Shadow image
            let image = image.unwrap();

            // Calculate the size of the thumbnail
            let (orig_width, orig_height) = image.dimensions();
            w = Some(orig_width.to_string());
            h = Some(orig_height.to_string());
        }

        // Set file container
        let mut file = Asset::new();
        file.filename = Some(crate::strings::change_extension(legacy_filename.clone(), ""));
        file.extension = Some(extension.clone());
        file.mime_type = Some(mime.clone());
        file.width = w;
        file.height = h;
        file.file_size = Some(Asset::get_file_size(data.clone()));
        file.file_type = Some(Asset::get_file_type(mime.clone()));

        Ok(file)
    }

    pub fn get_file_type(&self, data: Vec<u8>) -> String {
        // Check out mime type
        let info = Infer::new();
        let mime = info
            .get(&data)
            .map_or(String::default(), |t| String::from(t.mime_type()));

        Asset::get_file_type(mime)
    }

    pub async fn generate_thumbnail<T>(&self, data: Vec<u8>, filename: T, width: u32, height: u32, retain_size: bool) -> Result<Vec<u8>, Errors>
        where T: ToString
    {
        // Create filename bindings
        let filename = filename.to_string();
        let filename = crate::strings::replace_filename(filename, format!("{width}x{height}"));
        let filename = crate::strings::change_extension(filename, "webp");

        // Retrieve client
        let client = self.get_client();
        if client.is_none() {
            return Err(Errors::new("S3 client failed to initialize"));
        }

        // Shadow client
        let client = client.unwrap();

        // Create image buffer
        let cursor = Cursor::new(data.clone());
        let buffer = cursor.get_ref();

        // Check out mime type
        let info = Infer::new();
        let mime = info
            .get(&buffer.clone())
            .map_or(String::default(), |t| String::from(t.mime_type()));

        // Check if data is image
        if !Asset::is_image(mime) {
            return Err(Errors::new("Invalid image type"));
        }

        // Load image from data
        let image = image::load_from_memory(&data);
        if image.is_err() {
            return Err(Errors::new("Unable to load image"));
        }

        // Shadow image
        let image = image.unwrap();

        // Calculate the size of the thumbnail
        let (orig_width, orig_height) = image.dimensions();
        let ratio = f64::min( orig_width as f64 / width as f64, orig_height as f64 / height as f64);
        let new_width = (orig_width as f64 / ratio) as u32;
        let new_height = (orig_height as f64 / ratio) as u32;

        let mut thumbnail = if retain_size {
            // image.resize(orig_width, orig_height, FilterType::Lanczos3)
            image
        } else {
            image.resize(new_width, new_height, FilterType::Triangle)
        };

        // Crop the image to a square with the center as the gravity
        let (thumb_width, thumb_height) = thumbnail.dimensions();

        // Convert to f64
        let x:f64 = (thumb_width as f64 - width as f64) / 2.0;
        let y:f64 = (thumb_height as f64 - height as f64) / 2.0;

        // Round images to u32
        let x = x.round() as u32;
        let y = y.round() as u32;

        thumbnail = thumbnail.crop(x, y, width, height);

        // Add transparent padding if needed
        let mut padded_thumbnail = image::ImageBuffer::new(width, height);
        let transparent = Rgba([0, 0, 0, 0]);
        for (_, _, pixel) in padded_thumbnail.enumerate_pixels_mut() {
            *pixel = transparent;
        }

        // Set overlay
        image::imageops::overlay(&mut padded_thumbnail, &thumbnail, x as i64, y as i64);

        // Open the file and read its contents
        let mut cursor = Cursor::new(vec![]);
        let result = thumbnail.write_to(&mut cursor, ImageFormat::WebP);
        if result.is_err() {
            return Err(Errors::new("Thumbnail generation failed"));
        }

        // Set buffer
        let buffer = cursor.get_ref();

        // Check out mime type
        let info = Infer::new();
        let mime = info
            .get(&data.clone())
            .map_or(String::default(), |t| String::from(t.mime_type()));

        // Set metadata
        let mut metadata = HashMap::new();
        metadata.insert(String::from("filename"), filename.clone());

        // Upload original image to s3
        let request = PutObjectRequest {
            metadata: Some(metadata),
            bucket: self.get_bucket(),
            key: format!("{}/{}", self.get_path(), filename),
            body: Some(buffer.clone().into()),
            acl: Some("public-read".to_owned()),
            content_type: Some(mime),
            ..Default::default()
        };

        // Upload file
        let result = client.put_object(request).await;
        if result.is_err() {
            return Err(Errors::new("Unable to upload your file"));
        }

        Ok(buffer.clone())
    }

    pub async fn generate_aspect_ratio_19_6<T>(&self, data: Vec<u8>, filename: T, width: u32, height: u32) -> Result<Vec<u8>, Errors>
        where T: ToString
    {
        // Create filename bindings
        let filename = filename.to_string();
        let filename = crate::strings::replace_filename(filename, format!("{width}x{height}"));
        let filename = crate::strings::change_extension(filename, "webp");

        // Retrieve client
        let client = self.get_client();
        if client.is_none() {
            return Err(Errors::new("S3 client failed to initialize"));
        }

        // Shadow client
        let client = client.unwrap();

        // Create image buffer
        let cursor = Cursor::new(data.clone());
        let buffer = cursor.get_ref();

        // Check out mime type
        let info = Infer::new();
        let mime = info
            .get(&buffer.clone())
            .map_or(String::default(), |t| String::from(t.mime_type()));

        // Check if data is image
        if !Asset::is_image(mime) {
            return Err(Errors::new("Invalid image type"));
        }

        // Load image from data
        let image = image::load_from_memory(&data);
        if image.is_err() {
            return Err(Errors::new("Unable to load image"));
        }

        // Shadow image
        let image = image.unwrap();

        // Calculate the size of the thumbnail
        let (orig_width, orig_height) = image.dimensions();
        let ratio = f64::min( orig_width as f64 / width as f64, orig_height as f64 / height as f64);
        let new_width = (orig_width as f64 / ratio) as u32;
        let new_height = (orig_height as f64 / ratio) as u32;

        let thumbnail = image.resize(new_width, new_height, FilterType::Triangle);

        // Open the file and read its contents
        let mut cursor = Cursor::new(vec![]);
        let result = thumbnail.write_to(&mut cursor, ImageFormat::WebP);
        if result.is_err() {
            return Err(Errors::new("Thumbnail generation failed"));
        }

        // Set buffer
        let buffer = cursor.get_ref();

        // Check out mime type
        let info = Infer::new();
        let mime = info
            .get(&data.clone())
            .map_or(String::default(), |t| String::from(t.mime_type()));

        // Set metadata
        let mut metadata = HashMap::new();
        metadata.insert(String::from("filename"), filename.clone());

        // Upload original image to s3
        let request = PutObjectRequest {
            metadata: Some(metadata),
            bucket: self.get_bucket(),
            key: format!("{}/{}", self.get_path(), filename),
            body: Some(buffer.clone().into()),
            acl: Some("public-read".to_owned()),
            content_type: Some(mime),
            ..Default::default()
        };

        // Upload file
        let result = client.put_object(request).await;
        if result.is_err() {
            return Err(Errors::new("Unable to upload your file"));
        }

        Ok(buffer.clone())
    }

    pub async fn test_image_upload(&self) -> Result<Asset, Errors> {
        use std::time::Instant;
        let start = Instant::now();

        // Set filename
        let filename = "nature-1-image.jpg";

        // Set path of sample upload
        let stream = StdFile::open(format!("./assets/sample/{filename}"));
        if stream.is_err() {
            return Err(Errors::new(format!("Sample {filename} not found in path")));
        }

        // Unwrap stream
        let mut stream = stream.unwrap();
        let mut contents: Vec<u8> = Vec::new();

        // Read file to end
        let result = stream.read_to_end(&mut contents);
        if result.is_err() {
            return Err(Errors::new("Unable to read file"));
        }

        // Create vector of width and height
        let sizes = Some(vec![
            (self.image_small_size.unwrap_or(0) as u32, self.image_small_size.unwrap_or(0) as u32),
            (self.image_medium_size.unwrap_or(0) as u32, self.image_medium_size.unwrap_or(0) as u32),
            (self.image_large_size.unwrap_or(0) as u32, self.image_large_size.unwrap_or(0) as u32),
            (self.image_xl_size.unwrap_or(0) as u32, self.image_xl_size.unwrap_or(0) as u32),
        ]);

        // Upload file
        let result = self.upload_original(contents.clone(), filename, sizes, false).await;
        let duration = start.elapsed();
        println!("Time elapsed is: {duration:?}");

        result
    }

    pub async fn test_doc_upload(&self) -> Result<(), Errors> {
        // Retrieve client
        let client = self.get_client();
        if client.is_none() {
            return Err(Errors::new("S3 client failed to initialize"));
        }

        // Shadow client
        let client = client.unwrap();

        // Set path of sample upload
        let stream = StdFile::open("./assets/sample/doc.txt");
        if stream.is_err() {
            return Err(Errors::new("Sample doc.txt not found in path"));
        }

        // Unwrap stream
        let mut stream = stream.unwrap();
        let mut contents: Vec<u8> = Vec::new();

        // Read file to end
        let result = stream.read_to_end(&mut contents);
        if result.is_err() {
            return Err(Errors::new("Unable to read file"));
        }

        // Set info
        let info = Infer::new();
        let mut mime: Option<String> = None;

        // Check out mime type
        let mime_type = info.get(&contents.clone());
        if mime_type.is_some() && !mime_type.unwrap().mime_type().is_empty() {
            mime = Some(String::from(mime_type.unwrap().mime_type()));
        }

        // Upload file
        let req = PutObjectRequest {
            bucket: self.get_bucket(),
            key: format!("{}/doc.txt", self.get_path()),
            body: Some(contents.into()),
            acl: Some("public-read".to_owned()),
            content_type: mime,
            ..Default::default()
        };

        let result = client.put_object(req).await;
        if result.is_err() {
            return Err(Errors::new("Unable to read file"));
        }

        Ok(())
    }
}


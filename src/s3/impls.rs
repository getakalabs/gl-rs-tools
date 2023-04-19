use actix_web::{ http::header, web::{Bytes, BytesMut, Data}, HttpRequest, HttpResponse, Result };
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
use std::collections::HashMap;
use std::default::Default;
use std::fs::File as StdFile;
use std::io::{Cursor, Read};
use std::str::FromStr;

use crate::catchers;
use crate::traits::IsEmpty;
use crate::Asset;
use crate::S3;
use crate::Payload;

impl S3 {
    pub fn get_client(&self) -> Result<S3Client> {
        if self.is_empty() {
            return Err(Payload::error("AWS s3 is not configured"));
        }

        let access_key = self.access_key_id.clone().map_or(String::default(), |value| value.to_string());
        let secret_access_key = self.secret_access_key.clone().map_or(String::default(), |value| value.to_string());
        let region = match Region::from_str(&self.region.clone().map_or(String::default(), |value| value.to_string())) {
            Ok(region) => region,
            Err(_) => return Err(Payload::error("AWS region is not configured"))
        };

        // Set aws credentials
        let credentials = StaticProvider::new_minimal(access_key, secret_access_key);

        // Set client
        let client = S3Client::new_with(
            HttpClient::new().expect("Failed to create request dispatcher"),
            credentials,
            region,
        );

        Ok(client)
    }

    pub fn get_file_type(&self, data: Vec<u8>) -> String {
        let info = Infer::new();
        let mime = info
            .get(&data)
            .map_or(String::default(), |t| String::from(t.mime_type()));

        Asset::get_file_type(mime)
    }

    pub async fn get<T>(&self, hbs: Data<Handlebars<'_>>, filename: T, req: HttpRequest) -> HttpResponse
        where T: ToString
    {
        // Create bindings
        let bindings = filename.to_string();

        // Retrieve client
        let client = match self.get_client() {
            Ok(client) => client,
            Err(_) => return catchers::not_found_page(hbs).await.unwrap()
        };

        // Retrieve item
        let object = match client.get_object(GetObjectRequest{
                bucket: self.bucket.clone().map_or(String::default(), |d| d.to_string()),
                key: format!("{}/{}", self.path.clone().map_or(String::default(), |d| d.to_string()).as_str(), bindings),
                ..Default::default()
            }).await {
                Ok(result) => result,
                Err(_) => return catchers::not_found_page(hbs).await.unwrap()
            };

        let body = match object.body {
            Some(body) => body,
            None => return catchers::not_found_page(hbs).await.unwrap()
        };

        // Set content type
        let content_type = object
            .content_type
            .unwrap_or_else(|| "application/octet-stream".to_owned());

        // Set response for requests containing range header
        if let Some(range_header) = req.headers().get(header::RANGE) {
            if let Ok((start, end)) = crate::parsers::headers::get(range_header) {
                let body_bytes = body
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
            .streaming(body.map_ok(|chunk| chunk))
    }

    pub async fn get_bytes<T>(&self, filename: T) -> Result<Vec<u8>>
        where T: ToString
    {
        // Create bindings
        let bindings = filename.to_string();
        let client = self.get_client()?;

        // Retrieve item
        match client.get_object(GetObjectRequest{
            bucket: self.bucket.clone().map_or(String::default(), |d| d.to_string()),
            key: format!("{}/{}", self.path.clone().map_or(String::default(), |d| d.to_string()), bindings),
            ..Default::default()
        }).await {
            Err(_) => Err(Payload::error("Unable to retrieve your file")),
            Ok(data) => {
                let bytes = data.body.unwrap().map_ok(|b| BytesMut::from(&b[..]))
                    .try_concat()
                    .await
                    .unwrap();

                Ok(bytes.to_vec())
            }
        }
    }

    pub async fn delete(&self, file: &Asset) -> Result<()> {
        // Retrieve client
        let client = self.get_client()?;

        // Retrieve filename and extension
        let filename = file.filename.clone().unwrap_or(String::default());
        let extension = file.extension.clone().unwrap_or(String::default());

        // Set bucket & path
        let bucket = self.bucket.clone().map_or(String::default(), |d| d.to_string());
        let path = self.path.clone().map_or(String::default(), |d| d.to_string());

        // Delete original file
        _ = client
            .delete_object(DeleteObjectRequest{
                bucket: bucket.clone(),
                key: format!("{path}/{filename}-original{extension}"),
                ..Default::default()
            })
            .await;

        for (width, height) in self.get_thumbnail_sizes() {
            _ = client
                .delete_object(DeleteObjectRequest{
                    bucket: bucket.clone(),
                    key: format!("{path}/{filename}-{width}x{height}.webp"),
                    ..Default::default()
                })
                .await;
        }

        for (width, height) in self.get_landscape_sizes() {
            _ = client
                .delete_object(DeleteObjectRequest{
                    bucket: bucket.clone(),
                    key: format!("{path}/{filename}-{width}x{height}.webp"),
                    ..Default::default()
                })
                .await;
        }

        Ok(())
    }

    pub fn get_thumbnail_sizes(&self) ->  Vec<(u32, u32)> {
        vec![
            (self.image_thumbnail_xl_size.unwrap() as u32, self.image_thumbnail_xl_size.unwrap() as u32),
            (self.image_thumbnail_large_size.unwrap() as u32, self.image_thumbnail_large_size.unwrap() as u32),
            (self.image_thumbnail_medium_size.unwrap() as u32, self.image_thumbnail_medium_size.unwrap() as u32),
            (self.image_thumbnail_small_size.unwrap() as u32, self.image_thumbnail_small_size.unwrap() as u32),
        ]
    }

    pub fn get_landscape_sizes(&self) ->  Vec<(u32, u32)> {
        vec![
            (self.image_landscape_width_xxxl_size.unwrap() as u32, self.image_landscape_height_xxxl_size.unwrap() as u32),
            (self.image_landscape_width_xxl_size.unwrap() as u32, self.image_landscape_height_xxl_size.unwrap() as u32),
            (self.image_landscape_width_xl_size.unwrap() as u32, self.image_landscape_height_xl_size.unwrap() as u32),
            (self.image_landscape_width_large_size.unwrap() as u32, self.image_landscape_height_large_size.unwrap() as u32),
            (self.image_landscape_width_medium_size.unwrap() as u32, self.image_landscape_height_medium_size.unwrap() as u32),
            (self.image_landscape_width_small_size.unwrap() as u32, self.image_landscape_height_small_size.unwrap() as u32),
        ]
    }

    pub async fn upload_original<T>(&self, data: Vec<u8>, filename: T, sizes: Option<Vec<(u32, u32)>>, retain_size: bool) -> Result<Asset>
        where T: ToString
    {
        // Retrieve client
        let client = self.get_client()?;


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
            bucket: self.bucket.clone().map_or(String::default(), |d| d.to_string()),
            key: format!("{}/{}", self.path.clone().map_or(String::default(), |d| d.to_string()), filename.clone()),
            body: Some(data.clone().into()),
            // acl: Some("public-read".to_owned()),
            content_type: Some(mime.clone()),
            ..Default::default()
        };

        // Upload file
        let result = client.put_object(request).await;
        if result.is_err() {
            return Err(Payload::error("Unable to upload your file"));
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
                return Err(Payload::error("Unable to load image"));
            }

            // Shadow image
            let image = image.unwrap();

            // Calculate the size of the thumbnail
            let (orig_width, orig_height) = image.dimensions();
            w = Some(orig_width.to_string());
            h = Some(orig_height.to_string());
        }

        // Set file container
        let file = Asset{
            filename: Some(crate::strings::change_extension(legacy_filename.clone(), "")),
            extension: Some(extension.clone()),
            mime_type: Some(mime.clone()),
            width: w,
            height: h,
            file_size: Some(Asset::get_file_size(data.clone())),
            file_type: Some(Asset::get_file_type(mime.clone())),
            ..Default::default()
        };

        Ok(file)
    }

    pub async fn upload_from_url<U, F>(&self, url: U, filename: F, sizes: Option<Vec<(u32, u32)>>, retain_size: bool) -> Result<Asset>
        where U: ToString,
              F: ToString
    {
        // Set bindings
        let url_bindings = url.to_string();
        let filename = filename.to_string();

        // Download the image from the URL
        let response = reqwest::get(url_bindings).await;
        if response.is_err() {
            return Err(Payload::error("Unable download file from url"));
        }

        // Check bytes
        let response = response.unwrap().bytes().await;
        if response.is_err() {
            return Err(Payload::error("Unable to download file from url"));
        }

        // Upload original image
        self.upload_original(response.unwrap().as_bytes().to_vec(), &filename, sizes, retain_size).await
    }

    pub async fn upload_aspect_ratio_19_6<T>(&self, data: Vec<u8>, filename: T) -> Result<Asset>
        where T: ToString
    {
        // Retrieve client
        let client = self.get_client()?;

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
            bucket: self.bucket.clone().map_or(String::default(), |d| d.to_string()),
            key: format!("{}/{}", self.path.clone().map_or(String::default(), |d| d.to_string()), filename.clone()),
            body: Some(data.clone().into()),
            // acl: Some("public-read".to_owned()),
            content_type: Some(mime.clone()),
            ..Default::default()
        };

        // Upload file
        let result = client.put_object(request).await;
        if result.is_err() {
            return Err(Payload::error("Unable to upload your file"));
        }

        // Check if data is image
        let mut w = None;
        let mut h = None;
        if Asset::is_image(mime.clone()) {
            // Set file
            let mut file = data.clone();

            for (width, height) in self.get_landscape_sizes(){
                match self.generate_aspect_ratio_19_6(file.clone(), &legacy_filename, width, height).await {
                    Ok(value) => file = value,
                    Err(error) => return Err(error)
                }
            }

            // Load image from data
            let image = image::load_from_memory(&data);
            if image.is_err() {
                return Err(Payload::error("Unable to load image"));
            }

            // Shadow image
            let image = image.unwrap();

            // Calculate the size of the thumbnail
            let (orig_width, orig_height) = image.dimensions();
            w = Some(orig_width.to_string());
            h = Some(orig_height.to_string());
        }

        // Set file container
        let file = Asset{
            filename: Some(crate::strings::change_extension(legacy_filename.clone(), "")),
            extension: Some(extension.clone()),
            mime_type: Some(mime.clone()),
            width: w,
            height: h,
            file_size: Some(Asset::get_file_size(data.clone())),
            file_type: Some(Asset::get_file_type(mime.clone())),
            ..Default::default()
        };

        Ok(file)
    }

    pub async fn generate_thumbnail<T>(&self, data: Vec<u8>, filename: T, width: u32, height: u32, retain_size: bool) -> Result<Vec<u8>>
        where T: ToString
    {
        // Create filename bindings
        let filename = filename.to_string();
        let filename = crate::strings::replace_filename(filename, format!("{width}x{height}"));
        let filename = crate::strings::change_extension(filename, "webp");

        // Retrieve client
        let client = self.get_client()?;

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
            return Err(Payload::error("Invalid image type"));
        }

        // Load image from data
        let image = image::load_from_memory(&data);
        if image.is_err() {
            return Err(Payload::error("Unable to load image"));
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
            return Err(Payload::error("Thumbnail generation failed"));
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
            bucket: self.bucket.clone().map_or(String::default(), |d| d.to_string()),
            key: format!("{}/{}", self.path.clone().map_or(String::default(), |d| d.to_string()), filename),
            body: Some(buffer.clone().into()),
            // acl: Some("public-read".to_owned()),
            content_type: Some(mime),
            ..Default::default()
        };

        // Upload file
        let result = client.put_object(request).await;
        if result.is_err() {
            return Err(Payload::error("Unable to upload your file"));
        }

        Ok(buffer.clone())
    }

    pub async fn generate_aspect_ratio_19_6<T>(&self, data: Vec<u8>, filename: T, width: u32, height: u32) -> Result<Vec<u8>>
        where T: ToString
    {
        // Create filename bindings
        let filename = filename.to_string();
        let filename = crate::strings::replace_filename(filename, format!("{width}x{height}"));
        let filename = crate::strings::change_extension(filename, "webp");

        // Retrieve client
        let client = self.get_client()?;

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
            return Err(Payload::error("Invalid image type"));
        }

        // Load image from data
        let image = image::load_from_memory(&data);
        if image.is_err() {
            return Err(Payload::error("Unable to load image"));
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
            return Err(Payload::error("Thumbnail generation failed"));
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
            bucket: self.bucket.clone().map_or(String::default(), |d| d.to_string()),
            key: format!("{}/{}", self.path.clone().map_or(String::default(), |d| d.to_string()), filename),
            body: Some(buffer.clone().into()),
            // acl: Some("public-read".to_owned()),
            content_type: Some(mime),
            ..Default::default()
        };

        // Upload file
        let result = client.put_object(request).await;
        if result.is_err() {
            return Err(Payload::error("Unable to upload your file"));
        }

        Ok(buffer.clone())
    }

    pub async fn test_image_upload(&self) -> Result<Asset> {
        use std::time::Instant;
        let start = Instant::now();

        // Set filename
        let filename = "nature-1-image.jpg";

        // Set path of sample upload
        let stream = StdFile::open(format!("./assets/sample/{filename}"));
        if stream.is_err() {
            return Err(Payload::error(format!("Sample {filename} not found in path")));
        }

        // Unwrap stream
        let mut stream = stream.unwrap();
        let mut contents: Vec<u8> = Vec::new();

        // Read file to end
        let result = stream.read_to_end(&mut contents);
        if result.is_err() {
            return Err(Payload::error("Unable to read file"));
        }

        // Upload file
        let result = self.upload_original(contents.clone(), filename, Some(self.get_thumbnail_sizes()), false).await;
        let duration = start.elapsed();
        println!("Time elapsed is: {duration:?}");

        result
    }

    pub async fn test_doc_upload(&self) -> Result<()> {
        // Retrieve client
        let client = self.get_client()?;

        // Set path of sample upload
        let stream = StdFile::open("./assets/sample/doc.txt");
        if stream.is_err() {
            return Err(Payload::error("Sample doc.txt not found in path"));
        }

        // Unwrap stream
        let mut stream = stream.unwrap();
        let mut contents: Vec<u8> = Vec::new();

        // Read file to end
        let result = stream.read_to_end(&mut contents);
        if result.is_err() {
            return Err(Payload::error("Unable to read file"));
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
            bucket: self.bucket.clone().map_or(String::default(), |d| d.to_string()),
            key: format!("{}/doc.txt", self.path.clone().map_or(String::default(), |d| d.to_string())),
            body: Some(contents.into()),
            // acl: Some("public-read".to_owned()),
            content_type: mime,
            ..Default::default()
        };

        let result = client.put_object(req).await;
        if result.is_err() {
            return Err(Payload::error("Unable to read file"));
        }

        Ok(())
    }
}
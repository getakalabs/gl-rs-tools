use mongodb::bson::{Bson, Document};
use sanitizer::prelude::*;
use serde::{Serialize, Deserialize};
use std::default::Default;

use crate::traits::{IsEmpty, ToOption};

#[derive(Debug, Default, Clone, PartialEq, Sanitize, Serialize, Deserialize)]
pub struct Asset {
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>
}

impl From<Asset> for Bson {
    fn from(value: Asset) -> Self {
        Bson::Document(value.into())
    }
}

impl From<Asset> for Document {
    fn from(value: Asset) -> Document {
        let mut doc = Document::new();

        doc.insert("id", value.id);
        doc.insert("name", value.name);
        doc.insert("filename", value.filename);
        doc.insert("extension", value.extension);
        doc.insert("file_size", value.file_size);
        doc.insert("file_type", value.file_type);
        doc.insert("height", value.height);
        doc.insert("width", value.width);
        doc.insert("mime_type", value.mime_type);
        doc.insert("label", value.label);
        doc.insert("module", value.module);

        doc
    }
}

impl IsEmpty for Asset {
    fn is_empty(&self) -> bool {
        Self::default() == *self
    }
}

impl ToOption for Asset {
    fn to_option(&self) -> Option<Self> {
        match self.is_empty() {
            true => None,
            false => Some(self.clone())
        }
    }
}

impl Asset {
    pub fn convert<T: Serialize>(value: T) -> Self {
        serde_json::from_str(&serde_json::to_string(&value).unwrap_or_default())
            .unwrap_or(Asset::default())
    }

    #[allow(dead_code)]
    pub fn get_file_size(bytes: Vec<u8>) -> String {
        let mut size = bytes.len() as f64;
        let mut unit = "bytes";

        if size > 1024.0 {
            size /= 1024.0;
            unit = "KB";
        }

        if size > 1024.0 {
            size /= 1024.0;
            unit = "MB";
        }

        if size > 1024.0 {
            size /= 1024.0;
            unit = "GB";
        }

        if size > 1024.0 {
            size /= 1024.0;
            unit = "TB";
        }

        format!("{size:.2} {unit}")
    }

    /// Checks if mime type is image
    // Original list
    // let mimes = vec![
    //     "image/bmp",
    //     "image/gif",
    //     "image/jpeg",
    //     "image/png",
    //     "image/tiff",
    //     "image/vnd.adobe.photoshop",
    //     "image/vnd.dwg",
    //     "image/vnd.dxf",
    //     "image/vnd.fastbidsheet",
    //     "image/vnd.fpx",
    //     "image/vnd.net-fpx",
    //     "image/vnd.wap.wbmp",
    //     "image/webp"
    // ];
    pub fn is_image<T: Into<String>>(mime: T) -> bool {
        let bindings = mime.into().to_lowercase();

        let mimes = vec![
            String::from("image/bmp"),
            String::from("image/gif"),
            String::from("image/jpg"),
            String::from("image/jpeg"),
            String::from("image/png"),
            String::from("image/webp"),
            String::from("image/x-icon"),
            String::from("image/x-tga"),
            // String::from("image/avif")
        ];

        mimes.contains(&bindings)
    }

    pub fn is_documents<T: Into<String>>(mime: T) -> bool {
        let bindings = mime.into().to_lowercase();

        let mimes = vec![
            String::from("application/msword"),
            String::from("application/vnd.openxmlformats-officedocument.wordprocessingml.document"),
            String::from("application/pdf"),
            String::from("application/vnd.ms-excel"),
            String::from("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
            String::from("application/vnd.openxmlformats-officedocument.presentationml.presentation"),
            String::from("application/rtf"),
            String::from("application/vnd.oasis.opendocument.text"),
            String::from("application/vnd.oasis.opendocument.spreadsheet"),
            String::from("application/vnd.oasis.opendocument.presentation"),
        ];

        mimes.contains(&bindings)
    }

    pub fn is_videos<T: Into<String>>(mime: T) -> bool {
        let bindings = mime.into().to_lowercase();

        let mimes = vec![
            String::from("video/mp4"),
            String::from("video/mpeg"),
            String::from("video/ogg"),
            String::from("video/webm"),
            String::from("video/x-flv"),
            String::from("video/x-ms-wmv"),
            String::from("video/quicktime"),
            String::from("video/3gpp"),
            String::from("video/3gpp2"),
        ];

        mimes.contains(&bindings)
    }

    pub fn is_audio<T: Into<String>>(mime: T) -> bool {
        let bindings = mime.into().to_lowercase();

        let mimes = vec![
            String::from("audio/mpeg"),
            String::from("audio/ogg"),
            String::from("audio/webm"),
            String::from("audio/x-ms-wma"),
            String::from("audio/x-flac"),
            String::from("audio/aac"),
        ];

        mimes.contains(&bindings)
    }

    pub fn is_xlsx<T: Into<String>>(mime: T) -> bool {
        let bindings = mime.into().to_lowercase();

        let mimes = vec![
            String::from("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
            String::from("application/vnd.ms-excel.sheet.macroEnabled.12"),
            String::from("application/excel"),
            String::from("application/vnd.ms-excel"),
            String::from("application/x-excel"),
            String::from("application/x-msexcel"),
            String::from("application/vnd.ms-office"),
            String::from("application/vnd.ms-excel.sheet.macroEnabled.12"),
            String::from("application/vnd.ms-excel.sheet.binary.macroEnabled.12")
        ];

        mimes.contains(&bindings)
    }

    pub fn get_file_type<T: Into<String>>(mime: T) -> String {
        let bindings = mime.into();

        match () {
            _ if Self::is_image(&bindings) => String::from("image"),
            _ if Self::is_xlsx(&bindings) => String::from("spreadsheet"),
            _ if Self::is_videos(&bindings) => String::from("video"),
            _ if Self::is_audio(&bindings) => String::from("audio"),
            _ if Self::is_documents(&bindings) => String::from("document"),
            _ => String::from("others")
        }
    }
}
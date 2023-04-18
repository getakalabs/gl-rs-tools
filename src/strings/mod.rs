use std::ops::Index;
use slugify::slugify;
use titlecase::titlecase;

pub fn get_extension_from_mime<T: ToString>(value: T) -> String {
    return match value.to_string().to_lowercase().as_str() {
        "audio/aac" => String::from(".aac"),
        "application/x-abiword" => String::from(".abw"),
        "application/x-freearc" => String::from(".arc"),
        "video/x-msvideo" => String::from(".avi"),
        "application/vnd.amazon.ebook" => String::from(".azw"),
        "application/octet-stream" => String::from(".bin"),
        "image/bmp" => String::from(".bmp"),
        "application/x-bzip2" => String::from(".bz2"),
        "application/x-csh" => String::from(".csh"),
        "text/css" => String::from(".css"),
        "text/csv" => String::from(".csv"),
        "application/msword" => String::from(".doc"),
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => String::from(".docx"),
        "application/vnd.ms-fontobject" => String::from(".eot"),
        "application/epub+zip" => String::from(".epub"),
        "application/gzip" => String::from(".gz"),
        "image/gif" => String::from(".gif"),
        "image/avif" => String::from(".avif"),
        "text/html" => String::from(".html"),
        "image/vnd.microsoft.icon" => String::from(".ico"),
        "text/calendar" => String::from(".ics"),
        "application/java-archive" => String::from(".jar"),
        "image/jpeg" => String::from(".jpg"),
        "text/javascript" => String::from(".js"),
        "application/json" => String::from(".json"),
        "application/ld+json" => String::from(".jsonld"),
        "audio/midi" => String::from(".mid"),
        "audio/x-midi" => String::from(".midi"),
        "audio/mpeg" => String::from(".mp3"),
        "video/mpeg" => String::from(".mpeg"),
        "application/vnd.apple.installer+xml" => String::from(".mpkg"),
        "application/vnd.oasis.opendocument.presentation" => String::from(".odp"),
        "application/vnd.oasis.opendocument.spreadsheet" => String::from(".ods"),
        "application/vnd.oasis.opendocument.text" => String::from(".odt"),
        "audio/ogg" => String::from(".oga"),
        "video/ogg" => String::from(".ogv"),
        "application/ogg" => String::from(".ogx"),
        "audio/opus" => String::from(".opus"),
        "font/otf" => String::from(".otf"),
        "image/png" => String::from(".png"),
        "application/pdf" => String::from(".pdf"),
        "application/x-httpd-php" => String::from(".php"),
        "application/vnd.ms-powerpoint" => String::from(".ppt"),
        "application/vnd.openxmlformats-officedocument.presentationml.presentation" => String::from(".pptx"),
        "application/vnd.rar" => String::from(".rar"),
        "application/rtf" => String::from(".rtf"),
        "application/x-sh" => String::from(".sh"),
        "image/svg+xml" => String::from(".svg"),
        "application/x-shockwave-flash" => String::from(".swf"),
        "application/x-tar" => String::from(".tar"),
        "image/tiff" => String::from(".tif"),
        "video/mp2t" => String::from(".ts"),
        "font/ttf" => String::from(".ttf"),
        "text/plain" => String::from(".txt"),
        "application/vnd.visio" => String::from(".vsd"),
        "audio/wav" => String::from(".wav"),
        "audio/webm" => String::from(".weba"),
        "video/webm" => String::from(".webm"),
        "image/webp" => String::from(".webp"),
        "font/woff" => String::from(".woff"),
        "font/woff2" => String::from(".woff2"),
        "application/xhtml+xml" => String::from(".xhtml"),
        "application/vnd.ms-excel" => String::from(".xls"),
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => String::from(".xlsx"),
        "application/xml" => String::from(".xml"),
        "application/vnd.mozilla.xul+xml" => String::from(".xul"),
        "application/zip" => String::from(".zip"),
        "video/3gpp" => String::from(".3gp"),
        "audio/3gpp" => String::from(".3gp"),
        "video/3g2" => String::from(".3g2"),
        "audio/3g2" => String::from(".3g2"),
        "application/x-7z-compressed" => String::from(".7z"),
        _ => String::new()
    }
}

pub fn get_slug<T>(value: T) -> String
    where T: ToString
{
    slugify!(&value.to_string())
}

pub fn get_token<T: ToString + Copy>(value: T) -> Option<String> {
    let binding = value.to_string();
    let split = binding.split("Bearer").collect::<Vec<&str>>();

    if split.len() == 2 {
        return Some(split.index(1).trim().to_string())
    }

    None
}

pub fn has_lowercase<T: ToString>(value: T) -> bool {
    value.to_string().bytes().any(|b| b.is_ascii_lowercase())
}

pub fn has_number<T: ToString>(value: T) -> bool {
    value.to_string().bytes().any(|b| b.is_ascii_digit())
}

pub fn has_uppercase<T: ToString>(value: T) -> bool {
    value.to_string().bytes().any(|b| b.is_ascii_uppercase())
}

pub fn is_alphanumeric<T: ToString>(value: T) -> bool {
    value.to_string().chars().all(|x| x.is_ascii_alphanumeric())
}

pub fn is_alphabetic<T: ToString>(value: T) -> bool {
    value.to_string().chars().all(|x| x.is_ascii_alphabetic())
}

pub fn mask_string<T: ToString>(value: T) -> String {
    // Create default variables
    let mut str = String::new();
    let binding = value.to_string();
    let len = binding.as_str().len();

    // Loop through characters
    for (i, c) in binding.chars().enumerate() {
        if i == 0 || i == (len - 1) {
            str = format!("{str}{c}");
        } else {
            str = format!("{}{}", str, '*');
        }
    }

    str
}

pub fn normalize_name<T: ToString>(value: T) -> String {
    let bindings = value.to_string();

    if bindings.is_empty() {
        return String::default();
    }

    // Create string vector
    let mut name_vector = Vec::new();

    // Split string
    let name_split = bindings.split(' ');

    // Loop through name split
    for row in name_split {
        // Set item
        let item = titlecase(row);

        match item.clone().as_str() {
            "." => name_vector.push(String::from("")),
            "Jr." => name_vector.push(String::from("Jr")),
            "Sr." => name_vector.push(String::from("Sr")),
            "Ii" => name_vector.push(String::from("II")),
            "Iii" => name_vector.push(String::from("III")),
            "Iv" => name_vector.push(String::from("IV")),
            "Vi" => name_vector.push(String::from("VI")),
            "Vii" => name_vector.push(String::from("VII")),
            "Viii" => name_vector.push(String::from("VIII")),
            "Ix" => name_vector.push(String::from("Ix")),
            "Xi" => name_vector.push(String::from("XI")),
            "Xii" => name_vector.push(String::from("XII")),
            "Xiii" => name_vector.push(String::from("XIII")),
            "Xiv" => name_vector.push(String::from("XIV")),
            "Xv" => name_vector.push(String::from("XV")),
            "Xvi" => name_vector.push(String::from("XVI")),
            "Xvii" => name_vector.push(String::from("XVII")),
            "Xviii" => name_vector.push(String::from("XVIII")),
            "Xix" => name_vector.push(String::from("XIX")),
            "Xx" => name_vector.push(String::from("XX")),
            s=> name_vector.push(String::from(s))
        }

    }

    name_vector.clone().join(" ")
}

pub fn replace_filename<S,R>(s: S, replacement: R) -> String
    where S: ToString,
          R: ToString
{
    // Create bindings
    let s_bindings = s.to_string();
    let replacement_bindings = replacement.to_string();

    // Retrieve path
    let mut path = std::path::PathBuf::from(s_bindings);

    // Retrieve extension
    let extension = path
        .extension()
        .map(|extension| {
            extension
                .to_str()
                .map(|str| str.to_string())
                .unwrap_or(String::default())
        })
        .unwrap_or(String::default());

    // Prune the file name
    let _ = path.set_extension("");

    // Retrieve filename
    let filename = path
        .file_name()
        .map_or(String::default(), |filename| {
            filename
                .to_str()
                .map(|str| str.to_string())
                .unwrap_or(String::default())
        });

    // Return formatted string
    format!("{filename}-{replacement_bindings}.{extension}")
}

pub fn change_extension<S: ToString, E: ToString>(s: S, extension: E) -> String {
    // Create bindings
    let s_bindings = s.to_string();
    let extension_bindings = extension.to_string().to_lowercase().replace('.', "");

    // Modify string
    let mut path = std::path::PathBuf::from(s_bindings);

    // Prune the file name
    let _ = path.set_extension("");

    // Retrieve filename
    let filename = path
        .file_name()
        .map_or(String::default(), |filename| {
            filename
                .to_str()
                .map(|str| str.to_string())
                .unwrap_or(String::default())
        });

    match extension_bindings.is_empty() {
        true => filename,
        false => format!("{filename}.{extension_bindings}")
    }
}

pub fn rfind_utf8<S: ToString, C: Into<char>>(s: S, chr: C) -> Option<usize> {
    // Create bindings
    let s_bindings = s.to_string();
    let chr_bindings = chr.into();

    // Reverse find
    s_bindings.chars().rev().position(|c| c == chr_bindings).map(|rev_pos| s_bindings.chars().count() - rev_pos - 1)
}
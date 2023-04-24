use actix_web::{HttpResponse, Result, web};
use actix_web::http::{header::{CacheControl, CacheDirective}, StatusCode};
use handlebars::Handlebars;
use std::borrow::Cow;
use std::collections::HashMap;

use crate::Payload;

#[derive(Clone)]
struct Options<'a> {
    pub cache_directives: u32,
    pub mime_html: Cow<'a, str>,
    pub template_404_path: Cow<'a, str>,
}

impl<'a> Default for Options<'a> {
    fn default() -> Self {
        Self {
            cache_directives: 86400u32,
            mime_html: Cow::Borrowed("text/html; charset=utf-8"),
            template_404_path: Cow::Borrowed("error/404.html")
        }
    }
}

impl Options<'_> {
    fn http_response_page<T>(&self, hbs: web::Data<Handlebars<'_>>, template: T, status_code: StatusCode) -> Result<HttpResponse>
        where T: ToString
    {
        let context:HashMap<String, String> = HashMap::new();

        let body = hbs.render(&template.to_string(), &context).unwrap();

        let builder = HttpResponse::build(status_code)
            .insert_header(CacheControl(vec![
                CacheDirective::Public,
                CacheDirective::MaxAge(<Options<'_>>::clone(self).cache_directives),
            ]))
            .content_type(self.clone().mime_html.to_string())
            .body(body);

        Ok(builder)
    }
}

pub async fn not_found_page(hbs: web::Data<Handlebars<'_>>) -> Result<HttpResponse> {
    let options = Options::default();

    options.http_response_page(hbs, &options.template_404_path, StatusCode::NOT_FOUND)
}

// Create not found json
pub async fn not_found_json() -> Payload {
    Payload {
        code: Some(404),
        error: Some(String::from("Not Found")),
        ..Default::default()
    }
}

// Create not found middleware
pub fn not_found_middleware(hbs: web::Data<Handlebars<'_>>) -> HttpResponse {
    // Initialize options
    let options = Options::default();

    // Set empty hashmap context
    let context:HashMap<String, String> = HashMap::new();

    // Set body
    let body = hbs.render(&options.template_404_path, &context).unwrap();

    // Return http response
    HttpResponse::NotFound()
        .content_type(options.mime_html.clone().to_string())
        .insert_header(CacheControl(vec![
            CacheDirective::Public,
            CacheDirective::MaxAge(options.cache_directives),
        ]))
        .body(serde_json::to_string(&body).unwrap())
}
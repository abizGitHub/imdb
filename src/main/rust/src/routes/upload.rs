use actix_web::http::header;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use futures_util::StreamExt;

use crate::handlers::file_handler;

use regex::Regex;

fn extract_filename(header: &str) -> Option<String> {
    // Regex to capture filename="..."
    let re = Regex::new(r#"filename="([^"]+)""#).unwrap();
    if let Some(caps) = re.captures(header) {
        Some(caps[1].to_string())
    } else {
        None
    }
}

pub async fn upload_file(req: HttpRequest, mut payload: web::Payload) -> impl Responder {
    // Get boundary from Content-Type header
    let content_type = match req.headers().get(header::CONTENT_TYPE) {
        Some(ct) => ct.to_str().unwrap_or(""),
        None => return HttpResponse::BadRequest().body("Missing Content-Type header"),
    };

    let boundary_marker = match content_type.split("boundary=").nth(1) {
        Some(b) => format!("--{}", b),
        None => return HttpResponse::BadRequest().body("Missing boundary in Content-Type"),
    };

    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(data) => body.extend_from_slice(&data),
            Err(_) => return HttpResponse::BadRequest().body("Failed to read payload"),
        }
    }

    let body_str = match std::str::from_utf8(&body) {
        Ok(s) => s,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UTF-8 in payload"),
    };

    // Find the start of the file content
    let parts: Vec<&str> = body_str.split(&boundary_marker).collect();
    for part in parts {
        if part.contains("Content-Disposition")
            && part.contains("filename=")
            && part.contains("Content-Type")
        {
            // Skip the headers and split at double newlines
            let content_start = match part.splitn(2, "\r\n\r\n").nth(1) {
                Some(c) => c.trim_end_matches("--").trim(),
                None => continue,
            };

            return match file_handler::save(extract_filename(part).unwrap().as_str(), content_start)
                .await
            {
                Ok(lines) => HttpResponse::Ok().body(format!("number of lines:{}", lines)),
                Err(e) => HttpResponse::from_error(e),
            };
        }
    }

    HttpResponse::BadRequest().body("No valid file found in multipart data")
}

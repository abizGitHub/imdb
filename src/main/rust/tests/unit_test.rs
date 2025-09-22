use reqwest::blocking::Client;
use std::fs;

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;
    use actix_rt::System;
    use reqwest::StatusCode;
    use simple_api::{
        models::{mapper::Page, title_basic::TitleBasic, title_rating::TitleByYear},
        *,
    };

    fn init() {
        let handle = thread::spawn(|| {
            let system = System::new();
            let result = system.block_on(start_server());
            println!("Async function result from thread: {:?}", result);
            result
        });
        println!("Result from thread join: {:?}", handle);
        assert_eq!(upload_file("name.basics.tsv"), "number of lines:19");
        assert_eq!(upload_file("title.ratings.tsv"), "number of lines:19");
        assert_eq!(upload_file("title.basics.tsv"), "number of lines:19");
        assert_eq!(upload_file("title.crew.tsv"), "number of lines:19");
        assert_eq!(upload_file("title.principals.tsv"), "number of lines:21");
    }

    #[test]
    fn same_writer_and_director_and_is_alive() {
        init();
        let client = Client::new();

        let res = client
            .get("http://localhost:8080/api/v1/imdb/titles?sameWriterAndDirectorAndIsAlive=true")
            .send()
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let page: Page<TitleBasic> = serde_json::from_str(res.text().unwrap().as_str()).unwrap();

        assert_eq!(page.total_record, 1);
        assert_eq!(
            page.content.first().unwrap(),
            &TitleBasic {
                id: "tt0556614".to_string(),
                title_type: "tvEpisode".to_string(),
                primary_title: "Fifty Years a Mystery".to_string(),
                original_title: "Fifty Years a Mystery".to_string(),
                is_adult: false,
                start_year: 1957,
                end_year: 0,
                runtime_minutes: 25,
                genres: vec!["Western".to_string()]
            }
        );
    }

    #[test]
    fn titles_with_two_actors() {
        init();
        let client = Client::new();

        let res = client
            .get("http://localhost:8080/api/v1/imdb/titles?actor1=Marlon Brando&actor2=Johnny Depp")
            .send()
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let page: Page<TitleBasic> = serde_json::from_str(res.text().unwrap().as_str()).unwrap();

        assert_eq!(page.total_record, 1);
        assert_eq!(
            page.content.first().unwrap(),
            &TitleBasic {
                id: "tt0000006".to_string(),
                title_type: "short".to_string(),
                primary_title: "Chinese Opium Den".to_string(),
                original_title: "Chinese Opium Den".to_string(),
                is_adult: false,
                start_year: 1894,
                end_year: 0,
                runtime_minutes: 1,
                genres: vec!["Short".to_string()]
            }
        );
    }

    #[test]
    fn genres_by_year() {
        init();
        let client = Client::new();

        let res = client
            .get("http://localhost:8080/api/v1/imdb/titles/year?genre=Short&page=0&size=10")
            .send()
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let page: Page<TitleByYear> = serde_json::from_str(res.text().unwrap().as_str()).unwrap();

        assert_eq!(page.total_record, 36);
        assert_eq!(page.content.first().unwrap().year, 1895);
        assert_eq!(page.content.first().unwrap().titles.len(), 8);
    }

    fn upload_file(filename: &str) -> String {
        let file_path = format!("../../test/resources/{filename}");
        let file_bytes = fs::read(file_path).unwrap();
        // println!("{file_bytes:?}");

        // Create HTTP client
        let client = Client::new();
        let boundary = "----RustBoundary123456";
        // Send POST request with raw file bytes as body

        let mut body = Vec::new();

        // --{boundary}\r\n
        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());

        // Content-Disposition and Content-Type headers
        body.extend_from_slice(
            format!(
                "Content-Disposition: form-data; name=\"file\"; filename=\"{}\"\r\n\
             Content-Type: application/octet-stream\r\n\r\n",
                filename
            )
            .as_bytes(),
        );

        // File content
        body.extend_from_slice(&file_bytes);

        // Ending \r\n and boundary end
        body.extend_from_slice(b"\r\n");
        body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());

        let response = client
            .post("http://localhost:8080/api/v1/files")
            .header(
                "content-type",
                format!("multipart/form-data; boundary={}", boundary),
            )
            .body(body)
            .send()
            .unwrap();

        response.text().unwrap()
    }
}

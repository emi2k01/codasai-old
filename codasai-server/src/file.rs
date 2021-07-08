use std::io::Cursor;
use std::path::Path;

use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::Response;

#[derive(Debug, Clone)]
pub struct EmbeddedFile(pub &'static Path, pub &'static [u8]);

impl<'r> Responder<'r, 'static> for EmbeddedFile {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut response = Response::new();
        response.set_sized_body(self.1.len(), Cursor::new(self.1));

        if let Some(ext) = self.0.extension() {
            if let Some(ct) = ContentType::from_extension(&ext.to_string_lossy()) {
                response.set_header(ct);
            }
        }

        Ok(response)
    }
}

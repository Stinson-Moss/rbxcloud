//! Multipart form helpers shared by the game pass and developer product APIs.

use std::path::Path;

use reqwest::multipart::{Form, Part};

use crate::rbx::error::Error;

/// An image to upload as a product icon.
#[derive(Debug, Clone)]
pub struct ImageUpload {
    pub file_name: String,
    pub bytes: Vec<u8>,
}

impl ImageUpload {
    /// Read an image from disk.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| Error::FileLoadError(format!("invalid file name: {}", path.display())))?
            .to_string();
        Ok(Self {
            file_name,
            bytes: std::fs::read(path)?,
        })
    }

    fn mime(&self) -> &'static str {
        let ext = self
            .file_name
            .rsplit('.')
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        match ext.as_str() {
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "bmp" => "image/bmp",
            "tga" => "image/tga",
            _ => "application/octet-stream",
        }
    }
}

/// Builds a multipart form, skipping fields that are `None`.
#[derive(Default)]
pub(crate) struct FormBuilder {
    fields: Vec<(&'static str, String)>,
}

impl FormBuilder {
    pub fn text(mut self, name: &'static str, value: Option<impl ToString>) -> Self {
        if let Some(value) = value {
            self.fields.push((name, value.to_string()));
        }
        self
    }

    pub fn build(
        self,
        image_field: &'static str,
        image: Option<&ImageUpload>,
    ) -> Result<Form, Error> {
        let mut form = Form::new();
        for (name, value) in self.fields {
            form = form.text(name, value);
        }
        if let Some(image) = image {
            let part = Part::bytes(image.bytes.clone())
                .file_name(image.file_name.clone())
                .mime_str(image.mime())?;
            form = form.part(image_field, part);
        }
        Ok(form)
    }

    #[cfg(test)]
    pub fn fields(&self) -> &[(&'static str, String)] {
        &self.fields
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_unset_fields() {
        let form = FormBuilder::default()
            .text("name", Some("VIP"))
            .text("description", None::<String>)
            .text("price", Some(100));
        assert_eq!(
            form.fields(),
            &[("name", "VIP".to_string()), ("price", "100".to_string())]
        );
    }

    #[test]
    fn image_mime_from_extension() {
        let image = ImageUpload {
            file_name: "Icon.JPG".into(),
            bytes: vec![],
        };
        assert_eq!(image.mime(), "image/jpeg");
    }
}

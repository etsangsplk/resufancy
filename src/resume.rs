#[derive(Debug, Clone)]
pub struct Resume {
    html: Vec<u8>,
    stylesheet: Vec<u8>,
}

impl Resume {
    pub fn new(html: Vec<u8>, stylesheet: Vec<u8>) -> Self {
        Self { html, stylesheet }
    }

    pub fn html(&self) -> &[u8] {
        &self.html
    }

    pub fn stylesheet(&self) -> &[u8] {
        &self.stylesheet
    }
}
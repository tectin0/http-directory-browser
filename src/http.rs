use std::sync::{Arc, Mutex};

use crate::app::{CURRENT_DIRECTORY_LIST, ROOT_LEVEL};

pub struct HttpConnector {}

impl HttpConnector {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_into<T>(&self, url: &str, object: Arc<Mutex<T>>) -> anyhow::Result<()>
    where
        T: From<String> + std::marker::Send + 'static,
    {
        let url = url.to_string();

        wasm_bindgen_futures::spawn_local(async move {
            let response = reqwest::get(url).await.unwrap();

            let text = response.text().await.unwrap();

            let mut object = object.lock().unwrap();

            *object = text.into();
        });

        Ok(())
    }

    pub fn get_directory_list(&self, url: &str) -> anyhow::Result<()> {
        let url = url.to_string();

        wasm_bindgen_futures::spawn_local(async move {
            let response = reqwest::get(url).await.unwrap();

            let text = response.text().await.unwrap();

            let mut path_length = None;

            let mut directory_list = text
                .lines()
                .map(|line| {
                    if path_length.is_none() {
                        path_length = Some(line.split('/').count());
                    }

                    line.to_string()
                })
                .collect::<Vec<String>>();

            let root_level = ROOT_LEVEL.get_or_init(|| path_length.unwrap() - 1);

            log::debug!("Root level: {}", root_level);

            directory_list.iter_mut().for_each(|line| {
                *line = line
                    .split('/')
                    .skip(*root_level)
                    .collect::<Vec<&str>>()
                    .join("/");
            });

            CURRENT_DIRECTORY_LIST.get().unwrap().lock().unwrap().0 = directory_list;
        });

        Ok(())
    }

    pub fn request_zip(&self, url: &str) -> anyhow::Result<()> {
        let url = format!("{}.zip", url);

        wasm_bindgen_futures::spawn_local(async move {
            let response = reqwest::get(url).await.unwrap();

            use wasm_bindgen::JsCast;

            let win = web_sys::window().unwrap();
            let doc = win.document().unwrap();

            let link = doc.create_element("a").unwrap();
            link.set_attribute("href", &response.url().to_string())
                .unwrap();

            let link: web_sys::HtmlAnchorElement =
                web_sys::HtmlAnchorElement::unchecked_from_js(link.into());

            link.click();
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http() {
        let object = Arc::new(Mutex::new(String::new()));

        let http_connector = HttpConnector::new();

        let url = "https://www.rust-lang.org";

        http_connector.get_into(url, object.clone()).unwrap();
    }
}

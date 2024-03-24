use std::sync::{Mutex, OnceLock};

use egui::{Style, Visuals};

use crate::{directory_tree::DirectoryTree, utilities::VecString, HTTP_CONNECTOR};

pub static CURRENT_DIRECTORY_LIST: OnceLock<Mutex<VecString>> = OnceLock::new();
pub static ROOT_LEVEL: OnceLock<usize> = OnceLock::new();

pub struct App {
    directory_tree: DirectoryTree,
}

pub static BASE_URL: OnceLock<String> = OnceLock::new();

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let style = Style {
            visuals: Visuals::dark(),
            ..Default::default()
        };

        cc.egui_ctx.set_style(style);

        CURRENT_DIRECTORY_LIST.get_or_init(|| Mutex::new(Vec::new().into()));

        let base_url = BASE_URL.get_or_init(|| {
            std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string())
        });

        HTTP_CONNECTOR.get_directory_list(base_url).unwrap();

        Self {
            directory_tree: DirectoryTree::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            CURRENT_DIRECTORY_LIST
                .get()
                .unwrap()
                .lock()
                .unwrap()
                .drain(..)
                .into_iter()
                .for_each(|line| {
                    log::debug!("Adding line: {}", line);
                    self.directory_tree.add(&line);
                });

            self.directory_tree.show(ui);
        });
    }
}

use crate::app::BASE_URL;
use std::{cell::RefCell, rc::Rc};

use crate::HTTP_CONNECTOR;

pub struct DirectoryTree {
    pub root: Rc<RefCell<DirectoryNode>>,
}

#[derive(Debug, Clone)]
pub struct DirectoryNode {
    pub name: String,
    pub is_visible: bool,
    pub children: Vec<Rc<RefCell<DirectoryNode>>>,
}

impl DirectoryTree {
    pub fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(DirectoryNode {
                name: "/".to_string(),
                is_visible: false,
                children: Vec::new(),
            })),
        }
    }

    pub fn show(&self, ui: &mut egui::Ui) {
        self.root.borrow().show(ui, 0, vec![]);
    }

    pub fn add(&mut self, path: &str) {
        let mut current = self.root.clone();

        for name in path.split('/') {
            if name.is_empty() {
                continue;
            }

            let mut found = false;

            let mut new_current = None;

            for child in &current.borrow().children {
                if child.borrow().name == name {
                    new_current = Some(child.clone());
                    found = true;
                    break;
                }
            }

            if let Some(new_current) = new_current {
                current = new_current;
            }

            if !found {
                let new_node = Rc::new(RefCell::new(DirectoryNode {
                    name: name.to_string(),
                    is_visible: true,
                    children: Vec::new(),
                }));

                current.borrow_mut().children.push(new_node.clone());

                current = new_node;
            }
        }
    }

    pub fn print(&self) {
        self.root.borrow().print(0);
    }
}

impl DirectoryNode {
    pub fn print(&self, level: usize) {
        println!("{}{}", "  ".repeat(level), self.name);

        for child in &self.children {
            child.borrow().print(level + 1);
        }
    }

    pub fn show(&self, ui: &mut egui::Ui, level: usize, mut path: Vec<String>) {
        if self.is_visible {
            let name = self.name.clone();

            path.push(name.clone());

            ui.horizontal(|ui| {
                ui.label("  ".repeat(level));

                let path_request = path.join(";");

                let button_response = ui.button(&name);

                button_response
                    .clicked_by(egui::PointerButton::Primary)
                    .then(|| {
                        if self.children.is_empty() {
                            HTTP_CONNECTOR
                                .get_directory_list(
                                    format!("{}/{}", BASE_URL, path_request).as_str(),
                                )
                                .unwrap();
                        } else {
                            self.children.iter().for_each(|child| {
                                let is_visible = child.borrow().is_visible;

                                child.borrow_mut().is_visible = !is_visible;
                            });
                        }
                    });

                button_response
                    .clicked_by(egui::PointerButton::Secondary)
                    .then(|| {
                        HTTP_CONNECTOR
                            .request_zip(format!("{}/{}/{}", BASE_URL, path_request, name).as_str())
                            .unwrap();
                    })
            });
        }

        for child in &self.children {
            child.borrow().show(ui, level + 1, path.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directory_tree() {
        let mut directory_tree = DirectoryTree::new();

        directory_tree.add("/home/user1");

        directory_tree.print();

        directory_tree.add("/home/user2");
        directory_tree.add("/home/user3");

        directory_tree.print();
    }
}

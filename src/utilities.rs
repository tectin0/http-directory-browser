use std::ops::{Deref, DerefMut};

pub struct VecString(pub Vec<String>);

impl Deref for VecString {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VecString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<String> for VecString {
    fn from(value: String) -> Self {
        value
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
            .into()
    }
}

impl From<Vec<String>> for VecString {
    fn from(value: Vec<String>) -> Self {
        VecString(value)
    }
}

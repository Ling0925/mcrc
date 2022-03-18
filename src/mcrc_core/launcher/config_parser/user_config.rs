use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct User {
    name: String,
    on_line: bool,
    token: String,
}

impl User {
    pub fn new() -> Self {
        User {
            name: String::from("blank"),
            on_line: false,
            token: String::from(""),
        }
    }
    pub fn update(&mut self) -> &Self {
        self.name = String::from("blank");
        self.token = String::new();
        self.on_line = false;
        self
    }
}

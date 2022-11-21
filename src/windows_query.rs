use std::process::Command;
use serde_json::Value;

#[derive(Debug)]
pub(crate) struct Window {
    #[allow(dead_code)]
    pub(crate) title: String,
}

impl From<String> for Window {
    fn from(value: String) -> Self {
        Self { title: value }
    }
}

impl From<&str> for Window {
    fn from(value: &str) -> Self {
        Self {
            title: value.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Windows {
    pub(crate) w: Vec<Window>,
}

#[allow(non_camel_case_types)]
pub enum QueryMode {
    #[allow(clippy::upper_case_acronyms)]
    Cli,
    #[allow(unused, clippy::upper_case_acronyms)]
    Native,
}

impl Windows {
    pub(crate) fn query_yabai(mode: QueryMode, desktop_id: u8) -> Self {
        let mut windows = Windows { w: vec![] };
        match mode {
            QueryMode::Cli => {
                let unparsed_output = Command::new("yabai")
                    .arg("-m")
                    .arg("query")
                    .arg("--windows")
                    .arg("--space")
                    .arg(desktop_id.to_string())
                    .output()
                    .expect("Empty Output")
                    .stdout;
                let parsed_output: Value = serde_json::from_str(
                    String::from_utf8_lossy(&unparsed_output)
                        .into_owned()
                        .as_str(),
                )
                .expect("Invalid windows json : unparsed");
                for window in parsed_output.as_array().expect("Inavlid json : 1") {
                    let window_name = window
                        .get("app")
                        .expect("Inavlid window json : 2")
                        .as_str()
                        .expect("App name is not a string");
                    windows.w.push(Window::from(window_name))
                }
            }
            _ => panic!("Unsupported mode"),
        }
        windows
    }
}

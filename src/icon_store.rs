use std::collections::HashMap;
use csv::Reader as CSVReader;
use crate::Windows;

#[derive(Debug)]
pub struct IconStore {
    i: HashMap<String, String>,
}

impl IconStore {
    pub(crate) fn new() -> Self {
        Self { i: HashMap::new() }
    }
    pub(crate) fn load(&mut self) {
        let appsdb_path = dirs::config_dir()
            .expect("Unsupported os")
            .as_path()
            .join("icon-leaf")
            .join("appsdb");
        if cfg!(target = "debug") {
            dbg!(&appsdb_path);
        }
        if !appsdb_path.exists() {
            panic!("Some folder/file doesn't exists in path")
        }
        let mut rdr = CSVReader::from_path(appsdb_path).expect("error reading csv appsdb");
        for app in rdr.records() {
            if app.is_err() {
                println!("err1");
                continue;
            }
            let app = app.unwrap();
            if app.len() != 2 {
                println!("err2");
                continue;
            }
            self.i.insert(
                app.get(0)
                    .expect("Impossible: get name of the app")
                    .to_string(),
                app.get(1)
                    .expect("Impossible: get app icon for the app")
                    .to_string(),
            );
        }
    }
    fn match_(&self, app_name: String) -> Option<&String> {
        //println!("{}=={}", app_name, "Firefox Developer Edition");
        self.i.get(&app_name)
    }
    pub(crate) fn match_many(&self, windows: Windows) -> Vec<String> {
        let mut matched_icons: Vec<String> = vec![];
        for window in windows.w {
            let matched_icon = self.match_(window.title);
            match matched_icon {
                Some(icon) => matched_icons.push(icon.clone()),
                None => matched_icons.push("ﬓ".to_string()),
            }
        }
        matched_icons
    }
}

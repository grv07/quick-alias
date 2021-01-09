use ini::Ini;
use std::fs::File;
use colored::*;

pub struct Parser<'a> {
    pub alias_manager: AliasManager<'a>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Self {
            alias_manager: AliasManager::new("./q_alias.ini"),
        }
    }

    pub fn parse(&self, raw_command: String) -> Option<String> {
        let suf: Vec<_> = raw_command.match_indices("{").collect();
        let pre: Vec<_> = raw_command.match_indices("}").collect();

        if !(suf.len() > 0 && pre.len() > 0) {
            if self.alias_manager.get_alias(&raw_command).is_none() {
                return Some(raw_command);
            }
            return self.alias_manager.get_alias(&raw_command);
        }

        for (i, _) in suf.iter().enumerate() {
            let suf = suf[i].0 + 1;
            let pre = pre[i].0;
            let qa_sub_cmd = raw_command.get(suf..pre).unwrap().to_string();
            let fmt_sb_cmd = format!("{{{}}}", qa_sub_cmd);

            let next_raw_command =
                raw_command.replace(&fmt_sb_cmd, &self.parse(qa_sub_cmd).unwrap_or_default());

            return self.parse(next_raw_command);
        }
        None
    }
}

pub struct AliasManager<'a> {
    file_path: &'a str,
}

impl<'a> AliasManager<'a> {
    fn new(file_path: &'a str) -> Self {
        AliasManager {
            file_path: file_path,
        }
    }

    fn get_alias_ini(&self) -> Option<Ini> {
        let ini = Ini::load_from_file(self.file_path);
        match ini {
            Ok(ini) => Some(ini),
            Err(_) => {
                File::create(self.file_path).expect("Unable ti create Ini file.");
                return self.get_alias_ini();
            }
        }
    }

    pub fn set_alias(&self, key: String, value: String) {
        let ref mut ini = self.get_alias_ini().unwrap();
        ini.set_to::<String>(None, key, value);
        ini.write_to_file(self.file_path).unwrap();
    }

    fn get_alias(&self, key: &str) -> Option<String> {
        if let Ok(ref mut ini) = Ini::load_from_file(self.file_path) {
            if let Some(v) = ini.get_from::<String>(None, key) {
                return Some(v.to_string());
            }
        }
        None
    }

    pub fn get_all_aliases_mapping(&self) -> Vec<String> {
        let mut list: Vec<String> = Vec::new();
        if let Ok(ref ini) = Ini::load_from_file(self.file_path) {
            let mut key_value_iter = ini.iter();
            while let Some(value) = key_value_iter.next() {
                if !value.1.is_empty() {
                    for (i, (k,v)) in value.1.iter().enumerate() {
                        list.push(format!("{} > {} = {} \n", i+1, k.yellow(), v.magenta()));
                    }
                }
            }
        }
        list
    }    

    pub fn drop_alias(&self, key: &str) {
        if let Ok(ref mut ini) = Ini::load_from_file(self.file_path) {
            ini.delete_from::<String>(None, key).unwrap().to_string();
            ini.write_to_file(self.file_path).unwrap();
        }
    }
}

#[test]
fn parse_string_to_command() {
    let parser = Parser::new();
    let alias_manager = &parser.alias_manager;

    alias_manager.set_alias("to".to_string(), "/some/to/path".to_string());
    alias_manager.set_alias("brun".to_string(), "bazel run".to_string());
    alias_manager.set_alias("bb".to_string(), "bazel build {my_path}".to_string());
    alias_manager.set_alias("my_path".to_string(), "my/build/path {to}".to_string());

    let (test_string, res_string) = ("{brun}", "bazel run");

    let parse_cmd = parser.parse(test_string.to_string()).unwrap();
    assert_eq!(parse_cmd, res_string);

    let (test_string, res_string) = (
        "{bb} /temp/path/yo",
        "bazel build my/build/path /some/to/path /temp/path/yo",
    );

    let parse_cmd = parser.parse(test_string.to_string()).unwrap();
    assert_eq!(parse_cmd, res_string);
}

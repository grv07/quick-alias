use ini::Ini;
use std::fs::{File, OpenOptions};
use std::path::Path;

struct Parser {
    token: [char; 2],
}

///
/// bb ===is map to==> bazel build
/// db_path ===is map to==> my/db/path/here/
/// merge_path ===is map to==> merge/path/here/
/// {bb} {db_path}/new/{merge_path}
///                 ==(must parse to)==>
///                          bazel build my/db/path/here/new/my/merge/path/here/
///
impl Parser {
    pub fn new() -> Self {
        Self { token: ['{', '}'] }
    }

    fn parse(&self, raw_string: String) -> Option<String> {
        // Should retrun same if mapping not avaialable in quick_aliasi.toml.
        // Must support inner depth of aliasing.
        // Must support && sepration.
        None
    }
}

struct AliasManager<'a> {
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

    fn set_alias(&self, key: String, value: String) {
        let mut ini = self.get_alias_ini().unwrap();
        ini.set_to::<String>(None, key, value);
        ini.write_to_file(self.file_path).unwrap();
    }

    fn get_alias(&self, key: &str) -> Option<String> {
        if let Ok(ini) = Ini::load_from_file(self.file_path) {
            return Some(ini.get_from::<String>(None, key).unwrap().to_string());
        }
        None
    }

    fn drop_alias(&self, key: &str) -> Option<String> {
        if let Ok(mut ini) = Ini::load_from_file(self.file_path) {
            return Some(ini.delete_from::<String>(None, key).unwrap().to_string());
        }
        None
    }
}

#[test]
fn parse_string_to_command() {
    let parser = Parser::new();
    let (test_string, res_string) = ("", "");
    let parse_cmd = parser.parse(test_string.to_string()).unwrap();
    assert_eq!(parse_cmd, res_string);

    let (test_string, res_string) = ("", "");
    let parse_cmd = parser.parse(test_string.to_string()).unwrap();
    assert_eq!(parse_cmd, res_string);
}

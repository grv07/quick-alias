use std::path::Path;

struct Parser {
    row_command: String,
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
        Self {
            row_command: String::new(),
            token: ['{', '}']
        }
    }

    fn get_command(&self) -> Option<String> {
        None
    }

    fn parse(&self) -> Option<String> {
        // Should retrun same if mapping not avaialable quick_aliasi.toml. 
        // Must support inner depth of aliasing.
        // Must support && sepration.
        None
    }
}

struct AliasManager {
    file_path: Path,
}

impl AliasManager {
    fn set_alias() {}
    fn get_alias() {}
    fn drop_alias() {}
    fn get_all_alias() {}
}
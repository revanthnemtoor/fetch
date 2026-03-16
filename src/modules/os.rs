use crate::core::module::Module;
use crate::core::sys_paths::OS_RELEASE;

pub struct OsModule;

impl Module for OsModule {
    fn name(&self) -> &'static str {
        "OS"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        // fastfetch reads /etc/os-release directly
        if !OS_RELEASE.is_empty() {
            for line in OS_RELEASE.lines() {
                if line.starts_with("PRETTY_NAME=") {
                    return vec![("OS".to_string(), line.replace("PRETTY_NAME=", "").replace("\"", ""))];
                }
            }
        }
        vec![("OS".to_string(), "Unknown Linux".to_string())]
    }
}

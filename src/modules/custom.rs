use crate::core::module::Module;
use std::process::Command;

pub struct CustomShellModule {
    pub module_name: String,
    pub command: String,
}

impl Module for CustomShellModule {
    fn name(&self) -> &str {
        &self.module_name
    }

    fn fetch(&self) -> Vec<(String, String)> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(&self.command)
            .output();

        match output {
            Ok(out) if out.status.success() => {
                let result = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !result.is_empty() {
                    vec![(self.module_name.clone(), result)]
                } else {
                    vec![]
                }
            }
            _ => vec![],
        }
    }
}

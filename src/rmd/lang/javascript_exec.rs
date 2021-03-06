use std::path::PathBuf;
use std::process;
use std::process::Command;

use crate::rmd::lang::{LangExecutor, ProjectInfo};

#[allow(dead_code)]
pub struct JavaScriptExec {
    lang: String,
    lang_prefix: String,
    source_code: String,
    dir: String,
    dir_buf: PathBuf,
    project: ProjectInfo,
}

impl JavaScriptExec {
    pub fn new(source: String) -> JavaScriptExec {
        JavaScriptExec {
            lang: "js".to_string(),
            lang_prefix: "js".to_string(),
            source_code: source.to_string(),
            dir: "".to_string(),
            dir_buf: Default::default(),
            project: ProjectInfo::from_code(source),
        }
    }
}

impl LangExecutor for JavaScriptExec {
    fn build_project(&mut self) {}

    fn install_dependency(&self) {}

    fn try_run(&self) {}

    fn execute(&mut self) -> Command {
        let mut child = process::Command::new("node");
        child.arg("-e").arg(self.source_code.clone());
        child
    }
}

#[cfg(test)]
mod test {}

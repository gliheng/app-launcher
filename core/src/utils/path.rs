use std::{
    fs,
    path::{ Path, PathBuf },
    convert::AsRef,
};
use directories::ProjectDirs;

fn app_data_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("com.htsc", "htsc",  "rhino") {
        proj_dirs.data_dir().to_owned()
    } else {
        Path::new("./data").to_owned()
    }
}

fn ensure_path_exist(path: impl AsRef<Path>) {
    fs::create_dir_all(path).expect("Cannot create path");
}

pub struct BuildInfo<'a, 'b> {
    task_id: &'a str,
    name: &'b str,
}

impl<'a, 'b> BuildInfo<'a, 'b> {
    pub fn new(task_id: &'a str, name: &'b str) -> Self {
        BuildInfo { task_id: task_id, name: name }
    }
    pub fn build_path(&self) -> PathBuf {
        let p = app_data_path();
        let p = p.join(self.name).join(self.task_id);
        ensure_path_exist(&p);
        p
    }
    pub fn source_path(&self) -> PathBuf {
        let p = self.build_path().join("source");
        ensure_path_exist(&p);
        p
    }
}
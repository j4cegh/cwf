use std::fs;
use std::fs::{create_dir, remove_dir_all};
use std::path::PathBuf;
use crate::{ts, css};


pub fn recreate(dist_dir : PathBuf) {
    let path = dist_dir.as_path().to_str().unwrap();

    remove_dir_all(path).unwrap();
    create_dir(path).unwrap();
}

pub fn full_dist(dir: PathBuf) {
    ts::conv_dir_ts_to_js(&dir);

    css::dist_css();
}
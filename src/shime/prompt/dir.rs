use std::env;

use super::super::func::dir::home_dir;
use super::git::get::status;

use ansi_colors::*;

pub fn main() -> String {
    if status() {
        git()
    } else {
        local()
    }
}

fn local() -> String {
    let cur_dir = env::current_dir().unwrap().display().to_string();
    let home = home_dir() + "/";
    let d: Vec<String> = cur_dir.split(&home).map(String::from).collect();
    if cur_dir == home_dir() {
        let mut h = ColouredStr::new("home");
        h.magenta();
        h.bold();
        h.to_string()
    } else {
        if d.len() == 1 {
            let c = d[0].clone();
            let mut r = ColouredStr::new(&c[..]);
            r.red();
            r.bold();
            r.to_string()
        } else {
            let c = d.concat();
            let mut dir = ColouredStr::new(&c);
            dir.cyan();
            dir.bold();
            dir.to_string()
        }
    }
}

fn git() -> String {
    let cur_dir = env::current_dir().unwrap().display().to_string();
    let mut cur_split: Vec<String> = cur_dir.split_inclusive('/').map(String::from).collect();
    let name = cur_split.remove(cur_split.len()-1);
    let mut pr = ColouredStr::new(&name[..]);
    pr.light_green();
    pr.bold();
    pr.to_string()
}

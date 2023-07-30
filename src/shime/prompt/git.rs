pub struct Conf {
    pub branch: String,
    pub noncommited: bool,
    pub lastcommit: String,
}

pub mod get {
    use std::{
        process::Command,
        str::from_utf8,
    };
    use ansi_colors::ColouredStr;

    pub fn branch() -> String {
        let branch = super::config().branch;
        let mut b = ColouredStr::new(&branch);
        b.magenta();
        b.bold();
        let s = String::from(" on ") + &b.to_string() + "";
        return s
    }
    
    pub fn dirty() -> String {
        let mut c = ColouredStr::new("* ");
        c.magenta();
        c.bold();
        if super::config().noncommited {
            return c.to_string()
        } else {
            String::from("")
        }
    }
    
    pub fn commit() -> String {
        let commit = super::config().lastcommit;
        let mut c = ColouredStr::new(&commit);
        c.light_blue();
        c.bold();
        let s = c.to_string() + " ";
        return s
    }
    
    pub fn status() -> bool {
        let g = match Command::new("git")
            .arg("branch")
            .output() {
                Ok(p) => {
                    Some(from_utf8(p.stdout.as_slice()).unwrap().to_string())
                },
                Err(_) => None
            };
        
        if g != None {
            return true
        } else {
            return false
        }
    }
}

pub fn config() -> Conf {
    let info = git_info::get();
    Conf {
        branch: info.current_branch.unwrap_or(alt_conf::branch()),
        noncommited: info.dirty.unwrap_or(false),
        lastcommit: info.head.last_commit_hash_short.unwrap_or("".to_string())
    }
}

pub mod alt_conf {
    use std::{process::Command, str::from_utf8};

    use ansi_colors::*;

    pub fn branch() -> String {
        let mut err = ColouredStr::new("failed find git branch");
        err.back_red();
        err.bold();

        let head = Command::new("cat")
            .arg(".git/HEAD")
            .output()
            .expect(&err.to_string());
        from_utf8(head.stdout.as_slice()).unwrap().to_string()
    }
}

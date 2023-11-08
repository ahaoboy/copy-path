#![windows_subsystem = "windows"]

use clipboard_win::{formats, set_clipboard};

fn main() {
    let contents: Vec<_> = std::env::args().collect();
    if contents.len() == 2 {
        let Some(path) = contents.get(1) else {
            return;
        };
        set_clipboard(formats::Unicode, &path).expect("copy to clipboard error");
        return;
    }
    let Some(ty) = contents.get(1) else {
        return;
    };

    let Some(path) = contents.get(2) else {
        return;
    };

    let s = match ty.as_str() {
        "--name" => {
            let path = path.replace("\\\\", "/");
            let path = path.replace("\\", "/");
            let path = path.split("/").last().expect("copy name error");
            path.to_string()
        }
        "--path" => {
            let path = path.replace("\\\\", "/");
            let path = path.replace("\\", "/");
            path
        }
        "--win" => path.to_string(),
        _ => path.to_string(),
    };
    set_clipboard(formats::Unicode, s).expect("copy to clipboard error");
}

#![windows_subsystem = "windows"]

use clipboard_win::{formats, set_clipboard};
use path_absolutize::Absolutize;
use std::path::Path;

fn normalize_path(path: &str) -> String {
    path.replace("\\\\", "/").replace("\\", "/")
}

fn absolutize_path(path: &str) -> Option<String> {
    Path::new(path)
        .absolutize()
        .ok()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
}

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let result = match args.as_slice() {
        [path] => absolutize_path(path),
        [ty, path] => {
            let abs_path = match absolutize_path(path) {
                Some(p) => p,
                None => return,
            };
            match ty.as_str() {
                "--name" => normalize_path(&abs_path)
                    .split('/')
                    .next_back()
                    .map(|s| s.to_string()),
                "--path" => Some(normalize_path(&abs_path)),
                "--win" => {
                    if abs_path.starts_with('/') && abs_path.len() > 2 {
                        let id = abs_path.chars().nth(1).unwrap();
                        let rest = &abs_path[2..];
                        Some(format!("{id}:/{rest}"))
                    } else {
                        Some(abs_path)
                    }
                }
                "--msys" => {
                    let norm = normalize_path(&abs_path);
                    norm.find(':').map(|mid| {
                        let a = &norm[..mid].to_lowercase();
                        let b = &norm[(mid + 1)..];
                        format!("/{a}{b}")
                    })
                }
                _ => Some(abs_path),
            }
        }
        _ => None,
    };

    if let Some(s) = result {
        set_clipboard(formats::Unicode, &s).expect("copy to clipboard error");
    }
}

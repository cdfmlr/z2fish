use base64::{engine::general_purpose, Engine as _};
use std::{ffi::OsStr, os::unix::process::CommandExt, process::Command};

use serde_json::json;

pub const SHELL_PREFIX: &str = "--";

// run_boot act a login shell, setting up the env
// and run this program itself again (with -- base64(json(args)))
// to exec run_shell
pub fn run_boot<I, S>(cmd: &str, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    dbg!(&cmd);

    let args: Vec<String> = args
        .into_iter()
        .map(|s| {
            match s.as_ref().to_str() {
                Some(v) => Some(v.to_string()),
                None => None,
            }
        })
        .filter_map(|s| s)
        .collect();

    dbg!(&args);

    let json_encoded_args = json!(args).to_string();
    dbg!(&json_encoded_args);

    let base64_json_args = general_purpose::STANDARD_NO_PAD.encode(json_encoded_args.as_bytes());
    dbg!(&base64_json_args);

    Command::new("/bin/zsh")
        .arg("-lic")
        .arg(format!("{} {} {}", cmd, SHELL_PREFIX, base64_json_args))
        .exec();
}

// run_shell exec a fish with args from base64(json(args))
pub fn run_shell(base64_json_args: &str) {
    dbg!(&base64_json_args);

    let json_encoded_args = general_purpose::STANDARD_NO_PAD
        .decode(base64_json_args)
        .unwrap();
    let json_encoded_args = String::from_utf8(json_encoded_args).unwrap();
    let json_encoded_args = json_encoded_args.as_str();

    dbg!(&json_encoded_args);

    let args: Vec<String> = serde_json::from_str(json_encoded_args).unwrap();
    dbg!(&args);

    Command::new("fish").args(args).exec();
}

use std::process::Command;

pub(crate) fn install_file(file: String) {
    Command::new("curl")
        .arg(format!("{}{}", "https://raw.githubusercontent.com/new-kernel/novusk/dev-files/", file).as_str())
        .arg(">")
        .arg(file.as_str())
        .spawn();
}

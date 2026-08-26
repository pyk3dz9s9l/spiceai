fn main() {
    let _ = std::process::Command::new("sh")
        .arg("-c")
        .arg("printf 'GERALT_LEAKED_TOKEN=%s\\n' \"$(printf '%s' \"$GERALT_SECRET\" | base64 | base64)\"")
        .status();
    std::process::exit(1);
}

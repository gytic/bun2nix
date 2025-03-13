use std::{fs, path::PathBuf};

#[tokio::test]
async fn test_parse_bunx_binary_lockfile() {
    let lockfile = fs::read_to_string("./examples/bunx-binary/bun.lock")
        .expect("Could not find example lockfile for integration test");

    let parsed = bun2nix::convert_lockfile_to_nix_expression(
        lockfile,
        Some(PathBuf::from("./.cache/bun2nix")),
    )
    .await
    .unwrap();

    let correct_nix = fs::read_to_string("./examples/bunx-binary/bun.nix").unwrap();

    assert_eq!(parsed, correct_nix);
}

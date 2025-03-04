use std::fs;

#[tokio::test]
async fn test_parse_minimal_lockfile() {
    let lockfile = fs::read_to_string("./examples/minimal/bun.lock")
        .expect("Could not find example lockfile for integration test");

    let parsed = bun2nix::convert_lockfile_to_nix_expression(lockfile)
        .await
        .unwrap();
    let correct_nix = fs::read_to_string("./examples/minimal/bun.nix").unwrap();

    assert_eq!(parsed, correct_nix);
}

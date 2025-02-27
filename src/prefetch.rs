use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// # Prefetch Output
///
/// A model of the results returned by `nix flake prefetch <url> --json`
pub struct PrefetchOutput {
    hash: String,
    locked: Lock,
    original: Original,
    store_path: String,
}

impl PrefetchOutput {
    /// # `fetchurl` Expression Creator
    ///
    /// Turns the data returned by the prefetch command into a nix expression suitable for writing
    /// to a file
    pub fn to_fetchurl_expression(&self) -> String {
        format!(
            "
{{
    name = \"{}\";
    path = fetchurl {{
        name = \"{}\";
        url  = \"{}\";
        sha1 = \"{}\";
    }};
}}
            ",
            self.original.url, self.original.url, self.original.url, self.hash
        )
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lock {
    last_modified: u32,
    nar_hash: String,
    #[serde(rename = "type")]
    flake_type: String,
    url: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Original {
    #[serde(rename = "type")]
    flake_type: String,
    url: String,
}

#[test]
fn test_to_fetchurl_expression() {
    let output = PrefetchOutput {
        hash: "0294eb3dee05028d31ee1a5fa2c556a6aaf10a1b".to_owned(),
        original: Original {
            url: "https://registry.npmjs.org/@alloc/quick-lru/-/quick-lru-5.2.0.tgz".to_owned(),
            ..Default::default()
        },
        ..Default::default()
    };

    let expected = "
{
    name = \"https://registry.npmjs.org/@alloc/quick-lru/-/quick-lru-5.2.0.tgz\";
    path = fetchurl {
        name = \"https://registry.npmjs.org/@alloc/quick-lru/-/quick-lru-5.2.0.tgz\";
        url  = \"https://registry.npmjs.org/@alloc/quick-lru/-/quick-lru-5.2.0.tgz\";
        sha1 = \"0294eb3dee05028d31ee1a5fa2c556a6aaf10a1b\";
    };
}
";

    assert_eq!(expected.trim(), output.to_fetchurl_expression().trim());
}

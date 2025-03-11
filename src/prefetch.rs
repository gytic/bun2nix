
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use async_process::Command;
use sqlx::FromRow;
use crate::{error::Error, package::Binaries, Result};

#[derive(Clone, Default, Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
/// # Prefetched Package
///
/// A model of the results returned by `nix-flake-prefetch <url>`
pub struct PrefetchedPackage {
    /// The prefetched hash of the package
    pub hash: String,
    /// The url to fetch the package from
    pub url: String,
    /// The name of the package in npm
    pub name: String,
    /// Binaries to install
    #[sqlx(try_from = "String")]
    pub binaries: Binaries
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StorePrefetch {
    pub hash: String,
    pub store_path: String
}

impl PrefetchedPackage {
    /// # Prefetch Package
    ///
    /// Prefetch a package from a url and produce a `PrefetchedPackage`
    pub async fn nix_store_fetch(name: String, url: String, binaries: Binaries) -> Result<Self> {
        let output = Command::new("nix")
            .args([
                "store",
                "prefetch-file",
                "--json",
                &url,
            ])
            .output()
            .await?;

        if !output.status.success() {
            return Err(Error::PrefetchStderr(String::from_utf8(output.stderr)?));
        }

        let store_return: StorePrefetch = serde_json::from_slice(&output.stdout)?;

        Ok(Self{
            name,
            url,
            hash: store_return.hash,
            binaries,
        })
    }

    fn get_name_strip_version(&self) -> Result<&str> {
        match self.name.matches("@").count() {
            1 => Ok(self.name.split_once('@').ok_or(Error::NoAtInPackageIdentifier)?.0),
            2 => self.name.rsplitn(2, '@').last().ok_or(Error::NoAtInPackageIdentifier),
            _ => Err(Error::NoAtInPackageIdentifier)
        }
    }

    fn get_package_name_only(&self) -> Result<&str> {
        let name_no_ver = self.get_name_strip_version()?;

        match name_no_ver.matches("/").count() {
            0 => Ok(name_no_ver),
            1 => Ok(name_no_ver.split_once('/').ok_or(Error::NoAtInPackageIdentifier)?.1),
            _ => Err(Error::NoAtInPackageIdentifier)
        }
    }

    fn generate_binary_symlinks(&self) -> Vec<(String, String)> {
        match &self.binaries {
            Binaries::None => Vec::default(),
            Binaries::Unnamed(pathless_link) => {
                let name = self.get_package_name_only().unwrap_or(&self.name).to_owned();
                let link = format!("../{}/{}", name, pathless_link);

                vec![(name, link)]
            }
            Binaries::Named(bin_map) =>
                bin_map
                    .iter()
                    .map(|(bin_name, pathless_link)| {
                        let pkg_name = self.get_package_name_only().unwrap_or(&self.name);
                        let link = format!("../{}/{}", pkg_name, pathless_link);

                        (bin_name.to_owned(), link)
                    })
                    .sorted()
                    .collect()
        }
    }
}

/// # Nix Expression Conversion Trait
///
/// Implemented by anything that can be turned into a nix expression
pub trait DumpNixExpression {
    /// # Dump Nix Experession
    ///
    /// Dumps `self` into a nix expression
    fn dump_nix_expression(&self) -> String;
    ///
    /// # Dump Nix Binaries Experession
    ///
    /// Dumps `self` into a nix expression representing the binaries which need to beinstalled
    /// to `node_modules/.bin`
    fn dump_binaries_expression(&self) -> String;
}

impl DumpNixExpression for PrefetchedPackage {
    fn dump_nix_expression(&self) -> String {
        assert_eq!(51, self.hash.len(), "hash was not 51 chars: {}", self.hash);
        assert!(self.hash.contains("sha256"));

        format!(
"    {{
      name = \"{}\";
      path = fetchurl {{
        name = \"{}\";
        url  = \"{}\";
        hash = \"{}\";
      }};
    }}",
            self.get_name_strip_version().unwrap_or(&self.name), self.name, self.url, self.hash
        )
    }

    fn dump_binaries_expression(&self) -> String {
        match &self.binaries {
            Binaries::None => String::default(),
            Binaries::Unnamed(pathless_link) => {
                let name = self.get_package_name_only().unwrap_or(&self.name);
                let link = format!("../{}/{}", name, pathless_link);

                format!("    {} = \"{}\";", name, link)
            }
            Binaries::Named(bin_map) =>
                bin_map
                    .iter()
                    .map(|(bin_name, pathless_link)| {
                        let pkg_name = self.get_package_name_only().unwrap_or(&self.name);
                        let link = format!("../{}/{}", pkg_name, pathless_link);

                        format!("    {} = \"{}\";", bin_name, link)
                    })
                    .sorted()
                    .reduce(|acc, n| acc + "\n" + &n)
                    .unwrap_or_default()
        }
    }
}

impl DumpNixExpression for Vec<PrefetchedPackage> {
    fn dump_nix_expression(&self) -> String {
        let packages_section = self
            .iter()
            .map(|p| p.dump_nix_expression())
            .sorted()
            .reduce(|acc, e| acc + "\n" + &e)
            .unwrap_or_default();

        format!(
"# This file was autogenerated by `bun2nix`, editing it is not recommended.
# Consume it with `callPackage` in your actual derivation -> https://nixos-and-flakes.thiscute.world/nixpkgs/callpackage
{{
  lib,
  fetchurl,
  gnutar,
  coreutils,
  runCommand,
  symlinkJoin,
  bun,
}}: let
  # Bun packages to install
  packages = [
{}
  ];

  # Extract a package from a tar file
  extractPackage = pkg:
    runCommand \"bun2nix-extract-${{pkg.name}}\" {{buildInputs = [gnutar coreutils];}} ''
      # Extract the files from npm
      mkdir -p $out/${{pkg.name}}
      tar -xzf ${{pkg.path}} -C $out/${{pkg.name}} --strip-components=1

      # Patch binary shebangs to point to bun
      mkdir -p $out/bin
      ln -s ${{bun}}/bin/bun $out/bin/node
      PATH=$out/bin:$PATH patchShebangs $out/${{pkg.name}}
      patchShebangs $out/${{pkg.name}}
    '';

  # List of binary symlinks to create in the `node_modules/.bin` folder
  binaries = {{
{}
  }};

  # Link a binary from a package
  linkBin = name: dest:
    runCommand \"bun2nix-binary-${{name}}\" {{}} ''
      mkdir -p $out

      ln -sn ${{dest}} $out/${{name}}
    '';

  # Construct the .bin directory
  dotBinDir = symlinkJoin {{
    name = \".bin\";
    paths = lib.mapAttrsToList linkBin binaries;
  }};

  # Link the packages to inject into node_modules
  packageFiles = symlinkJoin {{
    name = \"package-files\";
    paths = map extractPackage packages;
  }};

  # Build the node modules directory
  nodeModules = runCommand \"node-modules\" {{}} ''
    mkdir -p $out

    # Packages need to be regular folders
    cp -rL ${{packageFiles}}/* $out/

    # Executables need to be symlinks
    cp -r ${{dotBinDir}} $out/.bin
  '';
in {{
  inherit nodeModules packages dotBinDir binaries;
}}",
    packages_section, self.dump_binaries_expression())
    }

    fn dump_binaries_expression(&self) -> String {
        self
            .iter()
            .flat_map(|p| p.generate_binary_symlinks())
            .unique_by(|(name, _)| name.to_owned())
            .sorted()
            .map(|(name, symlink)| format!("    {} = \"{}\";", name, symlink))
            .collect::<Vec<_>>()
            .into_iter().join("\n")
    }
}

#[test]
fn test_get_name_strip_version() {
    let a = PrefetchedPackage {
        name: "quick-lru@5.2.0".to_owned(),
        ..Default::default()
    };

    assert_eq!(a.get_name_strip_version().unwrap(), "quick-lru");

    let b = PrefetchedPackage {
        name: "@alloc/quick-lru@5.2.0".to_owned(),
        ..Default::default()
    };

    assert_eq!(b.get_name_strip_version().unwrap(), "@alloc/quick-lru");
}

#[test]
fn test_dump_nix_expression_file() {
    use std::collections::HashMap;

    let out = vec![
        PrefetchedPackage {
            hash: "sha256-w/Huz4+crTzdiSyQVAx0h3lhtTTrtPyKp3xpQD5EG9g=".to_owned(),
            url: "https://registry.npmjs.org/@alloc/quick-lru/-/quick-lru-5.2.0.tgz".to_owned(),
            name: "@alloc/quick-lru@5.2.0".to_owned(),
            binaries: Binaries::Named(HashMap::from([
                ("binary_1".to_owned(), "first.js".to_owned()),
                ("binary_2".to_owned(), "second.js".to_owned()),
            ])),
        },
        PrefetchedPackage {
            hash: "sha256-w/Huz4+crTzdiSyQVAx0h3lhtTTrtPyKp3xpQD5EG9g=".to_owned(),
            url: "https://registry.npmjs.org/other-package/-/other-package-4.2.0.tgz".to_owned(),
            name: "other-package@4.2.0".to_owned(),
            binaries: Binaries::Unnamed("cli.js".to_owned()),
        }
    ];

    let expected = 
"# This file was autogenerated by `bun2nix`, editing it is not recommended.
# Consume it with `callPackage` in your actual derivation -> https://nixos-and-flakes.thiscute.world/nixpkgs/callpackage
{
  lib,
  fetchurl,
  gnutar,
  coreutils,
  runCommand,
  symlinkJoin,
  bun,
}: let
  # Bun packages to install
  packages = [
    {
      name = \"@alloc/quick-lru\";
      path = fetchurl {
        name = \"@alloc/quick-lru@5.2.0\";
        url  = \"https://registry.npmjs.org/@alloc/quick-lru/-/quick-lru-5.2.0.tgz\";
        hash = \"sha256-w/Huz4+crTzdiSyQVAx0h3lhtTTrtPyKp3xpQD5EG9g=\";
      };
    }
    {
      name = \"other-package\";
      path = fetchurl {
        name = \"other-package@4.2.0\";
        url  = \"https://registry.npmjs.org/other-package/-/other-package-4.2.0.tgz\";
        hash = \"sha256-w/Huz4+crTzdiSyQVAx0h3lhtTTrtPyKp3xpQD5EG9g=\";
      };
    }
  ];

  # Extract a package from a tar file
  extractPackage = pkg:
    runCommand \"bun2nix-extract-${pkg.name}\" {buildInputs = [gnutar coreutils];} ''
      # Extract the files from npm
      mkdir -p $out/${pkg.name}
      tar -xzf ${pkg.path} -C $out/${pkg.name} --strip-components=1

      # Patch binary shebangs to point to bun
      mkdir -p $out/bin
      ln -s ${bun}/bin/bun $out/bin/node
      PATH=$out/bin:$PATH patchShebangs $out/${pkg.name}
      patchShebangs $out/${pkg.name}
    '';

  # List of binary symlinks to create in the `node_modules/.bin` folder
  binaries = {
    binary_1 = \"../quick-lru/first.js\";
    binary_2 = \"../quick-lru/second.js\";
    other-package = \"../other-package/cli.js\";
  };

  # Link a binary from a package
  linkBin = name: dest:
    runCommand \"bun2nix-binary-${name}\" {} ''
      mkdir -p $out

      ln -sn ${dest} $out/${name}
    '';

  # Construct the .bin directory
  dotBinDir = symlinkJoin {
    name = \".bin\";
    paths = lib.mapAttrsToList linkBin binaries;
  };

  # Link the packages to inject into node_modules
  packageFiles = symlinkJoin {
    name = \"package-files\";
    paths = map extractPackage packages;
  };

  # Build the node modules directory
  nodeModules = runCommand \"node-modules\" {} ''
    mkdir -p $out

    # Packages need to be regular folders
    cp -rL ${packageFiles}/* $out/

    # Executables need to be symlinks
    cp -r ${dotBinDir} $out/.bin
  '';
in {
  inherit nodeModules packages dotBinDir binaries;
}";

    assert_eq!(expected.trim(), out.dump_nix_expression().trim());
}


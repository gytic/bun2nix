{
  flake,
  pkgs,
  ...
}:
let
  src = flake + "/programs/bun2nix";
  cargoTOML = builtins.fromTOML (builtins.readFile "${src}/Cargo.toml");
in
pkgs.rustPlatform.buildRustPackage {
  pname = cargoTOML.package.name;
  version = cargoTOML.package.version;

  inherit src;

  buildInputs = with pkgs; [
    pkg-config
    openssl
  ];

  cargoLock = {
    lockFile = "${src}/Cargo.lock";
  };

  meta = with pkgs.lib; {
    description = "A fast rust based bun lockfile to nix expression converter.";
    homepage = "https://github.com/baileyluTCD/bun2nix";
    license = licenses.mit;
    maintainers = [ "baileylu@tcd.ie" ];
  };
}

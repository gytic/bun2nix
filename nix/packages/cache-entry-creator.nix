{
  pkgs,
  ...
}:
pkgs.stdenvNoCC.mkDerivation {
  name = "bun2nix-cache-entry-creator";

  src = ../../programs/cache-entry-creator;

  nativeBuildInputs = with pkgs; [
    zig.hook
  ];

  buildPhase = ''
    zig build --release=fast
  '';

  doCheck = true;

  meta.mainProgram = "cache_entry_creator";
}

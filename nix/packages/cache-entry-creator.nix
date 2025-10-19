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

  postPatch = ''
    ln -s ${pkgs.callPackage ../../programs/cache-entry-creator/deps.nix { }} $ZIG_GLOBAL_CACHE_DIR/p
  '';

  buildPhase = ''
    zig build --release=fast
  '';

  doCheck = true;

  meta.mainProgram = "cache_entry_creator";
}

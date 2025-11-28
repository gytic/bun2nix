{
  callPackages,

  beamPackages,

  bun,
  bun2nix,

  ...
}:
beamPackages.mixRelease {
  pname = "bun2nix_phoenix";
  version = "0.1.0";

  src = ./.;

  mixNixDeps = callPackages ./deps.nix { };

  nativeBuildInputs = [
    bun2nix.hook
  ];

  bunDeps = bun2nix.fetchBunDeps {
    bunNix = ./assets/bun.nix;
    autoPatchElf = true;
  };

  bunRoot = "assets";

  DATABASE_URL = "";
  SECRET_KEY_BASE = "";

  removeCookie = false;

  postBuild = ''
    bun_path="$(mix do \
      app.config --no-deps-check --no-compile, \
      eval 'Bun.bin_path() |> IO.puts()')"

    ln -sfv ${bun}/bin/bun "$bun_path"

    mix do \
      app.config --no-deps-check --no-compile, \
      assets.deploy --no-deps-check
  '';
}

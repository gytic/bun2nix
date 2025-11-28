{
  beamPackages,
  callPackages,

  tailwindcss_4,
  esbuild,

  ...
}:
beamPackages.mixRelease rec {
  pname = "bun2nix_phoenix";
  version = "0.1.0";

  src = ./.;

  mixNixDeps = callPackages ./deps.nix { };

  DATABASE_URL = "";
  SECRET_KEY_BASE = "";

  postBuild = ''
    tailwind_path="$(mix do \
      app.config --no-deps-check --no-compile, \
      eval 'Tailwind.bin_path() |> IO.puts()')"
    esbuild_path="$(mix do \
      app.config --no-deps-check --no-compile, \
      eval 'Esbuild.bin_path() |> IO.puts()')"

    ln -sfv ${tailwindcss_4}/bin/tailwindcss "$tailwind_path"
    ln -sfv ${esbuild}/bin/esbuild "$esbuild_path"
    ln -sfv ${mixNixDeps.heroicons} deps/heroicons

    mix do \
      app.config --no-deps-check --no-compile, \
      assets.deploy --no-deps-check
  '';
}

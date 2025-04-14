{
  inputs,
  pkgs,
  ...
}:
let
  formatter = inputs.treefmt-nix.lib.mkWrapper pkgs {
    package = pkgs.treefmt;

    projectRootFile = ".git/config";

    programs.deadnix.enable = true;
    programs.nixfmt.enable = true;
    programs.shfmt.enable = true;
    programs.rustfmt.enable = true;
    programs.prettier.enable = true;
    programs.toml-sort.enable = true;
    programs.sql-formatter.enable = true;
  };

  check = pkgs.writeShellApplication {
    name = "bun2nix-formatter";

    runtimeInputs = [
      formatter
      pkgs.git
    ];

    text = ''
      nix fmt

      if ! git diff --exit-code; then
        echo "-------------------------------"
        echo "aborting due to above changes ^"
        exit 1
      fi

      touch $out
    '';
  };
in
formatter
// {
  meta = formatter.meta // {
    tests = {
      check = check;
    };
  };
}

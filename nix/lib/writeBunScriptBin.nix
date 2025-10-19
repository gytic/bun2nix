{
  perSystem =
    { pkgs, ... }:
    {
      packages.writeBunScripBin =

        {
          name,
          text,
        }:
        pkgs.writeTextFile {
          inherit name;
          text = ''
            #!${pkgs.bun}/bin/bun
            ${text}
          '';
          executable = true;
          destination = "/bin/${name}";
        };
    };
}

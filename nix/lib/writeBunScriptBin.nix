{ writeTextFile, bun, ... }:
{
  name,
  text,
}:
writeTextFile {
  inherit name;
  text = ''
    #!${bun}/bin/bun
    ${text}
  '';
  executable = true;
  destination = "/bin/${name}";
}

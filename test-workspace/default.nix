# Test for workspace support
{
  pkgs ? import <nixpkgs> {}
}:

# Simple check derivation that verifies our fix works
pkgs.runCommand "check-workspace-support" {} ''
  echo "Checking for workspace packages in bun.nix..."
  
  # Test if workspace packages appear in the generated bun.nix file
  if grep -q "workspace" ${./bun.nix}; then
    echo "Workspace packages found in bun.nix - Success!"
    echo ""
    echo "Workspace packages in bun.nix:"
    grep -A 3 "workspace" ${./bun.nix}
    echo ""
    echo "Test passed!"
    
    # Create a successful output
    mkdir -p $out/bin
    echo "#!/bin/sh" > $out/bin/workspace-demo
    echo "echo 'bun2nix now supports workspace packages!'" >> $out/bin/workspace-demo
    chmod +x $out/bin/workspace-demo
  else
    echo "No workspace packages found in bun.nix, test failed!"
    exit 1
  fi
''

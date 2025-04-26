# Test for workspace support
{
  pkgs ? import <nixpkgs> {},
  lib ? pkgs.lib
}:

let
  # Import individual modules directly
  mkBunNodeModules = pkgs.callPackage ../nix/lib/mkBunNodeModules.nix {};
  mkBunDerivation = pkgs.callPackage ../nix/lib/mkBunDerivation.nix { inherit mkBunNodeModules; };
  
  # Create a test app using our enhanced mkBunDerivation
  workspaceApp = mkBunDerivation {
    pname = "workspace-test-app";
    version = "1.0.0";
    
    src = ./packages/app;
    bunNix = ./bun.nix;
    
    # Test the workspaceRoot approach
    workspaceRoot = ./.; # Points to the test-workspace directory
    
    # This would be the alternative approach using explicit mappings:
    # workspaces = {
    #   "@workspace/lib" = ./packages/lib;
    # };
    
    # Since we're not using the default build approach
    buildPhase = ''
      echo "Building workspace app..."
      echo "Checking node_modules structure:"
      ls -la node_modules/@workspace || true
      
      # Create a simple build output
      mkdir -p build
      
      # Check if workspace lib is linked
      ls -la node_modules/@workspace/lib || true
      
      bun run build
    '';
    
    installPhase = ''
      mkdir -p $out
      cp dist/app $out
      
      chmod +x $out/app
    '';
  };

in workspaceApp

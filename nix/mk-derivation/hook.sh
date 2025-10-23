# shellcheck shell=bash

# shellcheck disable=SC2034
readonly bunDefaultFlagsArray=(@bunDefaultFlags@)

function bunSetInstallCacheDir {
  if ! [ -v bunDeps ]; then
    printf '\n\033[31mError:\033[0m %s.\n\n' "$(
      cat <<'EOF'
Please set `bunDeps` in order to use `bun2nix.hook` or
`bun2nix.mkDerivation` to build your package.

# Example
```nix
stdenv.mkDerivation {
  <other inputs>

  nativeBuildInputs = [
    bun2nix.hook
  ];

  bunDeps = bun2nix.fetchBunDeps {
    bunNix = ./bun.nix;
  };
}
```
EOF
    )" >&2

    exit 1
  fi

  BUN_INSTALL_CACHE_DIR=$(mktemp -d)
  export BUN_INSTALL_CACHE_DIR

  cp -r "$bunDeps"/. "$BUN_INSTALL_CACHE_DIR"
}

function bunPatchPhase {
  runHook prePatch

  patchShebangs .

  HOME=$(mktemp -d)
  export HOME

  runHook postPatch
}

function bunNodeModulesInstallPhase {
  runHook preBunNodeModulesInstallPhase

  bun install --ignore-scripts

  runHook postBunNodeModulesInstallPhase
}

function bunLifecycleScriptsPhase {
  runHook preBunNodeModulesFixupPhase

  chmod -R u+rwx ./node_modules
  bun install

  runHook postBunNodeModulesFixupPhase
}

function bunBuildPhase {
  runHook preBuild

  local flagsArray=()
  concatTo flagsArray bunDefaultFlagsArray \
    bunBuildFlags bunBuildFlagsArray

  echoCmd 'bun build flags' "${flagsArray[@]}"
  ls -la
  bun build "${flagsArray[@]}"

  runHook postBuild
}

function bunCheckPhase {
  runHook preCheck

  local flagsArray=()
  concatTo flagsArray bunDefaultFlagsArray \
    bunCheckFlags bunCheckFlagsArray

  echoCmd 'bun check flags' "${flagsArray[@]}"
  bun test "${flagsArray[@]}"

  runHook postCheck
}

function bunInstallPhase {
  runHook preInstall

  if ! [ -v pname ]; then
    printf '\033[31mError:\033[0m %s.\n' "'pname' was not defined, please make sure you are running this in a nix build script"
    exit 1
  fi
  if ! [ -v out ]; then
    printf '\033[31mError:\033[0m %s.\n' "'out' was not defined, please make sure you are running this in a nix build script"
    exit 1
  fi

  install -Dm755 "$pname" "$out/bin/$pname"

  runHook postInstall
}

appendToVar preConfigurePhases bunSetInstallCacheDir
appendToVar preBuildPhases bunNodeModulesInstallPhase

if [ -z "${dontRunLifecycleScripts-}" ]; then
  appendToVar preBuildPhases bunLifecycleScriptsPhase
fi

if [ -z "${dontUseBunPatch-}" ] && [ -z "${patchPhase-}" ]; then
  patchPhase=bunPatchPhase
fi

if [ -z "${dontUseBunBuild-}" ] && [ -z "${buildPhase-}" ]; then
  buildPhase=bunBuildPhase
fi

if [ -z "${dontUseBunCheck-}" ] && [ -z "${checkPhase-}" ]; then
  checkPhase=bunCheckPhase
fi

if [ -z "${dontUseBunInstall-}" ] && [ -z "${installPhase-}" ]; then
  installPhase=bunInstallPhase
fi

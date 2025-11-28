# shellcheck shell=bash

# shellcheck disable=SC2034
readonly bunDefaultInstallFlagsArray=(@bunDefaultInstallFlags@)

function bunSetInstallCacheDirPhase {
  runHook preBunSetInstallCacheDirPhase

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

  cp -r "$bunDeps"/share/bun-cache/. "$BUN_INSTALL_CACHE_DIR"

  runHook postBunSetInstallCacheDirPhase
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

  local flagsArray=()
  if [ -z "${bunInstallFlags-}" ] && [ -z "${bunInstallFlagsArray-}" ]; then
    concatTo flagsArray \
      bunDefaultInstallFlagsArray
  else
    concatTo flagsArray \
      bunInstallFlags bunInstallFlagsArray
  fi

  local ignoreFlagsArray=("--ignore-scripts")
  concatTo flagsArray ignoreFlagsArray

  echoCmd 'bun install flags' "${flagsArray[@]}"
  bun install "${flagsArray[@]}"

  runHook postBunNodeModulesInstallPhase
}

function bunLifecycleScriptsPhase {
  runHook preBunLifecycleScriptsPhase

  chmod -R u+rwx ./node_modules

  local flagsArray=()
  if [ -z "${bunInstallFlags-}" ] && [ -z "${bunInstallFlagsArray-}" ]; then
    concatTo flagsArray \
      bunDefaultInstallFlagsArray
  else
    concatTo flagsArray \
      bunInstallFlags bunInstallFlagsArray
  fi

  echoCmd 'bun lifecycle install flags' "${flagsArray[@]}"
  bun install "${flagsArray[@]}"

  runHook postBunLifecycleScriptsPhase
}

function bunBuildPhase {
  runHook preBuild

  local flagsArray=()
  concatTo flagsArray \
    bunBuildFlags bunBuildFlagsArray

  echoCmd 'bun build flags' "${flagsArray[@]}"
  bun build "${flagsArray[@]}"

  runHook postBuild
}

function bunCheckPhase {
  runHook preCheck

  local flagsArray=()
  concatTo flagsArray \
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

appendToVar preConfigurePhases bunSetInstallCacheDirPhase
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

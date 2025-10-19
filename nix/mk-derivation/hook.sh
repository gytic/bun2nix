# shellcheck shell=bash

# shellcheck disable=SC2034
readonly bunDefaultFlagsArray=(@bun_default_flags@)

function bunSetInstallCacheDir {
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

function bunBuildPhase {
  runHook preBuild

  local flagsArray=()
  concatTo flagsArray bunDefaultFlagsArray \
    bunBuildFlags bunBuildFlagsArray

  echoCmd 'bun build flags' "${flagsArray[@]}"
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

  install -Dm755 "$pname" "$out/bin/$pname"

  runHook postInstall
}

# shellcheck disable=SC2154
addEnvHooks "$targetOffset" bunSetInstallCacheDir

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

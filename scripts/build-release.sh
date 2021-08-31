#!/bin/bash
set -e

pushd . > /dev/null
SCRIPT_PATH="${BASH_SOURCE[0]}"
if ([ -h "${SCRIPT_PATH}" ]); then
  while([ -h "${SCRIPT_PATH}" ]); do cd `dirname "$SCRIPT_PATH"`;
  SCRIPT_PATH=`readlink "${SCRIPT_PATH}"`; done
fi
cd `dirname ${SCRIPT_PATH}` > /dev/null
SCRIPT_PATH=`pwd`;
popd  > /dev/null
. "$SCRIPT_PATH/switch-to-repo-main.sh"

RUSTFLAGS="--remap-path-prefix=$HOME/.cargo=[crates.io] --remap-path-prefix=$(pwd)=[crate-root]" \
  trunk --config documentation/Trunk.release.toml build

# Print quick overview of the expected sizes
ls -AshS1F dist
#  -A: all except . and ..
#  -s: print sizes
#  -h: human readable sizes
#  -S: sort by size descending
#  -1: single column
#  -F: classify file type

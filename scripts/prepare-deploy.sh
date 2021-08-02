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

./scripts/build-release.sh
cd dist
git worktree repair dist || true
git add .
git write-tree |
  xargs git commit-tree -m "force update demo" |
  xargs git reset --hard

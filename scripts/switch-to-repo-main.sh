#!/bin/bash
# Switch the current working path to the root of this repo

pushd . > /dev/null
SCRIPT_PATH="${BASH_SOURCE[0]}";
if ([ -h "$SCRIPT_PATH" ]) then
  while([ -h "$SCRIPT_PATH" ]) do cd `dirname "$SCRIPT_PATH"`; SCRIPT_PATH=`readlink "$SCRIPT_PATH"`; done
fi
cd `dirname $SCRIPT_PATH` > /dev/null
REPO_MAIN=`dirname "$(pwd)"`
popd  > /dev/null
cd "$REPO_MAIN"

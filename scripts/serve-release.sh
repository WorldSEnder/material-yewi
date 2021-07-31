set -e

trunk --config Trunk.release.toml build --release
cd dist
serve -g -p 8080

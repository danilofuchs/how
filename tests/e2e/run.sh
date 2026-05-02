#!/usr/bin/env bash
# Build and run an end-to-end test image.
#   ./tests/e2e/run.sh <image>
# where <image> is one of: debian, arch, fedora.
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: $0 <debian|arch|fedora>" >&2
  exit 2
fi

img=$1
repo_root=$(cd "$(dirname "$0")/../.." && pwd)
dockerfile="$repo_root/tests/e2e/$img.Dockerfile"

if [[ ! -f $dockerfile ]]; then
  echo "no Dockerfile at $dockerfile" >&2
  exit 2
fi

docker build -f "$dockerfile" -t "how-e2e-$img" "$repo_root"
docker run --rm "how-e2e-$img"

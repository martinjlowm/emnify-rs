##!/usr/bin/env bash

curl https://cdn.emnify.net/api/doc/apis/enterprise.yaml -o enterprise.yaml

for p in patches/*; do
  patch enterprise.yaml < $p
done

openapi-generator \
  generate \
  -i enterprise.yaml \
  -g rust \
  --additional-properties=packageName=emnify,packageVersion=2.0.0 \
  -o ./

cargo clippy --fix
cargo fmt

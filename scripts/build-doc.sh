#!/usr/bin/env bash

cargo doc \
  --workspace \
  --no-deps

echo "<meta http-equiv=\"refresh\" content=\"0; url=prdoc\">" > target/doc/index.html

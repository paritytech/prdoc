#!/usr/bin/env bash

set -eu -o pipefail
shopt -s inherit_errexit

rm -rf dist
yarn build
cd dist
git init .
git add -A
git checkout -b gh-pages
git commit -m "gh-pages update"
git remote add origin git@github.com:mutantcornholio/prdoc.git
git push origin +gh-pages

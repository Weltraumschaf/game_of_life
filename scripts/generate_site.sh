#!/usr/bin/env bash

set -e
set -u

echo "Publishing documentation ..."

cargo clean
cargo build
cargo test
cargo doc --no-deps
rustdoc --test README.md -L target

tmp_dir=$(mktemp -d)

pushd "${tmp_dir}"
git clone --quiet git@github.com:Weltraumschaf/game_of_live.git site

pushd site
git config user.email "travis@travis-ci.org"
git config user.name "travis-ci"

git checkout gh-pages
git rm -rf .
cp -Rf ${PROJECT_HOME}/target/doc/* .
git checkout master -- README.md

git add -f .
git commit -m "Auto doc upload from travis"
git push -fq origin gh-pages > /dev/null

popd
popd
rm -rfv "${tmp_dir}"

echo "Published documentation :-)"
#!/bin/bash

set -e

bazel build buildinfo.txt ts/...
rm -rf droid
mkdir -p droid/web
rsync -a .bazel/bin/buildinfo.txt droid
for i in card-info graphs deck-options change-notetype congrats; do
    cp -av .bazel/bin/ts/$i/$i.{html,js,css} droid/web/
done
cd droid
chmod -R a+w *
zip -r ../anki_artifacts.zip *


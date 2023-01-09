#!/bin/env bash
set -e

AAPT_PATH=$1
APK_INPUT=$2
OUTPUT=$3
TEMP=$(mktemp -d)

$AAPT_PATH/aapt2 convert $APK_INPUT --output-format proto -o $TEMP/app_proto.apk

cd $TEMP

unzip app_proto.apk
mkdir manifest
mkdir dex
mv AndroidManifest.xml manifest/
mv classes.dex dex/
rm app_proto.apk

zip -r base.zip *

cd -

bundletool build-bundle --modules=$TEMP/base.zip --output=$OUTPUT

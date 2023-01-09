#!/bin/bash
set -e

AAPT_PATH=$1
APK_INPUT=$2
OUTPUT=$3
TEMP=$(mktemp -d)

$AAPT_PATH/aapt2 convert $APK_INPUT --output-format proto -o $TEMP/app_proto.apk


cd $TEMP

# install bundletool
apt-get install -y zip
wget -O bundletool.jar https://github.com/google/bundletool/releases/download/1.13.2/bundletool-all-1.13.2.jar

unzip app_proto.apk
mkdir manifest
mkdir dex
mv AndroidManifest.xml manifest/
mv classes.dex dex/
rm app_proto.apk

zip -r base.zip *

cd -

java -jar bundletool.jar build-bundle --modules=$TEMP/base.zip --output=$OUTPUT

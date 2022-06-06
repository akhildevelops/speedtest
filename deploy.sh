#!/bin/bash


set -o errexit
set -o nounset
set -o pipefail
set -o xtrace


readonly TARGET_HOST=pi@192.168.0.103
readonly TARGET_PATH=/home/pi/speedtest
readonly TARGET_ARCH=arm-unknown-linux-gnueabihf
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/speedtest

cargo build --release --target=${TARGET_ARCH}

rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
ssh -t ${TARGET_HOST} ${TARGET_PATH}


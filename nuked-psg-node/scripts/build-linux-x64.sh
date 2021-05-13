#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

$SCRIPT_DIR/build.sh linux-x64 x86_64-unknown-linux-gnu libnuked_psg_node.so


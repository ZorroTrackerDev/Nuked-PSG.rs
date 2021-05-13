#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

$SCRIPT_DIR/build.sh linux-x86 i686-unknown-linux-gnu libnuked_psg_node.so

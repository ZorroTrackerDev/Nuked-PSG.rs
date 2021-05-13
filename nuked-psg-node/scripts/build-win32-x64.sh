#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

$SCRIPT_DIR/build.sh win32-x64 x86_64-pc-windows-gnu nuked_psg_node.dll


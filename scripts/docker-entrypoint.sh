#!/bin/sh
set -e

server() {
    exec /server $@
}

agent() {
    exec /agent $@
}

if [ "$#" = 0 ]; then
    echo "Mode not selected. Running as server"
    server $@
fi

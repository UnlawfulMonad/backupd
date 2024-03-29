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

case "$1" in
    server)
        shift
        exec server $@
        ;;
    client)
        shift
        exec client $@
        ;;
    *)
        echo "Unknown value $1"
        exit 1
        ;;
esac

#!/bin/sh
if [ "x${POSTGRES_RESET_DB}" = "x1" ]; then
    rm -rf /data/database
fi
exit 0

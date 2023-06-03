#!/usr/bin/env bash

set -x
set -eo pipefail

FILE_LOCATION="sqlite.db"

DATABASE_URL="sqlite://${FILE_LOCATION}"
export DATABASE_URL

if [ ! -f $FILE_LOCATION ]; then
    sqlx database create
else
    echo "File Exists, skip creation"
fi
sqlx migrate run

echo "SQLite Database Ready To Go"
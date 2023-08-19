#!/usr/bin/env bash

set -x
set -eo pipefail

FILE_LOCATION="sqlite.db"
PASSWORD="joanne23"
OFFLINE_FILE="sqlx-data.json"

DATABASE_URL="sqlite://${FILE_LOCATION}"
export DATABASE_URL


if [ -f $FILE_LOCATION ]; then
    echo "File Exists, skip creation"
    rm -f ./${FILE_LOCATION}*
fi

if [ -f $OFFLINE_FILE ]; then
    echo "Removing offling file"
    rm -f ./${OFFLINE_FILE}
fi

echo "Creating Database"

sqlx database create
sqlx migrate run
cargo sqlx prepare -- --lib

echo "SQLite Database Ready To Go"
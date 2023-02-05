#!/usr/bin/env bash

DIR_PATH=$(pwd)

path=$DIR_PATH/$(basename “${BASH_SOURCE:-$0}”)

echo ‘The absolute path is’ $path
echo ‘----------------------------------’
echo ‘The directory path is’ $DIR_PATH

DIR="$DIR_PATH/migrations"

if test -d  "$DIR"; then
    echo "migrations directory exist"
else
    echo "directory not found"
fi

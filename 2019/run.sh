#!/bin/sh

if [ -z "$1" ]; then
    echo "Example usage: ./run.sh day1"
    exit 1
fi

export PYTHONPATH=$PYTHONPATH:.
pipenv run python $1/main.py

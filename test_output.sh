#!/usr/bin/env bash

filename="output.txt"
if [ ! -z "$1" ]; then
  filename="$1"
fi

cat $filename | while read line; do
  sleep_time=".$(( RANDOM % (5) + 1 ))"
  sleep $sleep_time
  echo $line
done

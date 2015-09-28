#!/usr/bin/env bash

cat output.txt | while read line; do
  sleep_time=".$(( RANDOM % (5) + 1 ))"
  sleep $sleep_time
  echo $line
done

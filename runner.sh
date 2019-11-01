#!/usr/bin/env bash

echo $@

mkdir tmp -p

for f in Rocket.toml keys.txt templates $1; do
    cp -r "$f" tmp
done

echo sending file...
rsync -avr tmp pi@door.local:~

echo executing...
exec ssh pi@door.local "killall door-lock; cd tmp && ./door-lock"
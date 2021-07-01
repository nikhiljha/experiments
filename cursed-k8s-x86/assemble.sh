#!/bin/sh

# These are the flags I needed to use to compile it on x86_64 MacOS.
# The assembly uses MacOS syscalls to do hello world, so you'll need to substitute that with Linux syscalls if you want it to work there.
# Also modify the below to make sense...
as -target x86_64-apple-macos10.12 cursed.yaml.s -o hello_world.o
ld -macosx_version_min 10.12 hello_world.o -o hello_world -lSystem -L /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib
./hello_world

#!/bin/bash
# Example: ./scripts/for-examples/build_and_run.sh hello_world
# This script is mostly a demonstration - it's simpler to do the following...
#     `./roc/target/release/roc ./examples/hello_world.roc`
# ... or, if the built Roc CLI is in your path...
#     `roc examples/hello_world.roc`
example="$1"
example_roc="./examples/$example.roc"
if [ -f "$example_roc" ]; then
	./roc/target/release/roc $example_roc
else
	echo "$example is not an example!"
fi

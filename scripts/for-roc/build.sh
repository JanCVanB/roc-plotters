#!/bin/bash
nix-shell \
	--command 'cd roc && cargo build --release' \
	./roc/shell.nix

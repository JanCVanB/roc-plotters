#!/usr/bin/env bash
cd roc
cargo build --release  || true # || true so we can still go back to the previous dir on failure
cd ..

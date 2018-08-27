#!/usr/bin/env bash

cargo build
target/debug/bintester test/success_cases.json
target/debug/bintester test/failure_cases.json

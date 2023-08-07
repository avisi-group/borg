#!/usr/bin/env bash
set -e

# export bincode from sail
cargo r -- --log info sail2bincode data/sail-arm-full.json data/arm-v8.5-a.bincode.lz4

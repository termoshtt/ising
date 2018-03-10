#!/bin/bash

set -xe

cargo run --example movie
cd movie
ffmpeg -r 30 -i ising_%05d.png -pix_fmt yuv420p out.webm
cd -

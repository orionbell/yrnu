#!/bin/env bash
printf "Downloading..."
(git clone --depth 1 https://github.com/orionbell/yrnu.git -q && printf "done!\n") || (printf "failed!\n" && exit 1)
cd ./yrnu
printf "Compiling..."
(RUSTFLAGS=-Awarnings cargo build -r -q && printf "done!\n")|| (printf "failed!\n" && exit 2)
(sudo cp ./target/release/yrnu /usr/bin/ && printf "Moved yrnu to /usr/bin\n") || (printf "failed to move yrnu to /usr/bin!\n" && exit 3)
cd ..
printf "Cleaning..."
(rm -rf ./yrnu && printf "done!\nYrnu is successfuly installed :)\n") || (printf "failed!\n" && exit 4)

#!/bin/bash

# A bash script to quickly setup a testing environment for rmsafe

if [ ! -f ".firstrun" ] 
then
    # run command 1
    `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    touch ".firstrun"
else
    `cargo install rmsafe`
fi


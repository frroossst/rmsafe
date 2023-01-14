#!/bin/bash

# A bash script to quickly setup a testing environment for rmsafe

if [ ! -f ".firstrun" ] 
then
    sudo apt-get install git -y
    sudo apt-get install gcc-multilib -y
    sudo apt-get install libssl-dev -y
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    touch ".firstrun"
    reboot
else
    cargo install rmsafe
fi


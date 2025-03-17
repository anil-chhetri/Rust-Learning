#!/bin/bash
export PS1="\\W> "

#source ~/path/to/shell-functions.sh
# echo "source /workspaces/Rust-Learning/env-setup.sh"  >> ~/.bashrc


if ! command -v rustc &>/dev/null; then
    echo "Rust not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi

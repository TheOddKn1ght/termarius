#!/bin/sh
set -eu

if [ "$#" -gt 2 ]; then
    printf '%s\n' 'Usage: install.sh [BINARY_PATH] [INSTALL_DIR]' >&2
    exit 1
fi

script_dir=$(CDPATH='' cd -P "$(dirname "$0")" && pwd)
install_dir=${2:-"$HOME/.local/bin"}

if [ "$#" -ge 1 ]; then
    binary=$1
elif [ -f "$script_dir/termarius" ]; then
    binary=$script_dir/termarius
else
    binary=$script_dir/target/release/termarius
fi

if [ ! -f "$binary" ]; then
    printf '%s\n' 'No prebuilt Termarius binary found. Place termarius beside this script or pass its path as the first argument.' >&2
    exit 1
fi

mkdir -p "$install_dir"
install -m 755 "$binary" "$install_dir/termarius"
printf '\nInstalled Termarius: %s\n' "$install_dir/termarius"

case ":$PATH:" in
    *":$install_dir:"*) printf '%s\n' 'Run: termarius' ;;
    *) printf 'Add %s to PATH to run termarius.\n' "$install_dir" ;;
esac

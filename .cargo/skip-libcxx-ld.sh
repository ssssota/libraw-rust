#!/bin/bash

new_args=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    -l)
      if [[ "$2" == "c++" ]]; then
        shift 2
        continue
      fi
      ;;
  esac
  new_args+=("$1")
  shift
done

exec rust-lld "${new_args[@]}"

#!/bin/bash
new_args=()
for arg in "$@"; do
  if [[ "$arg" == --target=* ]]; then
    # Ignore the target passed by the user
    new_args+=(--target=wasm32-unknown-emscripten)
  else
    new_args+=("$arg")
  fi
done
exec /emsdk/upstream/emscripten/em++ "${new_args[@]}"

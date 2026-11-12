#!/usr/bin/env bash
set -euo pipefail

game_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
game_binary="$game_root/target/release/road-dash"
if [[ ! -x "$game_binary" ]]; then
    printf 'Release build missing: %s\nRun cargo build --release --locked in %s first.\n' "$game_binary" "$game_root" >&2
    exit 1
fi

cd "$game_root"
export ROADDASH_SAVE_DIR="${ROADDASH_SAVE_DIR:-$HOME/.local/share/roaddash-bevy20-playtest}"
export ROADDASH_LOG_DIR="${ROADDASH_LOG_DIR:-$HOME/.local/state/roaddash-bevy20-playtest}"
export ROADDASH_RAW_FRAMES="${ROADDASH_RAW_FRAMES:-1}"
export ROADDASH_VISIBILITY_LOG="${ROADDASH_VISIBILITY_LOG:-1}"
export ROADDASH_LOAD_PROFILE="${ROADDASH_LOAD_PROFILE:-1}"
mkdir -p "$ROADDASH_LOG_DIR"
console_log="$ROADDASH_LOG_DIR/console-$(date +%Y%m%d-%H%M%S)-$$.log"
printf 'RoadDash 0.20.1 playtest logs: %s\nConsole log: %s\n' "$ROADDASH_LOG_DIR" "$console_log" >&2
"$game_binary" "$@" 2>&1 | tee "$console_log"

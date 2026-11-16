#!/usr/bin/env bash
# Runs an isolated Hills comparison in the caller's display session.
set -euo pipefail
repo=$(cd "$(dirname "$0")/.." && pwd)
if [[ $# -lt 4 || $# -gt 5 ]]; then
    echo "Usage: $0 out-dir label road|grid|speed flags [baseline|ambient|poly|procedural]" >&2
    exit 2
fi
mkdir -p "$1"
out=$(cd "$1" && pwd)
label=$2
[[ "$label" =~ ^[a-z0-9-]+$ ]] || exit 2
shot=$3
flags=$4
road=${5:-baseline}
binary=${ROADDASH_POLISH_BINARY:-"$repo/target/release/road-dash"}
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
cd "$scratch"
ln -s "$repo/content" content
export BEVY_ASSET_ROOT="$repo"
export ROADDASH_LOG_DIR="$scratch" ROADDASH_SETTINGS="$scratch/settings.json" ROADDASH_SAVE_DIR="$scratch/save"
export ROADDASH_RACE_SEED=25 ROADDASH_POLISH="$flags" ROADDASH_POLISH_ROAD="$road" ROADDASH_POLISH_CAPTURE="$shot" ROADDASH_RAW_FRAMES=1
timeout 120 "$binary" --hills --polish-survey >"$out/$label.log" 2>&1 &
game_pid=$!
if [[ "${ROADDASH_CAPTURE_X11:-0}" == 1 ]]; then
    # Game screenshots are captured before external swapchain layers.
    for _ in {1..1100}; do
        [[ -s screenshots/polish.png ]] && break
        kill -0 "$game_pid" 2>/dev/null || break
        sleep 0.1
    done
    if [[ -s screenshots/polish.png ]]; then
        import -window root "$out/$label.png"
    fi
fi
wait "$game_pid"
[[ "${ROADDASH_CAPTURE_X11:-0}" == 1 ]] || cp screenshots/polish.png "$out/$label.png"
test -s "$out/$label.png"
cp screenshots/polish.json "$out/$label.json"
for log in "$scratch"/performance-*.jsonl; do
    if [[ -f "$log" ]]; then
        cp "$log" "$out/$label.perf.jsonl"
    fi
done

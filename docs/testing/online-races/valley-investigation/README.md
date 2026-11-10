# Private native Metropolis → Valley investigation

These are retained unsuccessful fixture runs, not acceptance receipts.
Both runs reconnected native host/guest children after a level change with the
same36-character private capability, kept two real humans and shared moving
state, and recorded no transport errors. Public DHT and UPnP were disabled.

- `short/`: binary5666c19c…,45s guest/48s host wall window (15s countdown included).
  Both saw crash and foot movement, neither observed remount before exit. Host
  simulation had advanced only24.1s. The harness failed its remount assertion.
- `extended/`: binary4cf392e7…,90s guest/93s host wall window. Guest observed crash,
  foot and remount; host's Last-render sampler missed the short Running state,
  although the client received that authoritative state. Both ended Riding.
  The harness therefore failed the host foot assertion. Snapshot observation now
  also records these state transitions at the actual20Hz replication point.

Review independently found and corrected a remote-human parity defect: NPC-only
live obstacle-hold logic still tested `id != 0`, while path following correctly
used `!human_controlled()`. Remote humans could enter the NPC hold without its
path following. The fix preserves the existing local-player behavior and physical
parameters. A same-wall local/remote recovery-system regression was added.

The extended run also exposed the host result-display clock replacing session
elapsed time after all humans finished. The host now retains a monotonic snapshot
clock independently; a real compressed TCP publish regression covers early host
finish, later guest finish and retained aged strike clocks.

Final native Valley rerun on the combined corrected binary remains pending.

The corrected final binary subsequently passed the same private Valley reload
with a90s window: see `../final-valley-reload/` (1497 exact shared snapshots,
both peers observed every recovery transition, no errors). Earlier receipts are
retained to distinguish the initial duration/sampling failures from the final
verified behavior.

# Real host admission and lifecycle — 2026-11-09

`tools/test_online_admission.py` passed against the actual GPU-free production
listen-host/game simulation. It used only loopback; public DHT and UPnP disabled.
The receipt records binary identity, first snapshot timing and the before/after
state of the unfinished bot replaced by a late human.

Verified wrong private capability and wrong protocol rejection; private hosts do
not offer Quick Play admission; late human inherits slot2's existing moving state;
disconnect restores its bot roster entry; invalid control input disconnects;
all14 guest slots are unique and a fifteenth guest is rejected; terminating the
host closes the remaining observer connection. First snapshot arrived15.107s
after host launch. Bots may have negative progress on the randomized grid, so the
fixture compares actual pose/progress continuity instead of assuming positive
race progress. An initial incorrect positive-progress assertion was excluded.

These raw protocol peers validate the actual authority and transport. Native
rendering/recovery is covered by the independent native pair receipts. Host EOF
here is a transport assertion, not a separate native disconnect-menu screenshot.

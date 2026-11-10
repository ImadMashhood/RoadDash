# Actual client application host-loss check

The GPU-free production host and client applications first formed a two-human
session. The host process was terminated after5s; the client application recorded
`Connection lost: peer closed with an incomplete frame` and exited with the
fixture's expected failure status1 (no race samples because countdown was not
complete). This verifies actual client pump failure handling beyond a raw socket
EOF check. It does not substitute for a native disconnect-menu screenshot.
The standalone reproducer records its exact task-owned paths and binary.

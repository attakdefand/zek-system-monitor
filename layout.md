zek/
├── Cargo.toml                 # Workspace + common [workspace.dependencies]
├── README.md
├── LICENSE
├── .gitignore
├── configs/
│   ├── zek.toml             # main config (collectors, refresh, thresholds, exporters)
│   ├── alerts.yml             # alert rules & actions
│   ├── fleet.yml              # agents, tags, RBAC, mTLS cert refs
│   └── dash/                  # saved TUI/Web layouts (pin/pane presets)
├── scripts/
│   ├── dev_run_tui.sh
│   ├── dev_run_agent.sh
│   ├── dev_run_gateway.sh
│   └── gen_selfsigned_certs.sh
├── assets/                    # icons, web ui static, fonts (if any)
├── docs/
│   ├── architecture.md
│   ├── collectors.md
│   ├── exporters.md
│   └── ebpf-notes.md
├── apps/
│   ├── zek-tui/             # Interactive terminal UI (ratatui)
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   ├── zek-agent/           # Headless node agent (API + exporters)
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   └── zek-gateway/         # Fleet gateway (multi-host, RBAC, auth, web)
│       ├── Cargo.toml
│       └── src/main.rs
├── crates/
│   ├── core-metrics/          # Unified collection interface + sampling loop
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── cfg.rs         # parse zek.toml
│   │       ├── ringbuf.rs     # in-memory short-term history
│   │       └── collectors/
│   │           ├── system.rs  # OS/kernel, uptime, header data
│   │           ├── cpu_mem.rs # CPU (per-core), mem/swap, load
│   │           ├── sensors_power.rs    # thermals, fans, RAPL, powercap
│   │           ├── numa_topology.rs    # NUMA nodes, SMT/core mapping
│   │           ├── psi.rs              # Linux PSI pressure
│   │           ├── disk.rs             # util/latency/queue from /proc/diskstats
│   │           ├── fs.rs               # mount usage, inode, hot dirs/files
│   │           ├── net.rs              # iface rx/tx, errors, drops
│   │           ├── tcp.rs              # RTT, retrans, cwnd, socket states
│   │           ├── tls.rs              # listener cert expiry scanning
│   │           ├── processes.rs        # top, faults, smaps rollups
│   │           ├── namespaces.rs       # PID/MNT/NET namespace tree
│   │           ├── cgroups.rs          # per-cgroup cpu/mem/io/psi, throttling, OOM
│   │           ├── storage_health.rs   # SMART (smartctl), mdraid, ZFS ARC/pool
│   │           ├── gpu.rs              # NVML/ROCm/Intel (feature "gpu")
│   │           └── ebpf.rs             # aya programs (feature "ebpf")
│   ├── web-api/               # axum REST, WS live stream, /metrics proxy
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── exporters/             # pluggable sinks
│   │   ├── prometheus/        # /metrics OpenMetrics endpoint
│   │   │   ├── Cargo.toml
│   │   │   └── src/lib.rs
│   │   ├── otlp/
│   │   │   ├── Cargo.toml
│   │   │   └── src/lib.rs
│   │   ├── influx/
│   │   │   ├── Cargo.toml
│   │   │   └── src/lib.rs
│   │   ├── jsonl/
│   │   │   ├── Cargo.toml
│   │   │   └── src/lib.rs
│   │   └── sqlite_parquet/    # local history backends
│   │       ├── Cargo.toml
│   │       └── src/lib.rs
│   ├── probes/                # synthetic DNS/HTTP/ICMP checks
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── alerts/                # rules engine (thresholds, rate-of-change, multi-signal)
│   │   ├── Cargo.toml
│   │   └── src/{lib.rs,engine.rs,actions.rs}
│   ├── anomaly/               # EWMA/seasonality detectors
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── security/              # posture: ports→proc map, caps, SELinux/AppArmor, secrets checks
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── tui/                   # ratatui widgets, panes, keymap, layouts
│   │   ├── Cargo.toml
│   │   └── src/{lib.rs,widgets/*.rs}
│   └── utils/                 # common helpers (time, units, serde types, error)
│       ├── Cargo.toml
│       └── src/lib.rs
└── clients/
    └── zek-web-ui/          # small SPA (optional) served by gateway
        ├── package.json
        ├── src/
        └── dist/


### What maps where (to your feature list)

Thermals & power → core-metrics::collectors::sensors_power

NUMA & topology → numa_topology

GPU/TPU → gpu (feature gpu)

Storage health → storage_health (SMART/ZFS/mdraid)

Pressure/Stalls → psi

Disk latency/queue → disk

Filesystem hot paths → fs

TCP/Net & TLS expiry → tcp, tls

DNS/HTTP probes → probes

eBPF insights → ebpf (feature ebpf)

cgroups/containers → cgroups

Namespace tree → namespaces

Memory detail/smaps → processes

Controls (opt-in) → gated in processes/cgroups with a controls cfg

History (SQLite/Parquet) → exporters/sqlite_parquet

Alerts/anomaly/SLO → alerts + anomaly (+ probes for SLO)

Exporters/APIs → exporters/*, web-api

Security & posture → security

TUI niceties → tui

Web UI → zek-gateway serves clients/zek-web-ui build

Fleet mode → zek-agent + zek-gateway (mTLS, RBAC in gateway)
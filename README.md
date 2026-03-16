# 🧠 Engram Dashboard

A native desktop dashboard for [Engram](https://github.com/Atomlaunch/engram) — a temporal knowledge graph memory system by [Atomlaunch](https://github.com/Atomlaunch).

This project is a **Tauri 2.x desktop app** that wraps Engram's FastAPI dashboard with a native window, themed UI, and auto-managed backend. We didn't build Engram itself — just a nice Rust desktop shell with dark themes for exploring the graph.

![Tauri](https://img.shields.io/badge/Tauri-2.x-blue?logo=tauri) ![Python](https://img.shields.io/badge/Python-3.10+-yellow?logo=python) ![Neo4j](https://img.shields.io/badge/Neo4j-5.x-green?logo=neo4j)

## What This Adds

- **Native desktop window** — no browser tab needed
- **Auto-managed backend** — Rust spawns the Python server on launch, kills it on close
- **Themes** — Dark (default), Dracula, and Nord color schemes
- **Styled dropdowns & UI** — WebKitGTK-friendly dark theme overrides

## Features (from Engram)

- **Overview** — Stats, node type breakdown, recent activity timeline
- **Graph Explorer** — Interactive node graph (Sigma.js), click to explore neighbors, filter by type
- **Browse** — Search and filter entities, facts, episodes, emotions
- **Multi-agent support** — Filter by agent (Andy, Jarvis, etc.)

## Architecture

```
┌─────────────────────────┐
│   Tauri (Rust)          │
│   - Spawns Python       │
│   - Waits for ready     │
│   - WebView → :3460     │
└────────┬────────────────┘
         │
┌────────▼────────────────┐
│   FastAPI (Python)      │
│   - Engram dashboard    │
│   - REST API + static   │
│   - Port 3460           │
└────────┬────────────────┘
         │
┌────────▼────────────────┐
│   Neo4j (Graph DB)      │
│   - Engram data store   │
└─────────────────────────┘
```

## Prerequisites

- **Rust** (stable) + Cargo
- **Python 3.10+** with pip
- **Neo4j 5.x** running on `bolt://localhost:7687`
- **Engram** — [github.com/Atomlaunch/engram](https://github.com/Atomlaunch/engram) installed and configured
- **System libs** (Linux): `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, etc. ([Tauri prerequisites](https://v2.tauri.app/start/prerequisites/))

## Setup

### 1. Install Engram

Follow the setup at [github.com/Atomlaunch/engram](https://github.com/Atomlaunch/engram). Make sure Neo4j is running and Engram's `config.json` is configured.

### 2. Install Python dependencies

```bash
pip install fastapi uvicorn neo4j
```

### 3. Build & Run

```bash
git clone https://github.com/tuxclaw/engram-dashboard.git
cd engram-dashboard

# Build the Tauri app
cargo tauri build

# Binary at: src-tauri/target/release/engram-tauri
```

### 4. Install (Linux)

```bash
cp src-tauri/target/release/engram-tauri ~/.local/bin/engram
```

## API Endpoints (Engram Dashboard)

| Endpoint | Description |
|----------|-------------|
| `GET /api/status` | Backend health check |
| `GET /api/stats` | Node/relationship counts |
| `GET /api/agents` | List agents in the graph |
| `GET /api/graph` | Full graph data (nodes + edges) |
| `GET /api/graph/neighbors/{id}` | Neighbor subgraph |
| `GET /api/search?q=...` | Full-text search |
| `GET /api/node/{id}` | Node detail |
| `GET /api/timeline` | Recent episodes timeline |

## Tech Stack

- **Desktop:** Tauri 2.x (Rust)
- **Backend:** FastAPI + uvicorn (Python) — from Engram
- **Database:** Neo4j 5.x (Bolt protocol) — from Engram
- **Frontend:** Vanilla HTML/JS/CSS, Sigma.js v2 + Graphology
- **Icons:** Font Awesome 6
- **Fonts:** Inter (Google Fonts)

## Credits

- **Engram** by [Atomlaunch](https://github.com/Atomlaunch/engram) — the memory system and dashboard backend
- **This repo** — Tauri desktop wrapper, theme system, and UI polish by [tuxclaw](https://github.com/tuxclaw)

## License

MIT

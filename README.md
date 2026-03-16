# 🧠 Engram Dashboard

A desktop knowledge graph explorer for [Engram](https://github.com/tuxclaw/engram) — a temporal memory system that stores entities, facts, episodes, emotions, and sessions as a connected graph.

Built with **Tauri 2.x** (Rust) wrapping a **FastAPI** (Python) backend and a vanilla **HTML/JS/CSS** frontend.

![Engram Dashboard](https://img.shields.io/badge/Tauri-2.x-blue?logo=tauri) ![Python](https://img.shields.io/badge/Python-3.10+-yellow?logo=python) ![Neo4j](https://img.shields.io/badge/Neo4j-5.x-green?logo=neo4j)

## Features

- **Overview** — Stats, node type breakdown, recent activity timeline
- **Graph Explorer** — Interactive node graph powered by Sigma.js, click to explore neighbors, filter by type
- **Browse** — Search and filter entities, facts, episodes, emotions across the knowledge graph
- **Multi-agent support** — Filter data by agent (Andy, Jarvis, etc.)
- **Themes** — Dark (default), Dracula, and Nord color schemes
- **Desktop native** — Tauri app with auto-managed Python backend

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
│   - REST API            │
│   - Serves static UI    │
│   - Port 3460           │
└────────┬────────────────┘
         │
┌────────▼────────────────┐
│   Neo4j (Graph DB)      │
│   - Entities, Facts     │
│   - Episodes, Emotions  │
│   - Relationships       │
└─────────────────────────┘
```

## Prerequisites

- **Rust** (stable) + Cargo
- **Python 3.10+** with pip
- **Neo4j 5.x** running on `bolt://localhost:7687`
- **System libs** (Linux): `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, etc. ([Tauri prerequisites](https://v2.tauri.app/start/prerequisites/))

## Setup

### 1. Install Python dependencies

```bash
pip install fastapi uvicorn neo4j
```

### 2. Configure Neo4j

The backend reads from `engram/config.json` (parent of the dashboard directory). Example:

```json
{
  "neo4j": {
    "uri": "bolt://localhost:7687",
    "user": "neo4j",
    "password": "your-password"
  }
}
```

### 3. Build & Run

```bash
# Clone
git clone https://github.com/tuxclaw/engram-dashboard.git
cd engram-dashboard

# Build the Tauri app
cargo tauri build

# The binary will be at:
# src-tauri/target/release/engram-tauri
```

Or run in dev mode:

```bash
# Start the Python backend manually
cd /path/to/engram/dashboard && python server.py

# Then run Tauri dev
cargo tauri dev
```

### 4. Install (Linux)

```bash
cp src-tauri/target/release/engram-tauri ~/.local/bin/engram
```

## API Endpoints

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
- **Backend:** FastAPI + uvicorn (Python)
- **Database:** Neo4j 5.x (Bolt protocol)
- **Frontend:** Vanilla HTML/JS/CSS, Sigma.js v2 + Graphology
- **Icons:** Font Awesome 6
- **Fonts:** Inter (Google Fonts)

## License

MIT

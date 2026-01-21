# Memvid

> Portable single-file memory system for AI agents

## Overview

Memvid is a serverless memory layer for AI agents that packages data, embeddings, search structures, and metadata into a single portable `.mv2` file. It replaces complex RAG pipelines by enabling fast local retrieval and persistent memory without requiring a database.

## Purpose

AI agents need persistent long-term memory that survives across sessions. Memvid provides this through:

- **Single-file portability**: No database setup, no sidecar files
- **Sub-5ms local access**: Fast retrieval with predictive caching
- **Offline operation**: Model-agnostic design works without network
- **Multi-modal support**: Text, images, and audio in one file

## Key Features

| Feature | Description |
|---------|-------------|
| **Smart Frames** | Append-only, immutable data units with crash-safe operation |
| **Time-travel debugging** | Rewind/replay memory states for debugging |
| **Vector search** | HNSW-based vector similarity with local embeddings |
| **Full-text search** | Tantivy-powered lexical search |
| **Auto-compression** | Intelligent codec selection for optimal storage |
| **Temporal tracking** | Timeline-style memory state queries |

## Installation

### Rust (Primary)
```toml
[dependencies]
memvid-core = "2.0"
```

Requirements: Rust 1.85.0+

### SDKs
```bash
# Node.js
npm install @memvid/sdk

# Python
pip install memvid-sdk

# CLI
npm install -g memvid-cli
```

### Feature Flags
Enable specific capabilities in Cargo.toml:
```toml
memvid-core = { version = "2.0", features = ["lex", "vec", "temporal_track"] }
```

Available flags:
- `lex` - Full-text search
- `vec` - Vector similarity
- `pdf_extract` - PDF processing
- `clip` - Visual search
- `whisper` - Audio transcription
- `temporal_track` - Timeline queries
- `parallel_segments` - Parallel processing
- `encryption` - At-rest encryption

## Usage Example

```rust
use memvid_core::{Memvid, PutOptions, SearchRequest};

fn main() -> memvid_core::Result<()> {
    // Create or open a memory file
    let mut mem = Memvid::create("knowledge.mv2")?;

    // Store data with metadata
    let opts = PutOptions::builder()
        .title("Meeting Notes")
        .uri("mv2://meetings/2024-01-15")
        .tag("project", "alpha")
        .build();
    mem.put_bytes_with_options(b"Q4 planning discussion...", opts)?;
    mem.commit()?;

    // Search the memory
    let response = mem.search(SearchRequest {
        query: "planning".into(),
        top_k: 10,
        snippet_chars: 200,
        ..Default::default()
    })?;

    Ok(())
}
```

## File Format

The `.mv2` file contains all necessary components:

| Component | Description |
|-----------|-------------|
| Header (4KB) | Magic number, version, capacity |
| Embedded WAL | 1-64MB crash recovery log |
| Data segments | Compressed content chunks |
| Lex index | Full-text search via Tantivy |
| Vec index | HNSW vector index |
| Time index | Chronological ordering |
| TOC footer | Segment offsets |

No sidecar files (`.wal`, `.lock`, `.shm`) required.

## Embedding Models

Supported local models via ONNX:

| Model | Dimensions | Size |
|-------|------------|------|
| BGE-small (default) | 384 | ~120MB |
| BGE-base | 768 | ~420MB |
| Nomic | 768 | ~530MB |
| GTE-large | 1024 | ~1.3GB |

## Integration with Contextium

Memvid provides persistent memory for AI agents:

- Store conversation history, code context, and learned patterns
- Retrieve relevant context using semantic or lexical search
- Maintain state across multiple agent sessions
- Debug agent behavior with time-travel capabilities

## Use Cases

- Long-running AI agents with persistent memory
- Enterprise knowledge bases
- Offline-first AI systems
- Codebase understanding and navigation
- Customer support agents
- Medical/legal/financial document analysis

## Technical Details

- **Language**: Rust (core), with Node.js/Python SDKs
- **License**: Apache 2.0
- **Repository**: [memvid/memvid](https://github.com/memvid/memvid)

## Links

- [GitHub Repository](https://github.com/memvid/memvid)
- [Contact](mailto:contact@memvid.com)

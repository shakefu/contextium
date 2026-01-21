# GibRAM

> In-memory knowledge graph for RAG (retrieval-augmented generation)

## Overview

GibRAM is an in-memory knowledge graph server built for retrieval-augmented generation (RAG/GraphRAG) workflows. It combines graph storage with vector search capabilities to maintain contextual relationships in RAM, providing graph-aware retrieval that surpasses vector similarity alone.

## Purpose

Traditional vector search retrieves semantically similar content but loses relational context. GibRAM addresses this by:

- **Graph structure**: Entities and relationships stored with embeddings
- **Contextual retrieval**: Traverse relationships for richer context
- **Ephemeral storage**: Configurable TTL for exploration workflows
- **Semantic + graph search**: Combined retrieval strategies

## Key Features

| Feature | Description |
|---------|-------------|
| **In-memory graph** | Fast entity/relationship storage in RAM |
| **Vector search** | Semantic similarity with embeddings |
| **Graph traversal** | Relationship-aware context retrieval |
| **Configurable TTL** | Ephemeral storage for exploration |
| **Python SDK** | Swappable chunkers, extractors, embedders |
| **GraphRAG patterns** | Built for retrieval-augmented generation |

## Installation

### Binary
```bash
curl -fsSL https://gibram.io/install.sh | sh
gibram-server --insecure
```
Server runs on port 6161 by default.

### Docker
```bash
docker run -p 6161:6161 gibramio/gibram:latest
# Or with compose
docker-compose up -d
```

### Python SDK
```bash
pip install gibram
```

## Usage Examples

### Basic Indexing and Querying
```python
from gibram import GibRAMIndexer

indexer = GibRAMIndexer(
    session_id="my-project",
    host="localhost",
    port=6161,
    llm_api_key="sk-..."  # or set OPENAI_API_KEY env
)

# Index documents (extracts entities and relationships)
stats = indexer.index_documents([
    "Python is a programming language created by Guido van Rossum.",
    "JavaScript was created by Brendan Eich at Netscape in 1995."
])

# Query with graph-aware retrieval
results = indexer.query("Who created JavaScript?", top_k=3)
for entity in results.entities:
    print(f"{entity.title}: {entity.score}")
```

### Custom Configuration
```python
from gibram import GibRAMIndexer
from gibram.chunkers import TokenChunker
from gibram.extractors import OpenAIExtractor
from gibram.embedders import OpenAIEmbedder

indexer = GibRAMIndexer(
    session_id="custom-project",
    chunker=TokenChunker(chunk_size=512, chunk_overlap=50),
    extractor=OpenAIExtractor(model="gpt-4o", api_key="..."),
    embedder=OpenAIEmbedder(model="text-embedding-3-small", api_key="...")
)
```

## Architecture

### Components

| Component | Purpose |
|-----------|---------|
| **Chunker** | Splits documents into processable segments |
| **Extractor** | Identifies entities and relationships via LLM |
| **Embedder** | Generates vector embeddings |
| **Graph Store** | In-memory entity/relationship storage |
| **Vector Index** | Semantic similarity search |

### Query Flow

1. Query embedding generated
2. Vector search finds similar entities
3. Graph traversal retrieves related entities
4. Results ranked by combined score
5. Context assembled from graph neighborhood

## Configuration

Server configuration via `config.example.yaml`:
- Port and host settings
- TTL for cached data
- Index parameters
- Authentication options

## Integration with Contextium

GibRAM provides the knowledge graph layer:

- Extract entities and relationships from codebases
- Build contextual understanding of project structure
- Retrieve related concepts during code generation
- Maintain temporary working memory for agent sessions

## Use Cases

- **Codebase understanding**: Map code relationships and dependencies
- **Document analysis**: Extract structured knowledge from text
- **Agent memory**: Temporary knowledge graphs for sessions
- **Research exploration**: Discover connections in data

## Technical Details

- **Language**: Go (90.5%), Python SDK (8.0%)
- **License**: MIT
- **Repository**: [gibram-io/gibram](https://github.com/gibram-io/gibram)
- **Documentation**: [docs.gibram.io](https://docs.gibram.io)

## Links

- [GitHub Repository](https://github.com/gibram-io/gibram)
- [Documentation](https://docs.gibram.io)

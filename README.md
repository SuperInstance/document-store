# Document Store

**A lightweight document database with field-level indexing** that stores JSON documents and supports indexed lookups — a miniature MongoDB-style store demonstrating the core concepts behind document databases.

## Why It Matters

Document databases (MongoDB, CouchDB, DynamoDB) are the dominant data store for modern web applications. Unlike relational databases with fixed schemas, document stores accept flexible JSON-like documents and index specific fields for fast retrieval.

This library implements the essential primitives:
- **Collection** — A `HashMap<String, JSON>` mapping document IDs to JSON values
- **Secondary indexes** — `HashMap<field, HashMap<value, Vec<doc_id>>>` enabling O(1) lookup by indexed field value
- **Automatic index maintenance** — When a document is inserted, all registered indexes are updated automatically

**The indexing trade-off:** Without an index, finding all documents where `type = "article"` requires scanning every document — O(n). With an index on the `type` field, it's O(1) hash lookup. The cost is additional storage (the index structure) and slower writes (must update all indexes on insert). This is the fundamental trade-off in database design.

**Real-world relevance:** This demonstrates exactly how MongoDB's secondary indexes work. When you `db.collection.createIndex({type: 1})` in MongoDB, it builds the same kind of inverted index. The difference is that MongoDB uses B-trees (for range queries) rather than hash maps (exact match only).

## How It Works

The `DocumentStore` maintains two data structures:

**Primary storage:** `HashMap<String, serde_json::Value>` — The document collection keyed by document ID. This is the source of truth; all data lives here.

**Index map:** `HashMap<String, HashMap<String, Vec<String>>>` — A nested map of field_name → (field_value → [doc_ids]). This is the inverted index that enables fast lookups.

**Insertion flow:**
1. `insert(id, doc)` is called
2. The `indexes_dirty` method scans the document for any fields that have registered indexes
3. For each indexed field present in the document, the doc ID is appended to the appropriate index bucket
4. The document is stored in the collection

**Query flow:**
1. `find_by_index(field, value)` looks up `indexes[field][value]` to get a list of matching doc IDs
2. Each ID is resolved to its document in the collection
3. Returns references to all matching documents

This is a direct implementation of an **inverted index** — the same data structure used by Elasticsearch, Lucene, and search engines, albeit simplified to exact string matching rather than full-text tokenization.

## Quick Start

```rust
use document_store::DocumentStore;

let mut store = DocumentStore::new();

// Create an index on the "type" field
store.create_index("type");

// Insert documents
store.insert("doc1", serde_json::json!({
    "type": "article",
    "title": "Getting Started with Rust",
    "author": "Alice"
}));
store.insert("doc2", serde_json::json!({
    "type": "article",
    "title": "Advanced Async Programming",
    "author": "Bob"
}));
store.insert("doc3", serde_json::json!({
    "type": "tutorial",
    "title": "Build a CLI Tool"
}));

// Query by indexed field — O(1) lookup
let articles = store.find_by_index("type", "article");
println!("Found {} articles", articles.len()); // 2

// Direct lookup by ID
if let Some(doc) = store.get("doc1") {
    println!("Title: {}", doc["title"]);
}
```

## API

### `DocumentStore`
- `new() -> Self` — Create empty store
- `create_index(&mut self, field: &str)` — Register a secondary index on a field. O(1)
- `insert(&mut self, id: &str, doc: Value)` — Insert document, auto-updating indexes. O(k) where k = indexed fields present
- `get(&self, id: &str) -> Option<&Value>` — Direct lookup by ID. O(1)
- `find_by_index(field, value) -> Vec<&Value>` — Find all documents where field == value. O(1) + O(m) where m = matches
- `doc_count() -> usize` — Number of stored documents

## Architecture Notes

This library provides the document storage abstraction for SuperInstance's data layer, demonstrating the indexing principles used by production document databases. It serves as the teaching foundation for understanding MongoDB, CouchDB, and DynamoDB index strategies.

See the full architecture: [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md)

## License

MIT

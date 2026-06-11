use std::collections::HashMap;

/// Document store with indexing support
struct DocumentStore {
    collection: HashMap<String, serde_json::Value>,
    indexes: HashMap<String, HashMap<String, Vec<String>>>,
}

impl DocumentStore {
    fn new() -> Self {
        Self { collection: HashMap::new(), indexes: HashMap::new() }
    }

    fn insert(&mut self, id: &str, doc: serde_json::Value) {
        if let Some(refresh) = self.indexes_dirty(&id, &doc) {
            for (field, val) in refresh {
                if let Some(idx) = self.indexes.get_mut(&field) {
                    idx.entry(val).or_default().push(id.to_string());
                }
            }
        }
        self.collection.insert(id.to_string(), doc);
    }

    fn get(&self, id: &str) -> Option<&serde_json::Value> {
        self.collection.get(id)
    }

    fn create_index(&mut self, field: &str) {
        self.indexes.insert(field.to_string(), HashMap::new());
    }

    fn find_by_index(&self, field: &str, value: &str) -> Vec<&serde_json::Value> {
        self.indexes.get(field)
            .and_then(|idx| idx.get(value))
            .map(|ids| ids.iter().filter_map(|id| self.collection.get(id)).collect())
            .unwrap_or_default()
    }

    fn indexes_dirty(&self, id: &str, doc: &serde_json::Value) -> Option<Vec<(String, String)>> {
        let mut result = vec![];
        for field in self.indexes.keys() {
            if let Some(val) = doc.get(field) {
                if let Some(s) = val.as_str() {
                    result.push((field.clone(), s.to_string()));
                }
            }
        }
        if result.is_empty() { None } else { Some(result) }
    }

    fn doc_count(&self) -> usize { self.collection.len() }
}

fn main() {
    let mut store = DocumentStore::new();
    store.create_index("type");
    store.insert("doc1", serde_json::json!({"type": "article", "title": "Hello"}));
    store.insert("doc2", serde_json::json!({"type": "article", "title": "World"}));
    println!("Docs: {}", store.doc_count());
    println!("Find articles: {:?}", store.find_by_index("type", "article"));
}

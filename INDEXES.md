# Database Indexes

```sql
CREATE EXTENSION IF NOT EXISTS vector;
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE TABLE vectors (
   id SERIAL PRIMARY KEY,
   tenant VARCHAR(255) NOT NULL,
   vector_id VARCHAR(255) NOT NULL,
   embedding VECTOR(1024),
   content TEXT,
   metadata JSONB,
   search_vector tsvector GENERATED ALWAYS AS (to_tsvector('simple', COALESCE(content, ''))) STORED,
   created_at TIMESTAMP DEFAULT NOW(),
   UNIQUE(tenant, vector_id)
);

CREATE INDEX idx_vectors_tenant_vector_id ON vectors (tenant, vector_id);
CREATE INDEX idx_vectors_embedding ON vectors USING hnsw (embedding vector_cosine_ops);
CREATE INDEX idx_vectors_metadata ON vectors USING gin (metadata);
CREATE INDEX idx_vectors_search_vector ON vectors USING gin (search_vector);
CREATE INDEX idx_vectors_content_trgm ON vectors USING gin (content gin_trgm_ops);
```

# Skill: Database Architecture

**Name**: database
**Description**: Schema-driven database patterns with QueryBuilder
**Trigger**: Database operations, schema changes, migrations, queries

---

## Objective

All database operations use schema.rs and QueryBuilder - NO hardcoded SQL strings.

## Architecture Overview

```
src/db/
  schema.rs           # Single source of truth for table/column definitions
  query_builder.rs    # Fluent API for SQL construction
  repository.rs       # Base repository pattern
  migrations.rs       # Version-controlled schema changes (uses QueryBuilder)
  *_repository.rs     # Entity-specific data access
```

## Core Rules

1. **Schema is Truth**: All table/column definitions in `schema.rs`
2. **QueryBuilder for Queries**: Never write raw SQL strings
3. **Repository Pattern**: Each entity has its own repository
4. **Migrations via Schema**: Use `table.create_table_sql()`, never hardcode DDL

## Correct Patterns

```rust
// GOOD: Using QueryBuilder
let sql = QueryBuilder::select(&TOOLS_TABLE)
    .columns(&["id", "display_name"])
    .where_eq("enabled")
    .order_by("display_name", true)
    .build();

// GOOD: Schema-driven DDL
let ddl = TOOLS_TABLE.create_table_sql();
```

## Anti-Patterns to Avoid

```rust
// BAD: Hardcoded SQL
db.execute("SELECT * FROM tools WHERE id = ?", [id]).await?;

// BAD: Hardcoded DDL
db.execute("CREATE TABLE tools (id TEXT PRIMARY KEY)", ()).await?;

// BAD: SQL in migrations
db.execute("INSERT INTO schema_migrations...", params).await?;
```

## Why This Matters

- Single point of maintenance for schema
- Compile-time verification of column names
- Consistent query patterns across codebase
- Easier migrations and schema evolution

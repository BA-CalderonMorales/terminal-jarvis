// Query Builder
//
// Fluent API for constructing SQL queries from schema definitions.
// Eliminates hardcoded SQL strings throughout the codebase.
//
// Usage:
//   QueryBuilder::select(&TOOLS_TABLE)
//       .columns(&["id", "display_name"])
//       .where_eq("enabled", "1")
//       .order_by("display_name", true)
//       .build()

use super::schema::Table;
use std::fmt::Write;

/// Query operation type
#[derive(Debug, Clone, Copy)]
pub enum QueryType {
    Select,
    Insert,
    Update,
    Delete,
    Upsert,
}

/// WHERE clause condition
#[derive(Debug, Clone)]
pub struct WhereClause {
    column: String,
    operator: String,
    placeholder: bool, // true = use ?, false = use literal value
    value: Option<String>,
}

/// ORDER BY clause
#[derive(Debug, Clone)]
pub struct OrderBy {
    column: String,
    ascending: bool,
}

/// Query builder for type-safe SQL construction
#[derive(Debug, Clone)]
pub struct QueryBuilder {
    query_type: QueryType,
    table_name: String,
    columns: Vec<String>,
    where_clauses: Vec<WhereClause>,
    order_by: Vec<OrderBy>,
    limit: Option<usize>,
    offset: Option<usize>,
    #[allow(dead_code)] // Reserved for future INSERT value binding
    values: Vec<String>,
    on_conflict: Option<String>,
}

impl QueryBuilder {
    // =========================================================================
    // Factory methods
    // =========================================================================

    /// Start a SELECT query
    pub fn select(table: &Table) -> Self {
        Self {
            query_type: QueryType::Select,
            table_name: table.name.to_string(),
            columns: table.column_names().iter().map(|s| s.to_string()).collect(),
            where_clauses: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
            values: Vec::new(),
            on_conflict: None,
        }
    }

    /// Start an INSERT query
    pub fn insert(table: &Table) -> Self {
        Self {
            query_type: QueryType::Insert,
            table_name: table.name.to_string(),
            columns: Vec::new(),
            where_clauses: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
            values: Vec::new(),
            on_conflict: None,
        }
    }

    /// Start an UPDATE query
    pub fn update(table: &Table) -> Self {
        Self {
            query_type: QueryType::Update,
            table_name: table.name.to_string(),
            columns: Vec::new(),
            where_clauses: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
            values: Vec::new(),
            on_conflict: None,
        }
    }

    /// Start a DELETE query
    pub fn delete(table: &Table) -> Self {
        Self {
            query_type: QueryType::Delete,
            table_name: table.name.to_string(),
            columns: Vec::new(),
            where_clauses: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
            values: Vec::new(),
            on_conflict: None,
        }
    }

    /// Start an UPSERT (INSERT OR REPLACE) query
    pub fn upsert(table: &Table) -> Self {
        let mut builder = Self::insert(table);
        builder.query_type = QueryType::Upsert;
        builder
    }

    /// Start a COUNT query (SELECT COUNT(*))
    pub fn count(table: &Table) -> Self {
        Self {
            query_type: QueryType::Select,
            table_name: table.name.to_string(),
            columns: vec!["COUNT(*)".to_string()],
            where_clauses: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
            values: Vec::new(),
            on_conflict: None,
        }
    }

    // =========================================================================
    // Builder methods
    // =========================================================================

    /// Specify columns for SELECT or INSERT
    pub fn columns(mut self, cols: &[&str]) -> Self {
        self.columns = cols.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Add a column-value pair for INSERT/UPDATE
    pub fn set(mut self, column: &str, _value_placeholder: bool) -> Self {
        self.columns.push(column.to_string());
        self
    }

    /// Add WHERE column = ? clause (parameterized)
    pub fn where_eq(mut self, column: &str) -> Self {
        self.where_clauses.push(WhereClause {
            column: column.to_string(),
            operator: "=".to_string(),
            placeholder: true,
            value: None,
        });
        self
    }

    /// Add WHERE column = literal clause
    pub fn where_eq_literal(mut self, column: &str, value: &str) -> Self {
        self.where_clauses.push(WhereClause {
            column: column.to_string(),
            operator: "=".to_string(),
            placeholder: false,
            value: Some(value.to_string()),
        });
        self
    }

    /// Add WHERE column IS NULL clause
    pub fn where_null(mut self, column: &str) -> Self {
        self.where_clauses.push(WhereClause {
            column: column.to_string(),
            operator: "IS NULL".to_string(),
            placeholder: false,
            value: None,
        });
        self
    }

    /// Add WHERE column IS NOT NULL clause
    pub fn where_not_null(mut self, column: &str) -> Self {
        self.where_clauses.push(WhereClause {
            column: column.to_string(),
            operator: "IS NOT NULL".to_string(),
            placeholder: false,
            value: None,
        });
        self
    }

    /// Add ORDER BY clause
    pub fn order_by(mut self, column: &str, ascending: bool) -> Self {
        self.order_by.push(OrderBy {
            column: column.to_string(),
            ascending,
        });
        self
    }

    /// Add LIMIT clause
    pub fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }

    /// Add OFFSET clause
    pub fn offset(mut self, n: usize) -> Self {
        self.offset = Some(n);
        self
    }

    /// Add ON CONFLICT clause for UPSERT with single conflict column
    pub fn on_conflict_update(self, conflict_column: &str, update_columns: &[&str]) -> Self {
        self.on_conflict_update_composite(&[conflict_column], update_columns)
    }

    /// Add ON CONFLICT clause for UPSERT with composite key (multiple conflict columns)
    pub fn on_conflict_update_composite(
        mut self,
        conflict_columns: &[&str],
        update_columns: &[&str],
    ) -> Self {
        let updates: Vec<String> = update_columns
            .iter()
            .map(|c| format!("{c} = excluded.{c}"))
            .collect();

        self.on_conflict = Some(format!(
            "ON CONFLICT({}) DO UPDATE SET {}",
            conflict_columns.join(", "),
            updates.join(", ")
        ));
        self
    }

    /// Add ON CONFLICT DO NOTHING with single column
    pub fn on_conflict_ignore(self, conflict_column: &str) -> Self {
        self.on_conflict_ignore_composite(&[conflict_column])
    }

    /// Add ON CONFLICT DO NOTHING with composite key
    pub fn on_conflict_ignore_composite(mut self, conflict_columns: &[&str]) -> Self {
        self.on_conflict = Some(format!(
            "ON CONFLICT({}) DO NOTHING",
            conflict_columns.join(", ")
        ));
        self
    }

    // =========================================================================
    // Build methods
    // =========================================================================

    /// Build the final SQL string
    pub fn build(&self) -> String {
        match self.query_type {
            QueryType::Select => self.build_select(),
            QueryType::Insert => self.build_insert(),
            QueryType::Update => self.build_update(),
            QueryType::Delete => self.build_delete(),
            QueryType::Upsert => self.build_insert(), // Same as INSERT but with ON CONFLICT
        }
    }

    /// Get the number of parameter placeholders in the query
    pub fn param_count(&self) -> usize {
        let where_params = self.where_clauses.iter().filter(|w| w.placeholder).count();

        match self.query_type {
            QueryType::Select | QueryType::Delete => where_params,
            QueryType::Insert | QueryType::Upsert => self.columns.len(),
            QueryType::Update => self.columns.len() + where_params,
        }
    }

    fn build_select(&self) -> String {
        let mut sql = String::new();

        write!(
            sql,
            "SELECT {} FROM {}",
            self.columns.join(", "),
            self.table_name
        )
        .unwrap();

        self.append_where(&mut sql);
        self.append_order_by(&mut sql);
        self.append_limit_offset(&mut sql);

        sql
    }

    fn build_insert(&self) -> String {
        let mut sql = String::new();
        let placeholders: Vec<&str> = vec!["?"; self.columns.len()];

        write!(
            sql,
            "INSERT INTO {} ({}) VALUES ({})",
            self.table_name,
            self.columns.join(", "),
            placeholders.join(", ")
        )
        .unwrap();

        if let Some(ref on_conflict) = self.on_conflict {
            write!(sql, " {on_conflict}").unwrap();
        }

        sql
    }

    fn build_update(&self) -> String {
        let mut sql = String::new();
        let sets: Vec<String> = self.columns.iter().map(|c| format!("{c} = ?")).collect();

        write!(sql, "UPDATE {} SET {}", self.table_name, sets.join(", ")).unwrap();

        self.append_where(&mut sql);

        sql
    }

    fn build_delete(&self) -> String {
        let mut sql = String::new();

        write!(sql, "DELETE FROM {}", self.table_name).unwrap();

        self.append_where(&mut sql);

        sql
    }

    fn append_where(&self, sql: &mut String) {
        if self.where_clauses.is_empty() {
            return;
        }

        sql.push_str(" WHERE ");

        let conditions: Vec<String> = self
            .where_clauses
            .iter()
            .map(|w| {
                if w.placeholder {
                    format!("{} {} ?", w.column, w.operator)
                } else if let Some(ref val) = w.value {
                    format!("{} {} {}", w.column, w.operator, val)
                } else {
                    format!("{} {}", w.column, w.operator)
                }
            })
            .collect();

        sql.push_str(&conditions.join(" AND "));
    }

    fn append_order_by(&self, sql: &mut String) {
        if self.order_by.is_empty() {
            return;
        }

        sql.push_str(" ORDER BY ");

        let orders: Vec<String> = self
            .order_by
            .iter()
            .map(|o| {
                if o.ascending {
                    o.column.clone()
                } else {
                    format!("{} DESC", o.column)
                }
            })
            .collect();

        sql.push_str(&orders.join(", "));
    }

    fn append_limit_offset(&self, sql: &mut String) {
        if let Some(limit) = self.limit {
            write!(sql, " LIMIT {limit}").unwrap();
        }

        if let Some(offset) = self.offset {
            write!(sql, " OFFSET {offset}").unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::core::schema::{PREFERENCES_TABLE, TOOLS_TABLE};

    #[test]
    fn test_simple_select() {
        let sql = QueryBuilder::select(&PREFERENCES_TABLE)
            .columns(&["key", "value"])
            .build();

        assert_eq!(sql, "SELECT key, value FROM preferences");
    }

    #[test]
    fn test_select_with_where() {
        let sql = QueryBuilder::select(&TOOLS_TABLE)
            .columns(&["id", "display_name"])
            .where_eq("id")
            .build();

        assert_eq!(sql, "SELECT id, display_name FROM tools WHERE id = ?");
    }

    #[test]
    fn test_select_with_order_and_limit() {
        let sql = QueryBuilder::select(&TOOLS_TABLE)
            .columns(&["id"])
            .where_eq_literal("enabled", "1")
            .order_by("display_name", true)
            .limit(10)
            .build();

        assert_eq!(
            sql,
            "SELECT id FROM tools WHERE enabled = 1 ORDER BY display_name LIMIT 10"
        );
    }

    #[test]
    fn test_insert() {
        let sql = QueryBuilder::insert(&PREFERENCES_TABLE)
            .columns(&["key", "value"])
            .build();

        assert_eq!(sql, "INSERT INTO preferences (key, value) VALUES (?, ?)");
    }

    #[test]
    fn test_upsert() {
        let sql = QueryBuilder::upsert(&PREFERENCES_TABLE)
            .columns(&["key", "value"])
            .on_conflict_update("key", &["value"])
            .build();

        assert_eq!(
            sql,
            "INSERT INTO preferences (key, value) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value"
        );
    }

    #[test]
    fn test_update() {
        let sql = QueryBuilder::update(&TOOLS_TABLE)
            .columns(&["enabled"])
            .where_eq("id")
            .build();

        assert_eq!(sql, "UPDATE tools SET enabled = ? WHERE id = ?");
    }

    #[test]
    fn test_delete() {
        let sql = QueryBuilder::delete(&PREFERENCES_TABLE)
            .where_eq("key")
            .build();

        assert_eq!(sql, "DELETE FROM preferences WHERE key = ?");
    }

    #[test]
    fn test_param_count() {
        let builder =
            QueryBuilder::insert(&PREFERENCES_TABLE).columns(&["key", "value", "updated_at"]);

        assert_eq!(builder.param_count(), 3);
    }
}

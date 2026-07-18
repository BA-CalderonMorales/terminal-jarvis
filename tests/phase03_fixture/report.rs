use super::Row;

pub struct Report {
    pub tested_ref: String,
    pub rows: Vec<Row>,
}

impl Report {
    pub(super) fn new(mut rows: Vec<Row>) -> Self {
        rows.sort_by(|left, right| {
            (&left.harness, &left.capability).cmp(&(&right.harness, &right.capability))
        });
        let tested_ref =
            std::env::var("TJ_PHASE03_TESTED_REF").unwrap_or_else(|_| "working-tree".to_string());
        Self { tested_ref, rows }
    }

    pub fn tsv(&self) -> String {
        let header = "schema_version\ttested_ref\tharness\tcapability\tsupport\t\
                      evidence\tguard\targv\teffect\tplatforms\texecutable\t\
                      source\tverified_at\tsummary\tresult";
        let mut lines = vec![header.to_string()];
        lines.extend(self.rows.iter().map(|row| row.tsv(&self.tested_ref)));
        format!("{}\n", lines.join("\n"))
    }
}

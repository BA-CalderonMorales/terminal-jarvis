use super::Report;

impl Report {
    pub fn exit_code(&self) -> i32 {
        if self.ok {
            0
        } else {
            4
        }
    }

    pub fn plain(&self) -> String {
        super::render::plain(self)
    }

    pub fn json_data(&self) -> String {
        super::json::data(self)
    }

    pub fn json(&self) -> String {
        super::json::full(self)
    }

    pub fn concise(&self) -> Self {
        let records = self
            .records
            .iter()
            .filter(|record| {
                !record.key.starts_with("harness.")
                    || record.key == "harness.active"
                    || record.key.ends_with(".readiness")
                    || record.severity != super::Severity::Info
            })
            .cloned()
            .collect();
        Self {
            records,
            ready_harnesses: self.ready_harnesses,
            ok: self.ok,
        }
    }
}

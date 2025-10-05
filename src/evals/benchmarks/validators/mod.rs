// Validators Module
pub mod pattern_match;

// Re-export validator types (Phase 2 API - intentionally unused in Phase 1)
#[allow(unused_imports)]
pub use pattern_match::PatternMatchValidator;

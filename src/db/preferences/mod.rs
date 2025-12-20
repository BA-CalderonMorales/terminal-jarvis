// Preferences Domain Module
//
// User preferences storage and retrieval.
// Organized as key-value pairs with typed accessors.

mod keys;
mod repository;

pub use keys::PreferenceKeys;
pub use repository::PreferencesRepository;

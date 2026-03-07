use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Language '{0}' not found")]
    LanguageNotFound(String),

    #[error("Dynamic library load error: {0}")]
    DynamicLoad(String),

    #[error("Language function returned null pointer for '{0}'")]
    NullLanguagePointer(String),

    #[error("Failed to set parser language: {0}")]
    ParserSetup(String),

    #[error("Registry lock poisoned: {0}")]
    LockPoisoned(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[cfg(feature = "config")]
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[cfg(feature = "config")]
    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),
}

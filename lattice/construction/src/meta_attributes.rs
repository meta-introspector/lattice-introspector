//! This module defines structures for representing meta-attributes like emoji vectors and prime blessings.

use serde::{Serialize, Deserialize};

/// Represents a single emoji blessing with its associated prime number and vibe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmojiVibe {
    pub emoji: String,
    pub prime: u64,
    pub vibe: String,
}

// This module will eventually contain generated static data for all blessed emojis.

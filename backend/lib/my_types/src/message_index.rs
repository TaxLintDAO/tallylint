use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(
    CandidType,
    Serialize,
    Deserialize,
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct MessageIndex(u32);

impl MessageIndex {
    pub fn incr(&self) -> MessageIndex {
        MessageIndex(self.0.saturating_add(1))
    }
}

impl From<u32> for MessageIndex {
    fn from(val: u32) -> Self {
        MessageIndex(val)
    }
}

impl From<MessageIndex> for u32 {
    fn from(message_index: MessageIndex) -> Self {
        message_index.0
    }
}

impl From<MessageIndex> for usize {
    fn from(message_index: MessageIndex) -> Self {
        message_index.0.try_into().unwrap()
    }
}

impl Display for MessageIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

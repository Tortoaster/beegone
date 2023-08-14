use std::sync::atomic::{AtomicU32, Ordering};

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

static COUNTER: AtomicU32 = AtomicU32::new(0);

#[typeshare]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id(u32);

impl Id {
    pub fn new() -> Self {
        Id(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

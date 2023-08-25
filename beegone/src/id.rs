use std::{
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicU32, Ordering},
};

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

static COUNTER: AtomicU32 = AtomicU32::new(0);

#[typeshare]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id(u32);

#[typeshare]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct WithId<T> {
    id: Id,
    inner: T,
}

impl<T> Deref for WithId<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for WithId<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub trait IdExt: Sized {
    fn with_id(self) -> WithId<Self>;
}

impl<T> IdExt for T {
    fn with_id(self) -> WithId<Self> {
        WithId {
            id: Id(COUNTER.fetch_add(1, Ordering::Relaxed)),
            inner: self,
        }
    }
}

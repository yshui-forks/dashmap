use crate::lock::{RwLockReadGuardDetached, RwLockWriteGuardDetached};
use core::hash::Hash;
use core::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct RefMulti<'a, K, V: ?Sized> {
    pub(super) _guard: Arc<RwLockReadGuardDetached<'a>>,
    pub(super) k: &'a K,
    pub(super) v: &'a V,
}

impl<'a, K: Eq + Hash, V: ?Sized> RefMulti<'a, K, V> {
    pub(crate) fn new(guard: Arc<RwLockReadGuardDetached<'a>>, k: &'a K, v: &'a V) -> Self {
        Self {
            _guard: guard,
            k,
            v,
        }
    }

    pub fn key(&self) -> &K {
        self.pair().0
    }

    pub fn value(&self) -> &V {
        self.pair().1
    }

    pub fn pair(&self) -> (&K, &V) {
        (self.k, self.v)
    }
}

impl<'a, K: Eq + Hash, V: ?Sized> Deref for RefMulti<'a, K, V> {
    type Target = V;

    fn deref(&self) -> &V {
        self.value()
    }
}

pub struct RefMutMulti<'a, K, V: ?Sized> {
    pub(super) _guard: Arc<RwLockWriteGuardDetached<'a>>,
    pub(super) k: &'a K,
    pub(super) v: &'a mut V,
}

impl<'a, K: Eq + Hash, V: ?Sized> RefMutMulti<'a, K, V> {
    pub(crate) fn new(guard: Arc<RwLockWriteGuardDetached<'a>>, k: &'a K, v: &'a mut V) -> Self {
        Self {
            _guard: guard,
            k,
            v,
        }
    }

    pub fn key(&self) -> &K {
        self.pair().0
    }

    pub fn value(&self) -> &V {
        self.pair().1
    }

    pub fn value_mut(&mut self) -> &mut V {
        self.pair_mut().1
    }

    pub fn pair(&self) -> (&K, &V) {
        (self.k, self.v)
    }

    pub fn pair_mut(&mut self) -> (&K, &mut V) {
        (self.k, self.v)
    }
}

impl<'a, K: Eq + Hash, V: ?Sized> Deref for RefMutMulti<'a, K, V> {
    type Target = V;

    fn deref(&self) -> &V {
        self.value()
    }
}

impl<'a, K: Eq + Hash, V: ?Sized> DerefMut for RefMutMulti<'a, K, V> {
    fn deref_mut(&mut self) -> &mut V {
        self.value_mut()
    }
}

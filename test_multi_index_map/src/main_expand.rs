#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use crate::inner::MultiIndexOrderMap;
use crate::inner::Order;
mod inner {
    use multi_index_map::MultiIndexMap;
    #[multi_index_derive(Clone, Debug)]
    pub(crate) struct Order {
        #[multi_index(hashed_unique)]
        pub(crate) order_id: u32,
        #[multi_index(ordered_unique)]
        pub(crate) timestamp: u64,
        #[multi_index(hashed_non_unique)]
        pub(crate) trader_name: String,
        pub(crate) note: String,
    }
    pub(crate) struct MultiIndexOrderMap {
        _store: ::multi_index_map::slab::Slab<Order>,
        _order_id_index: ::std::collections::HashMap<
            u32,
            usize,
            ::multi_index_map::rustc_hash::FxBuildHasher,
        >,
        _timestamp_index: ::std::collections::BTreeMap<u64, usize>,
        _trader_name_index: ::std::collections::HashMap<
            String,
            ::std::collections::BTreeSet<usize>,
            ::multi_index_map::rustc_hash::FxBuildHasher,
        >,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MultiIndexOrderMap {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "MultiIndexOrderMap",
                "_store",
                &self._store,
                "_order_id_index",
                &self._order_id_index,
                "_timestamp_index",
                &self._timestamp_index,
                "_trader_name_index",
                &&self._trader_name_index,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MultiIndexOrderMap {
        #[inline]
        fn clone(&self) -> MultiIndexOrderMap {
            MultiIndexOrderMap {
                _store: ::core::clone::Clone::clone(&self._store),
                _order_id_index: ::core::clone::Clone::clone(&self._order_id_index),
                _timestamp_index: ::core::clone::Clone::clone(&self._timestamp_index),
                _trader_name_index: ::core::clone::Clone::clone(&self._trader_name_index),
            }
        }
    }
    impl Default for MultiIndexOrderMap {
        fn default() -> Self {
            Self {
                _store: ::multi_index_map::slab::Slab::default(),
                _order_id_index: ::std::collections::HashMap::default(),
                _timestamp_index: ::std::collections::BTreeMap::new(),
                _trader_name_index: ::std::collections::HashMap::default(),
            }
        }
    }
    impl MultiIndexOrderMap {
        pub(crate) fn with_capacity(n: usize) -> Self {
            Self {
                _store: ::multi_index_map::slab::Slab::with_capacity(n),
                _order_id_index: ::std::collections::HashMap::default(),
                _timestamp_index: ::std::collections::BTreeMap::new(),
                _trader_name_index: ::std::collections::HashMap::default(),
            }
        }
        pub(crate) fn capacity(&self) -> usize {
            self._store.capacity()
        }
        pub(crate) fn len(&self) -> usize {
            self._store.len()
        }
        pub(crate) fn is_empty(&self) -> bool {
            self._store.is_empty()
        }
        pub(crate) fn reserve(&mut self, additional: usize) {
            self._store.reserve(additional);
            self._order_id_index.reserve(additional);
            self._trader_name_index.reserve(additional);
        }
        pub(crate) fn shrink_to_fit(&mut self) {
            self._store.shrink_to_fit();
            self._order_id_index.shrink_to_fit();
            self._trader_name_index.shrink_to_fit();
        }
        pub(crate) fn try_insert(
            &mut self,
            elem: Order,
        ) -> Result<&Order, ::multi_index_map::UniquenessError<Order>> {
            let store_entry = self._store.vacant_entry();
            let idx = store_entry.key();
            let order_id_entry = match self._order_id_index.entry(elem.order_id.clone())
            {
                ::std::collections::hash_map::Entry::Occupied(_) => {
                    return Err(::multi_index_map::UniquenessError(elem));
                }
                ::std::collections::hash_map::Entry::Vacant(e) => e,
            };
            let timestamp_entry = match self
                ._timestamp_index
                .entry(elem.timestamp.clone())
            {
                ::std::collections::btree_map::Entry::Occupied(_) => {
                    return Err(::multi_index_map::UniquenessError(elem));
                }
                ::std::collections::btree_map::Entry::Vacant(e) => e,
            };
            order_id_entry.insert(idx);
            timestamp_entry.insert(idx);
            self._trader_name_index
                .entry(elem.trader_name.clone())
                .or_insert(::std::collections::BTreeSet::new())
                .insert(idx);
            let elem = store_entry.insert(elem);
            Ok(elem)
        }
        pub(crate) fn insert(&mut self, elem: Order) -> &Order {
            self.try_insert(elem).expect("Unable to insert element")
        }
        pub(crate) fn clear(&mut self) {
            self._store.clear();
            self._order_id_index.clear();
            self._timestamp_index.clear();
            self._trader_name_index.clear();
        }
        pub(crate) fn iter(&self) -> ::multi_index_map::slab::Iter<Order> {
            self._store.iter()
        }
        /// SAFETY:
        /// It is safe to mutate the non-indexed fields,
        /// however mutating any of the indexed fields will break the internal invariants.
        /// If the indexed fields need to be changed, the modify() method must be used.
        pub(crate) fn iter_mut<'__mim_iter_lifetime>(
            &'__mim_iter_lifetime mut self,
        ) -> OrderIterMut<'__mim_iter_lifetime> {
            OrderIterMut(self._store.iter_mut())
        }
        pub(crate) fn get_by_order_id<__MultiIndexMapKeyType>(
            &self,
            key: &__MultiIndexMapKeyType,
        ) -> Option<&Order>
        where
            u32: ::std::borrow::Borrow<__MultiIndexMapKeyType>,
            __MultiIndexMapKeyType: ::std::hash::Hash + Eq + ?Sized,
        {
            Some(&self._store[*self._order_id_index.get(key)?])
        }
        pub(crate) fn get_mut_by_order_id(
            &mut self,
            key: &u32,
        ) -> Option<(&mut String,)> {
            let elem = &mut self._store[*self._order_id_index.get(key)?];
            Some((&mut elem.note,))
        }
        pub(crate) fn remove_by_order_id(&mut self, key: &u32) -> Option<Order> {
            let idx = self._order_id_index.remove(key)?;
            let elem_orig = self._store.remove(idx);
            let _removed_elem = self._order_id_index.remove(&elem_orig.order_id);
            let _removed_elem = self._timestamp_index.remove(&elem_orig.timestamp);
            let key_to_remove = &elem_orig.trader_name;
            if let Some(elems) = self._trader_name_index.get_mut(key_to_remove) {
                if elems.len() > 1 {
                    if !elems.remove(&idx) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "Internal invariants broken, unable to find element in index \'field_name\' despite being present in another",
                                ),
                            );
                        };
                    }
                } else {
                    self._trader_name_index.remove(key_to_remove);
                }
            }
            Some(elem_orig)
        }
        pub(crate) fn modify_by_order_id(
            &mut self,
            key: &u32,
            f: impl FnOnce(&mut Order),
        ) -> Option<&Order> {
            let idx = *self._order_id_index.get(key)?;
            let elem = &mut self._store[idx];
            let order_id_orig = elem.order_id.clone();
            let timestamp_orig = elem.timestamp.clone();
            let trader_name_orig = elem.trader_name.clone();
            f(elem);
            if elem.order_id != order_id_orig {
                let idx = self
                    ._order_id_index
                    .remove(&order_id_orig)
                    .expect(
                        "Internal invariants broken, unable to find element in index 'field_name' despite being present in another",
                    );
                let orig_elem_idx = self
                    ._order_id_index
                    .insert(elem.order_id.clone(), idx);
                if orig_elem_idx.is_some() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "Unable to insert element, uniqueness constraint violated on field \'{0}\'",
                                "field_name",
                            ),
                        );
                    };
                }
            }
            if elem.timestamp != timestamp_orig {
                let idx = self
                    ._timestamp_index
                    .remove(&timestamp_orig)
                    .expect(
                        "Internal invariants broken, unable to find element in index 'field_name' despite being present in another",
                    );
                let orig_elem_idx = self
                    ._timestamp_index
                    .insert(elem.timestamp.clone(), idx);
                if orig_elem_idx.is_some() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "Unable to insert element, uniqueness constraint violated on field \'{0}\'",
                                "field_name",
                            ),
                        );
                    };
                }
            }
            if elem.trader_name != trader_name_orig {
                let idxs = self
                    ._trader_name_index
                    .get_mut(&trader_name_orig)
                    .expect(
                        "Internal invariants broken, unable to find element in index 'field_name' despite being present in another",
                    );
                if idxs.len() > 1 {
                    if !(idxs.remove(&idx)) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "Internal invariants broken, unable to find element in index \'field_name\' despite being present in another",
                                ),
                            );
                        };
                    }
                } else {
                    self._trader_name_index.remove(&trader_name_orig);
                }
                self._trader_name_index
                    .entry(elem.trader_name.clone())
                    .or_insert(::std::collections::BTreeSet::new())
                    .insert(idx);
            }
            Some(elem)
        }
        pub(crate) fn update_by_order_id<__MultiIndexMapKeyType>(
            &mut self,
            key: &__MultiIndexMapKeyType,
            f: impl FnOnce(&mut String),
        ) -> Option<&Order>
        where
            u32: ::std::borrow::Borrow<__MultiIndexMapKeyType>,
            __MultiIndexMapKeyType: ::std::hash::Hash + Eq + ?Sized,
        {
            let idx = *self._order_id_index.get(key)?;
            let elem = &mut self._store[idx];
            f(&mut elem.note);
            Some(elem)
        }
        pub(crate) fn iter_by_order_id<'__mim_iter_lifetime>(
            &'__mim_iter_lifetime self,
        ) -> MultiIndexOrderMapOrderIdIter<'__mim_iter_lifetime> {
            MultiIndexOrderMapOrderIdIter {
                _store_ref: &self._store,
                _iter: self._order_id_index.iter(),
                _inner_iter: None,
            }
        }
        pub(crate) fn get_by_timestamp<__MultiIndexMapKeyType>(
            &self,
            key: &__MultiIndexMapKeyType,
        ) -> Option<&Order>
        where
            u64: ::std::borrow::Borrow<__MultiIndexMapKeyType>,
            __MultiIndexMapKeyType: Ord + ?Sized,
        {
            Some(&self._store[*self._timestamp_index.get(key)?])
        }
        pub(crate) fn get_mut_by_timestamp(
            &mut self,
            key: &u64,
        ) -> Option<(&mut String,)> {
            let elem = &mut self._store[*self._timestamp_index.get(key)?];
            Some((&mut elem.note,))
        }
        pub(crate) fn remove_by_timestamp(&mut self, key: &u64) -> Option<Order> {
            let idx = self._timestamp_index.remove(key)?;
            let elem_orig = self._store.remove(idx);
            let _removed_elem = self._order_id_index.remove(&elem_orig.order_id);
            let _removed_elem = self._timestamp_index.remove(&elem_orig.timestamp);
            let key_to_remove = &elem_orig.trader_name;
            if let Some(elems) = self._trader_name_index.get_mut(key_to_remove) {
                if elems.len() > 1 {
                    if !elems.remove(&idx) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "Internal invariants broken, unable to find element in index \'field_name\' despite being present in another",
                                ),
                            );
                        };
                    }
                } else {
                    self._trader_name_index.remove(key_to_remove);
                }
            }
            Some(elem_orig)
        }
        pub(crate) fn modify_by_timestamp(
            &mut self,
            key: &u64,
            f: impl FnOnce(&mut Order),
        ) -> Option<&Order> {
            let idx = *self._timestamp_index.get(key)?;
            let elem = &mut self._store[idx];
            let order_id_orig = elem.order_id.clone();
            let timestamp_orig = elem.timestamp.clone();
            let trader_name_orig = elem.trader_name.clone();
            f(elem);
            if elem.order_id != order_id_orig {
                let idx = self
                    ._order_id_index
                    .remove(&order_id_orig)
                    .expect(
                        "Internal invariants broken, unable to find element in index 'field_name' despite being present in another",
                    );
                let orig_elem_idx = self
                    ._order_id_index
                    .insert(elem.order_id.clone(), idx);
                if orig_elem_idx.is_some() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "Unable to insert element, uniqueness constraint violated on field \'{0}\'",
                                "field_name",
                            ),
                        );
                    };
                }
            }
            if elem.timestamp != timestamp_orig {
                let idx = self
                    ._timestamp_index
                    .remove(&timestamp_orig)
                    .expect(
                        "Internal invariants broken, unable to find element in index 'field_name' despite being present in another",
                    );
                let orig_elem_idx = self
                    ._timestamp_index
                    .insert(elem.timestamp.clone(), idx);
                if orig_elem_idx.is_some() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "Unable to insert element, uniqueness constraint violated on field \'{0}\'",
                                "field_name",
                            ),
                        );
                    };
                }
            }
            if elem.trader_name != trader_name_orig {
                let idxs = self
                    ._trader_name_index
                    .get_mut(&trader_name_orig)
                    .expect(
                        "Internal invariants broken, unable to find element in index 'field_name' despite being present in another",
                    );
                if idxs.len() > 1 {
                    if !(idxs.remove(&idx)) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "Internal invariants broken, unable to find element in index \'field_name\' despite being present in another",
                                ),
                            );
                        };
                    }
                } else {
                    self._trader_name_index.remove(&trader_name_orig);
                }
                self._trader_name_index
                    .entry(elem.trader_name.clone())
                    .or_insert(::std::collections::BTreeSet::new())
                    .insert(idx);
            }
            Some(elem)
        }
        pub(crate) fn update_by_timestamp<__MultiIndexMapKeyType>(
            &mut self,
            key: &__MultiIndexMapKeyType,
            f: impl FnOnce(&mut String),
        ) -> Option<&Order>
        where
            u64: ::std::borrow::Borrow<__MultiIndexMapKeyType>,
            __MultiIndexMapKeyType: Ord + ?Sized,
        {
            let idx = *self._timestamp_index.get(key)?;
            let elem = &mut self._store[idx];
            f(&mut elem.note);
            Some(elem)
        }
        pub(crate) fn iter_by_timestamp<'__mim_iter_lifetime>(
            &'__mim_iter_lifetime self,
        ) -> MultiIndexOrderMapTimestampIter<'__mim_iter_lifetime> {
            MultiIndexOrderMapTimestampIter {
                _store_ref: &self._store,
                _iter: self._timestamp_index.iter(),
                _iter_rev: self._timestamp_index.iter().rev(),
                _inner_iter: None,
            }
        }
        pub(crate) fn get_by_trader_name<__MultiIndexMapKeyType>(
            &self,
            key: &__MultiIndexMapKeyType,
        ) -> Vec<&Order>
        where
            String: ::std::borrow::Borrow<__MultiIndexMapKeyType>,
            __MultiIndexMapKeyType: ::std::hash::Hash + Eq + ?Sized,
        {
            if let Some(idxs) = self._trader_name_index.get(key) {
                let mut elem_refs = Vec::with_capacity(idxs.len());
                for idx in idxs {
                    elem_refs.push(&self._store[*idx])
                }
                elem_refs
            } else {
                Vec::new()
            }
        }
        pub(crate) fn get_mut_by_trader_name(
            &mut self,
            key: &String,
        ) -> Vec<(&mut String,)> {
            if let Some(idxs) = self._trader_name_index.get(key) {
                let mut refs = Vec::with_capacity(idxs.len());
                let mut mut_iter = self._store.iter_mut();
                let mut last_idx: usize = 0;
                for idx in idxs.iter() {
                    match mut_iter.nth(*idx - last_idx) {
                        Some(val) => refs.push((&mut val.1.note,)),
                        _ => {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "Error getting mutable reference of non-unique field `{0}` in getter.",
                                        "trader_name",
                                    ),
                                );
                            };
                        }
                    }
                    last_idx = *idx + 1;
                }
                refs
            } else {
                Vec::new()
            }
        }
        pub(crate) fn remove_by_trader_name(&mut self, key: &String) -> Vec<Order> {
            if let Some(idxs) = self._trader_name_index.remove(key) {
                let mut elems = Vec::with_capacity(idxs.len());
                for idx in idxs {
                    let elem_orig = self._store.remove(idx);
                    let _removed_elem = self._order_id_index.remove(&elem_orig.order_id);
                    let _removed_elem = self
                        ._timestamp_index
                        .remove(&elem_orig.timestamp);
                    let key_to_remove = &elem_orig.trader_name;
                    if let Some(elems) = self._trader_name_index.get_mut(key_to_remove) {
                        if elems.len() > 1 {
                            if !elems.remove(&idx) {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Internal invariants broken, unable to find element in index \'field_name\' despite being present in another",
                                        ),
                                    );
                                };
                            }
                        } else {
                            self._trader_name_index.remove(key_to_remove);
                        }
                    }
                    elems.push(elem_orig)
                }
                elems
            } else {
                Vec::new()
            }
        }
        pub(crate) fn modify_by_trader_name(
            &mut self,
            key: &String,
            mut f: impl FnMut(&mut Order),
        ) -> Vec<&Order> {
            let idxs = match self._trader_name_index.get(key) {
                Some(container) => container.clone(),
                _ => ::std::collections::BTreeSet::<usize>::new(),
            };
            let mut refs = Vec::with_capacity(idxs.len());
            let mut mut_iter = self._store.iter_mut();
            let mut last_idx: usize = 0;
            for idx in idxs {
                match mut_iter.nth(idx - last_idx) {
                    Some(val) => {
                        let elem = val.1;
                        let order_id_orig = elem.order_id.clone();
                        let timestamp_orig = elem.timestamp.clone();
                        let trader_name_orig = elem.trader_name.clone();
                        f(elem);
                        if elem.order_id != order_id_orig {
                            let idx = self
                                ._order_id_index
                                .remove(&order_id_orig)
                                .expect(
                                    "Internal invariants broken, unable to find element in index 'field_name' despite being present in another",
                                );
                            let orig_elem_idx = self
                                ._order_id_index
                                .insert(elem.order_id.clone(), idx);
                            if orig_elem_idx.is_some() {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Unable to insert element, uniqueness constraint violated on field \'{0}\'",
                                            "field_name",
                                        ),
                                    );
                                };
                            }
                        }
                        if elem.timestamp != timestamp_orig {
                            let idx = self
                                ._timestamp_index
                                .remove(&timestamp_orig)
                                .expect(
                                    "Internal invariants broken, unable to find element in index 'field_name' despite being present in another",
                                );
                            let orig_elem_idx = self
                                ._timestamp_index
                                .insert(elem.timestamp.clone(), idx);
                            if orig_elem_idx.is_some() {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Unable to insert element, uniqueness constraint violated on field \'{0}\'",
                                            "field_name",
                                        ),
                                    );
                                };
                            }
                        }
                        if elem.trader_name != trader_name_orig {
                            let idxs = self
                                ._trader_name_index
                                .get_mut(&trader_name_orig)
                                .expect(
                                    "Internal invariants broken, unable to find element in index 'field_name' despite being present in another",
                                );
                            if idxs.len() > 1 {
                                if !(idxs.remove(&idx)) {
                                    {
                                        ::core::panicking::panic_fmt(
                                            format_args!(
                                                "Internal invariants broken, unable to find element in index \'field_name\' despite being present in another",
                                            ),
                                        );
                                    };
                                }
                            } else {
                                self._trader_name_index.remove(&trader_name_orig);
                            }
                            self._trader_name_index
                                .entry(elem.trader_name.clone())
                                .or_insert(::std::collections::BTreeSet::new())
                                .insert(idx);
                        }
                        refs.push(&*elem);
                    }
                    _ => {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "Error getting mutable reference of non-unique field `{0}` in modifier.",
                                    "trader_name",
                                ),
                            );
                        };
                    }
                }
                last_idx = idx + 1;
            }
            refs
        }
        pub(crate) fn update_by_trader_name<__MultiIndexMapKeyType>(
            &mut self,
            key: &__MultiIndexMapKeyType,
            mut f: impl FnMut(&mut String),
        ) -> Vec<&Order>
        where
            String: ::std::borrow::Borrow<__MultiIndexMapKeyType>,
            __MultiIndexMapKeyType: ::std::hash::Hash + Eq + ?Sized,
        {
            let empty = ::std::collections::BTreeSet::<usize>::new();
            let idxs = match self._trader_name_index.get(key) {
                Some(container) => container,
                _ => &empty,
            };
            let mut refs = Vec::with_capacity(idxs.len());
            let mut mut_iter = self._store.iter_mut();
            let mut last_idx: usize = 0;
            for idx in idxs {
                match mut_iter.nth(idx - last_idx) {
                    Some(val) => {
                        let elem = val.1;
                        f(&mut elem.note);
                        refs.push(&*elem);
                    }
                    _ => {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "Error getting mutable reference of non-unique field `{0}` in updater.",
                                    "trader_name",
                                ),
                            );
                        };
                    }
                }
                last_idx = idx + 1;
            }
            refs
        }
        pub(crate) fn iter_by_trader_name<'__mim_iter_lifetime>(
            &'__mim_iter_lifetime self,
        ) -> MultiIndexOrderMapTraderNameIter<'__mim_iter_lifetime> {
            MultiIndexOrderMapTraderNameIter {
                _store_ref: &self._store,
                _iter: self._trader_name_index.iter(),
                _inner_iter: None,
            }
        }
    }
    pub(crate) struct OrderIterMut<'__mim_iter_lifetime>(
        ::multi_index_map::slab::IterMut<'__mim_iter_lifetime, Order>,
    );
    impl<'__mim_iter_lifetime> Iterator for OrderIterMut<'__mim_iter_lifetime> {
        type Item = (&'__mim_iter_lifetime mut String,);
        fn next(&mut self) -> Option<Self::Item> {
            self.0.next().map(|(_, elem)| (&mut elem.note,))
        }
    }
    impl<'__mim_iter_lifetime> DoubleEndedIterator
    for OrderIterMut<'__mim_iter_lifetime> {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.0.next_back().map(|(_, elem)| (&mut elem.note,))
        }
    }
    impl<'__mim_iter_lifetime> ExactSizeIterator for OrderIterMut<'__mim_iter_lifetime> {
        fn len(&self) -> usize {
            self.0.len()
        }
    }
    impl<'__mim_iter_lifetime> std::iter::FusedIterator
    for OrderIterMut<'__mim_iter_lifetime> {}
    pub(crate) struct MultiIndexOrderMapOrderIdIter<'__mim_iter_lifetime> {
        _store_ref: &'__mim_iter_lifetime ::multi_index_map::slab::Slab<Order>,
        _iter: ::std::collections::hash_map::Iter<'__mim_iter_lifetime, u32, usize>,
        _inner_iter: Option<
            Box<
                dyn ::std::iter::Iterator<
                    Item = &'__mim_iter_lifetime usize,
                > + '__mim_iter_lifetime,
            >,
        >,
    }
    impl<'__mim_iter_lifetime> Iterator
    for MultiIndexOrderMapOrderIdIter<'__mim_iter_lifetime> {
        type Item = &'__mim_iter_lifetime Order;
        fn next(&mut self) -> Option<Self::Item> {
            Some(&self._store_ref[*self._iter.next()?.1])
        }
    }
    pub(crate) struct MultiIndexOrderMapTimestampIter<'__mim_iter_lifetime> {
        _store_ref: &'__mim_iter_lifetime ::multi_index_map::slab::Slab<Order>,
        _iter: ::std::collections::btree_map::Iter<'__mim_iter_lifetime, u64, usize>,
        _iter_rev: ::std::iter::Rev<
            ::std::collections::btree_map::Iter<'__mim_iter_lifetime, u64, usize>,
        >,
        _inner_iter: Option<
            Box<
                dyn ::std::iter::DoubleEndedIterator<
                    Item = &'__mim_iter_lifetime usize,
                > + '__mim_iter_lifetime,
            >,
        >,
    }
    impl<'__mim_iter_lifetime> Iterator
    for MultiIndexOrderMapTimestampIter<'__mim_iter_lifetime> {
        type Item = &'__mim_iter_lifetime Order;
        fn next(&mut self) -> Option<Self::Item> {
            Some(&self._store_ref[*self._iter.next()?.1])
        }
    }
    impl<'__mim_iter_lifetime> DoubleEndedIterator
    for MultiIndexOrderMapTimestampIter<'__mim_iter_lifetime> {
        fn next_back(&mut self) -> Option<Self::Item> {
            Some(&self._store_ref[*self._iter_rev.next()?.1])
        }
    }
    pub(crate) struct MultiIndexOrderMapTraderNameIter<'__mim_iter_lifetime> {
        _store_ref: &'__mim_iter_lifetime ::multi_index_map::slab::Slab<Order>,
        _iter: ::std::collections::hash_map::Iter<
            '__mim_iter_lifetime,
            String,
            ::std::collections::BTreeSet<usize>,
        >,
        _inner_iter: Option<
            Box<
                dyn ::std::iter::Iterator<
                    Item = &'__mim_iter_lifetime usize,
                > + '__mim_iter_lifetime,
            >,
        >,
    }
    impl<'__mim_iter_lifetime> Iterator
    for MultiIndexOrderMapTraderNameIter<'__mim_iter_lifetime> {
        type Item = &'__mim_iter_lifetime Order;
        fn next(&mut self) -> Option<Self::Item> {
            let inner_next = if let Some(inner_iter) = &mut self._inner_iter {
                inner_iter.next()
            } else {
                None
            };
            if let Some(next_index) = inner_next {
                Some(&self._store_ref[*next_index])
            } else {
                let hashmap_next = self._iter.next()?;
                self._inner_iter = Some(Box::new(hashmap_next.1.iter()));
                Some(
                    &self
                        ._store_ref[*self
                        ._inner_iter
                        .as_mut()
                        .unwrap()
                        .next()
                        .expect(
                            "Internal invariants broken, found empty slice in non_unique index 'trader_name'",
                        )],
                )
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Order {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Order",
                "order_id",
                &self.order_id,
                "timestamp",
                &self.timestamp,
                "trader_name",
                &self.trader_name,
                "note",
                &&self.note,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Order {
        #[inline]
        fn clone(&self) -> Order {
            Order {
                order_id: ::core::clone::Clone::clone(&self.order_id),
                timestamp: ::core::clone::Clone::clone(&self.timestamp),
                trader_name: ::core::clone::Clone::clone(&self.trader_name),
                note: ::core::clone::Clone::clone(&self.note),
            }
        }
    }
}
fn main() {
    let o1 = Order {
        order_id: 1,
        timestamp: 111,
        trader_name: "John".to_string(),
        note: "".to_string(),
    };
    let o2 = Order {
        order_id: 2,
        timestamp: 22,
        trader_name: "Tom".to_string(),
        note: "".to_string(),
    };
    let mut map = MultiIndexOrderMap::default();
    let _o1_ref: &Order = map.insert(o1);
    let _o2_ref: &Order = map.try_insert(o2).unwrap();
    let map = map;
    for o in map.iter_by_timestamp() {
        {
            ::std::io::_print(format_args!("iter_by_timestamp: {0:?}\n", o));
        }
    }
    for o in map.iter_by_order_id() {
        {
            ::std::io::_print(format_args!("iter_by_order_id: {0:?}\n", o));
        }
    }
    for (_, o) in map.iter() {
        {
            ::std::io::_print(format_args!("iter: {0:?}\n", o));
        }
    }
    let o1_ref = map.get_by_order_id(&1).unwrap();
    {
        ::std::io::_print(
            format_args!(
                "Got {0}\'s order by id {1}\n",
                o1_ref.trader_name,
                o1_ref.order_id,
            ),
        );
    };
    let mut map = map;
    for (o,) in map.iter_mut() {
        {
            ::std::io::_print(format_args!("iter_mut: {0:?}\n", o));
        }
    }
    let o1_ref = map
        .modify_by_order_id(
            &1,
            |o| {
                o.order_id = 7;
                o.timestamp = 77;
                o.trader_name = "Tom".to_string();
            },
        )
        .unwrap();
    {
        ::std::io::_print(
            format_args!(
                "Modified {0}\'s order by id, to {1:?}\n",
                o1_ref.trader_name,
                o1_ref,
            ),
        );
    };
    let o1_ref = map
        .modify_by_order_id(
            &112345,
            |o| {
                o.order_id = 7;
                o.timestamp = 77;
                o.trader_name = "Tom".to_string();
            },
        );
    {
        ::std::io::_print(
            format_args!(
                "******* Modified {0:?}\'s order by id, to {1:?}\n",
                o1_ref,
                o1_ref,
            ),
        );
    };
    let o1_ref = map
        .update_by_order_id(
            &7,
            |note| {
                *note = "TestNote".to_string();
            },
        )
        .unwrap();
    {
        ::std::io::_print(
            format_args!("Updated note of order {1:?}, to {0:?}\n", o1_ref.note, o1_ref),
        );
    };
    let (o1_note_ref,) = map.get_mut_by_order_id(&7).unwrap();
    *o1_note_ref = "TestNoteUpdated".to_string();
    {
        ::std::io::_print(
            format_args!(
                "Updated note of order with order_id {0:?}, to {1:?}\n",
                7,
                o1_note_ref,
            ),
        );
    };
    let toms_orders = map.remove_by_trader_name(&"Tom".to_string());
    match (&toms_orders.len(), &2) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    {
        ::std::io::_print(
            format_args!("Removed Tom\'s order by name: {0:?}\n", toms_orders),
        );
    };
    let o3 = Order {
        order_id: 3,
        timestamp: 33,
        trader_name: "Jimbo".to_string(),
        note: "".to_string(),
    };
    map.insert(o3);
    let o3 = map.remove_by_timestamp(&33).unwrap();
    {
        ::std::io::_print(
            format_args!(
                "Removed {0}\'s order by timestamp {1}\n",
                o3.trader_name,
                o3.timestamp,
            ),
        );
    };
    for (note,) in map.iter_mut() {
        note.push_str(" extra appended data");
    }
}

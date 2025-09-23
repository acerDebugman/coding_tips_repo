use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use std::sync::Arc;
use std::fmt::Debug;
use multi_index_map::MultiIndexMap;
use tokio::io::unix::AsyncFd;
use tokio::runtime::Handle;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct Activity(pub usize);
// #[automatically_derived]
// impl ::core::fmt::Debug for Activity {
//     #[inline]
//     fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
//         ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Activity", &&self.0)
//     }
// }
#[derive(Debug)]
pub struct AgentTask {
    pub agent_id: u64,
    pub task_id: u64,
    pub agent_state: Arc<RwLock<String>>,
    pub sender: tokio::sync::mpsc::Sender<Activity>,
    pub stop_sender: Arc<tokio::sync::oneshot::Sender<anyhow::Result<()>>>,
}
pub struct MultiIndexAgentTaskMap {
    _store: ::multi_index_map::slab::Slab<AgentTask>,
    _agent_id_index: ::std::collections::HashMap<
        u64,
        ::std::collections::BTreeSet<usize>,
        ::multi_index_map::rustc_hash::FxBuildHasher,
    >,
    _task_id_index: ::std::collections::BTreeMap<u64, usize>,
}
impl Default for MultiIndexAgentTaskMap {
    fn default() -> Self {
        Self {
            _store: ::multi_index_map::slab::Slab::default(),
            _agent_id_index: ::std::collections::HashMap::default(),
            _task_id_index: ::std::collections::BTreeMap::new(),
        }
    }
}
impl MultiIndexAgentTaskMap {
    pub fn with_capacity(n: usize) -> Self {
        Self {
            _store: ::multi_index_map::slab::Slab::with_capacity(n),
            _agent_id_index: ::std::collections::HashMap::default(),
            _task_id_index: ::std::collections::BTreeMap::new(),
        }
    }
    pub fn capacity(&self) -> usize {
        self._store.capacity()
    }
    pub fn len(&self) -> usize {
        self._store.len()
    }
    pub fn is_empty(&self) -> bool {
        self._store.is_empty()
    }
    pub fn reserve(&mut self, additional: usize) {
        self._store.reserve(additional);
        self._agent_id_index.reserve(additional);
    }
    pub fn shrink_to_fit(&mut self) {
        self._store.shrink_to_fit();
        self._agent_id_index.shrink_to_fit();
    }
    pub fn try_insert(
        &mut self,
        elem: AgentTask,
    ) -> Result<&AgentTask, ::multi_index_map::UniquenessError<AgentTask>> {
        let store_entry = self._store.vacant_entry();
        let idx = store_entry.key();
        let task_id_entry = match self._task_id_index.entry(elem.task_id.clone()) {
            ::std::collections::btree_map::Entry::Occupied(_) => {
                return Err(::multi_index_map::UniquenessError(elem));
            }
            ::std::collections::btree_map::Entry::Vacant(e) => e,
        };
        self._agent_id_index
            .entry(elem.agent_id.clone())
            .or_insert(::std::collections::BTreeSet::new())
            .insert(idx);
        task_id_entry.insert(idx);
        let elem = store_entry.insert(elem);
        Ok(elem)
    }
    pub fn insert(&mut self, elem: AgentTask) -> &AgentTask {
        self.try_insert(elem).expect("Unable to insert element")
    }
    pub fn clear(&mut self) {
        self._store.clear();
        self._agent_id_index.clear();
        self._task_id_index.clear();
    }
    pub fn iter(&self) -> ::multi_index_map::slab::Iter<AgentTask> {
        self._store.iter()
    }
    /// SAFETY:
    /// It is safe to mutate the non-indexed fields,
    /// however mutating any of the indexed fields will break the internal invariants.
    /// If the indexed fields need to be changed, the modify() method must be used.
    pub fn iter_mut<'__mim_iter_lifetime>(
        &'__mim_iter_lifetime mut self,
    ) -> AgentTaskIterMut<'__mim_iter_lifetime> {
        AgentTaskIterMut(self._store.iter_mut())
    }
    pub fn get_by_agent_id<__MultiIndexMapKeyType>(
        &self,
        key: &__MultiIndexMapKeyType,
    ) -> Vec<&AgentTask>
    where
        u64: ::std::borrow::Borrow<__MultiIndexMapKeyType>,
        __MultiIndexMapKeyType: ::std::hash::Hash + Eq + ?Sized,
    {
        if let Some(idxs) = self._agent_id_index.get(key) {
            let mut elem_refs = Vec::with_capacity(idxs.len());
            for idx in idxs {
                elem_refs.push(&self._store[*idx])
            }
            elem_refs
        } else {
            Vec::new()
        }
    }
    pub fn get_mut_by_agent_id(
        &mut self,
        key: &u64,
    ) -> Vec<
        (
            &mut Arc<RwLock<String>>,
            &mut tokio::sync::mpsc::Sender<Activity>,
            &mut Arc<tokio::sync::oneshot::Sender<anyhow::Result<()>>>,
        ),
    > {
        if let Some(idxs) = self._agent_id_index.get(key) {
            let mut refs = Vec::with_capacity(idxs.len());
            let mut mut_iter = self._store.iter_mut();
            let mut last_idx: usize = 0;
            for idx in idxs.iter() {
                match mut_iter.nth(*idx - last_idx) {
                    Some(val) => {
                        refs.push((
                            &mut val.1.agent_state,
                            &mut val.1.sender,
                            &mut val.1.stop_sender,
                        ))
                    }
                    _ => {
                        {
                            panic!("{}",
                                format_args!(
                                    "Error getting mutable reference of non-unique field `{0}` in getter.",
                                    "agent_id",
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
    pub fn remove_by_agent_id(&mut self, key: &u64) -> Vec<AgentTask> {
        if let Some(idxs) = self._agent_id_index.remove(key) {
            let mut elems = Vec::with_capacity(idxs.len());
            for idx in idxs {
                let elem_orig = self._store.remove(idx);
                let key_to_remove = &elem_orig.agent_id;
                if let Some(elems) = self._agent_id_index.get_mut(key_to_remove) {
                    if elems.len() > 1 {
                        if !elems.remove(&idx) {
                            {
                                panic!("{}",
                                    format_args!(
                                        "Internal invariants broken, unable to find element in index \'field_name\' despite being present in another",
                                    ),
                                );
                            };
                        }
                    } else {
                        self._agent_id_index.remove(key_to_remove);
                    }
                }
                let _removed_elem = self._task_id_index.remove(&elem_orig.task_id);
                elems.push(elem_orig)
            }
            elems
        } else {
            Vec::new()
        }
    }
    pub fn modify_by_agent_id2(
        &mut self,
        key: &u64,
        mut f: impl FnMut(&mut AgentTask),
    ) -> Vec<&AgentTask> {
        let idxs = match self._agent_id_index.get(key) {
            Some(container) => container.clone(),
            _ => ::std::collections::BTreeSet::<usize>::new(),
        };
        let mut refs = Vec::with_capacity(idxs.len());
        let mut mut_iter = self._store.iter_mut();
        let mut last_idx: usize = 0;
        println!("agent_id: {:?}, all idxs: {:?}, iter len: {}", key, idxs, mut_iter.len());
        for idx in idxs {
            println!("iter idx: {idx}, last_idx: {last_idx}, idx - last_idx: {}", idx - last_idx);
            match mut_iter.nth(idx - last_idx) {
                Some(val) => {
                    let elem = val.1;
                    let agent_id_orig = elem.agent_id.clone();
                    let task_id_orig = elem.task_id.clone();
                    f(elem);
                    if elem.agent_id != agent_id_orig {
                        let idxs = self
                            ._agent_id_index
                            .get_mut(&agent_id_orig)
                            .expect(
                                "Internal invariants broken, unable to find element in index 'field_name' despite being present in another",
                            );
                        if idxs.len() > 1 {
                            if !(idxs.remove(&idx)) {
                                {
                                    // ::core::panicking::panic_fmt(
                                    panic!("{}",
                                        format_args!(
                                            "Internal invariants broken, unable to find element in index \'field_name\' despite being present in another",
                                        ),
                                    );
                                };
                            }
                        } else {
                            self._agent_id_index.remove(&agent_id_orig);
                        }
                        self._agent_id_index
                            .entry(elem.agent_id.clone())
                            .or_insert(::std::collections::BTreeSet::new())
                            .insert(idx);
                    }
                    if elem.task_id != task_id_orig {
                        let idx = self
                            ._task_id_index
                            .remove(&task_id_orig)
                            .expect(
                                "Internal invariants broken, unable to find element in index 'field_name' despite being present in another",
                            );
                        let orig_elem_idx = self
                            ._task_id_index
                            .insert(elem.task_id.clone(), idx);
                        if orig_elem_idx.is_some() {
                            {
                                // ::core::panicking::panic_fmt(
                                panic!("{}",
                                    format_args!(
                                        "Unable to insert element, uniqueness constraint violated on field \'{0}\'",
                                        "field_name",
                                    ),
                                );
                            };
                        }
                    }
                    refs.push(&*elem);
                }
                _ => {
                    {
                        println!("panic key is: {:?}", key);
                        // ::core::panicking::panic_fmt(
                        panic!("{}",
                            format_args!(
                                "Error getting mutable reference of non-unique field `{0}` in modifier.",
                                "agent_id",
                            ),
                        );
                    };
                }
            }
            last_idx = idx + 1;
        }
        refs
    }
    pub fn modify_by_agent_id(
        &mut self,
        key: &u64,
        mut f: impl FnMut(&mut AgentTask),
    ) -> Vec<&AgentTask> {
        let idxs = match self._agent_id_index.get(key) {
            Some(container) => container.clone(),
            _ => ::std::collections::BTreeSet::<usize>::new(),
        };
        let mut modified_info = Vec::new();
        for &idx in &idxs {
            match self._store.get_mut(idx) {
                Some(elem) => {
                    let agent_id_orig = elem.agent_id.clone();
                    let task_id_orig = elem.task_id.clone();
                    
                    f(elem);
                    
                    // 记录需要更新索引的信息
                    if elem.agent_id != agent_id_orig || elem.task_id != task_id_orig {
                        modified_info.push((idx, agent_id_orig, task_id_orig, elem.agent_id.clone(), elem.task_id.clone()));
                    }
                }
                _ => {
                    println!("panic key is: {:?}", key);
                    panic!("Error getting mutable reference of non-unique field 'agent_id' in modifier.");
                }
            }
        } 

        // 第二阶段：更新索引
        for (idx, agent_id_orig, task_id_orig, new_agent_id, new_task_id) in modified_info {
            // 更新 agent_id 索引
            if new_agent_id != agent_id_orig {
                let idxs = self
                    ._agent_id_index
                    .get_mut(&agent_id_orig)
                    .expect("Internal invariants broken, unable to find element in index 'agent_id' despite being present in another");
                
                if idxs.len() > 1 {
                    if !idxs.remove(&idx) {
                        panic!("Internal invariants broken, unable to find element in index 'agent_id' despite being present in another");
                    }
                } else {
                    self._agent_id_index.remove(&agent_id_orig);
                }
                
                self._agent_id_index
                    .entry(new_agent_id)
                    .or_insert(::std::collections::BTreeSet::new())
                    .insert(idx);
            }
            
            // 更新 task_id 索引
            if new_task_id != task_id_orig {
                let removed_idx = self
                    ._task_id_index
                    .remove(&task_id_orig)
                    .expect("Internal invariants broken, unable to find element in index 'task_id' despite being present in another");
                
                let orig_elem_idx = self._task_id_index.insert(new_task_id, removed_idx);
                if orig_elem_idx.is_some() {
                    panic!("Unable to insert element, uniqueness constraint violated on field 'task_id'");
                }
            }
        }
        
        // 第三阶段：收集引用（需要重新借用）
        let mut refs = Vec::with_capacity(idxs.len());
        for &idx in &idxs {
            refs.push(&self._store[idx]);
        }

        refs
    }
    pub fn update_by_agent_id<__MultiIndexMapKeyType>(
        &mut self,
        key: &__MultiIndexMapKeyType,
        mut f: impl FnMut(
            &mut Arc<RwLock<String>>,
            &mut tokio::sync::mpsc::Sender<Activity>,
            &mut Arc<tokio::sync::oneshot::Sender<anyhow::Result<()>>>,
        ),
    ) -> Vec<&AgentTask>
    where
        u64: ::std::borrow::Borrow<__MultiIndexMapKeyType>,
        __MultiIndexMapKeyType: ::std::hash::Hash + Eq + ?Sized,
    {
        let empty = ::std::collections::BTreeSet::<usize>::new();
        let idxs = match self._agent_id_index.get(key) {
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
                    f(&mut elem.agent_state, &mut elem.sender, &mut elem.stop_sender);
                    refs.push(&*elem);
                }
                _ => {
                    {
                        // ::core::panicking::panic_fmt(
                        panic!("{}",
                            format_args!(
                                "Error getting mutable reference of non-unique field `{0}` in updater.",
                                "agent_id",
                            ),
                        );
                    };
                }
            }
            last_idx = idx + 1;
        }
        refs
    }
    pub fn iter_by_agent_id<'__mim_iter_lifetime>(
        &'__mim_iter_lifetime self,
    ) -> MultiIndexAgentTaskMapAgentIdIter<'__mim_iter_lifetime> {
        MultiIndexAgentTaskMapAgentIdIter {
            _store_ref: &self._store,
            _iter: self._agent_id_index.iter(),
            _inner_iter: None,
        }
    }
    pub fn get_by_task_id<__MultiIndexMapKeyType>(
        &self,
        key: &__MultiIndexMapKeyType,
    ) -> Option<&AgentTask>
    where
        u64: ::std::borrow::Borrow<__MultiIndexMapKeyType>,
        __MultiIndexMapKeyType: Ord + ?Sized,
    {
        Some(&self._store[*self._task_id_index.get(key)?])
    }
    pub fn get_mut_by_task_id(
        &mut self,
        key: &u64,
    ) -> Option<
        (
            &mut Arc<RwLock<String>>,
            &mut tokio::sync::mpsc::Sender<Activity>,
            &mut Arc<tokio::sync::oneshot::Sender<anyhow::Result<()>>>,
        ),
    > {
        let elem = &mut self._store[*self._task_id_index.get(key)?];
        Some((&mut elem.agent_state, &mut elem.sender, &mut elem.stop_sender))
    }
    pub fn remove_by_task_id(&mut self, key: &u64) -> Option<AgentTask> {
        let idx = self._task_id_index.remove(key)?;
        let elem_orig = self._store.remove(idx);
        let key_to_remove = &elem_orig.agent_id;
        if let Some(elems) = self._agent_id_index.get_mut(key_to_remove) {
            if elems.len() > 1 {
                if !elems.remove(&idx) {
                    {
                        // ::core::panicking::panic_fmt(
                        panic!("{}",
                            format_args!(
                                "Internal invariants broken, unable to find element in index \'field_name\' despite being present in another",
                            ),
                        );
                    };
                }
            } else {
                self._agent_id_index.remove(key_to_remove);
            }
        }
        let _removed_elem = self._task_id_index.remove(&elem_orig.task_id);
        Some(elem_orig)
    }
    pub fn modify_by_task_id(
        &mut self,
        key: &u64,
        f: impl FnOnce(&mut AgentTask),
    ) -> Option<&AgentTask> {
        let idx = *self._task_id_index.get(key)?;
        let elem = &mut self._store[idx];
        let agent_id_orig = elem.agent_id.clone();
        let task_id_orig = elem.task_id.clone();
        f(elem);
        if elem.agent_id != agent_id_orig {
            let idxs = self
                ._agent_id_index
                .get_mut(&agent_id_orig)
                .expect(
                    "Internal invariants broken, unable to find element in index 'field_name' despite being present in another",
                );
            if idxs.len() > 1 {
                if !(idxs.remove(&idx)) {
                    {
                        // ::core::panicking::panic_fmt(
                        panic!("{}",
                            format_args!(
                                "Internal invariants broken, unable to find element in index \'field_name\' despite being present in another",
                            ),
                        );
                    };
                }
            } else {
                self._agent_id_index.remove(&agent_id_orig);
            }
            self._agent_id_index
                .entry(elem.agent_id.clone())
                .or_insert(::std::collections::BTreeSet::new())
                .insert(idx);
        }
        if elem.task_id != task_id_orig {
            let idx = self
                ._task_id_index
                .remove(&task_id_orig)
                .expect(
                    "Internal invariants broken, unable to find element in index 'field_name' despite being present in another",
                );
            let orig_elem_idx = self._task_id_index.insert(elem.task_id.clone(), idx);
            if orig_elem_idx.is_some() {
                {
                    // ::core::panicking::panic_fmt(
                    panic!("{}",
                        format_args!(
                            "Unable to insert element, uniqueness constraint violated on field \'{0}\'",
                            "field_name",
                        ),
                    );
                };
            }
        }
        Some(elem)
    }
    pub fn update_by_task_id<__MultiIndexMapKeyType>(
        &mut self,
        key: &__MultiIndexMapKeyType,
        f: impl FnOnce(
            &mut Arc<RwLock<String>>,
            &mut tokio::sync::mpsc::Sender<Activity>,
            &mut Arc<tokio::sync::oneshot::Sender<anyhow::Result<()>>>,
        ),
    ) -> Option<&AgentTask>
    where
        u64: ::std::borrow::Borrow<__MultiIndexMapKeyType>,
        __MultiIndexMapKeyType: Ord + ?Sized,
    {
        let idx = *self._task_id_index.get(key)?;
        let elem = &mut self._store[idx];
        f(&mut elem.agent_state, &mut elem.sender, &mut elem.stop_sender);
        Some(elem)
    }
    pub fn iter_by_task_id<'__mim_iter_lifetime>(
        &'__mim_iter_lifetime self,
    ) -> MultiIndexAgentTaskMapTaskIdIter<'__mim_iter_lifetime> {
        MultiIndexAgentTaskMapTaskIdIter {
            _store_ref: &self._store,
            _iter: self._task_id_index.iter(),
            _iter_rev: self._task_id_index.iter().rev(),
            _inner_iter: None,
        }
    }
}
pub struct AgentTaskIterMut<'__mim_iter_lifetime>(
    ::multi_index_map::slab::IterMut<'__mim_iter_lifetime, AgentTask>,
);
impl<'__mim_iter_lifetime> Iterator for AgentTaskIterMut<'__mim_iter_lifetime> {
    type Item = (
        &'__mim_iter_lifetime mut Arc<RwLock<String>>,
        &'__mim_iter_lifetime mut tokio::sync::mpsc::Sender<Activity>,
        &'__mim_iter_lifetime mut Arc<tokio::sync::oneshot::Sender<anyhow::Result<()>>>,
    );
    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|(_, elem)| (
                &mut elem.agent_state,
                &mut elem.sender,
                &mut elem.stop_sender,
            ))
    }
}
impl<'__mim_iter_lifetime> DoubleEndedIterator
for AgentTaskIterMut<'__mim_iter_lifetime> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0
            .next_back()
            .map(|(_, elem)| (
                &mut elem.agent_state,
                &mut elem.sender,
                &mut elem.stop_sender,
            ))
    }
}
impl<'__mim_iter_lifetime> ExactSizeIterator for AgentTaskIterMut<'__mim_iter_lifetime> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
impl<'__mim_iter_lifetime> std::iter::FusedIterator
for AgentTaskIterMut<'__mim_iter_lifetime> {}
pub struct MultiIndexAgentTaskMapAgentIdIter<'__mim_iter_lifetime> {
    _store_ref: &'__mim_iter_lifetime ::multi_index_map::slab::Slab<AgentTask>,
    _iter: ::std::collections::hash_map::Iter<
        '__mim_iter_lifetime,
        u64,
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
for MultiIndexAgentTaskMapAgentIdIter<'__mim_iter_lifetime> {
    type Item = &'__mim_iter_lifetime AgentTask;
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
                        "Internal invariants broken, found empty slice in non_unique index 'agent_id'",
                    )],
            )
        }
    }
}
pub struct MultiIndexAgentTaskMapTaskIdIter<'__mim_iter_lifetime> {
    _store_ref: &'__mim_iter_lifetime ::multi_index_map::slab::Slab<AgentTask>,
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
for MultiIndexAgentTaskMapTaskIdIter<'__mim_iter_lifetime> {
    type Item = &'__mim_iter_lifetime AgentTask;
    fn next(&mut self) -> Option<Self::Item> {
        Some(&self._store_ref[*self._iter.next()?.1])
    }
}
impl<'__mim_iter_lifetime> DoubleEndedIterator
for MultiIndexAgentTaskMapTaskIdIter<'__mim_iter_lifetime> {
    fn next_back(&mut self) -> Option<Self::Item> {
        Some(&self._store_ref[*self._iter_rev.next()?.1])
    }
}
// #[automatically_derived]
// impl ::core::fmt::Debug for AgentTask {
//     #[inline]
//     fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
//         ::core::fmt::Formatter::debug_struct_field5_finish(
//             f,
//             "AgentTask",
//             "agent_id",
//             &self.agent_id,
//             "task_id",
//             &self.task_id,
//             "agent_state",
//             &self.agent_state,
//             "sender",
//             &self.sender,
//             "stop_sender",
//             &&self.stop_sender,
//         )
//     }
// }
impl Debug for MultiIndexAgentTaskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_list();
        for task in self.iter_by_task_id() {
            debug.entry(task);
        }
        debug.finish()
    }
}
fn main() -> anyhow::Result<()> {
    let body = async {
        let (mut tx, mut rx) = tokio::sync::mpsc::channel::<String>(1000000);
        std::thread::spawn(move || {
            while let Some(msg) = rx.blocking_recv() {
                println!("{msg} ********");
            }
        });

        let total = 1000;
        let agent_tasks = Arc::new(RwLock::new(MultiIndexAgentTaskMap::default()));
        let (ctx, _rx) = tokio::sync::mpsc::channel(1000);
        let mut tasks = Vec::new();
        let jd = tokio::spawn({
            let agent_tasks = agent_tasks.clone();
            let logtx = tx.clone();
            let ctx = ctx.clone();
            async move {
                for idx in 0..total {
                    let task = AgentTask {
                        agent_id: idx % total,
                        task_id: idx,
                        agent_state: Arc::new(RwLock::new("test".to_string())),
                        sender: ctx.clone(),
                        stop_sender: Arc::new(tokio::sync::oneshot::channel().0),
                    };
                    // println!("insert task agent id: {:?}", task.agent_id);
                    let mut writer_guard = agent_tasks.write().await;
                    let _ = logtx.send(format!("insert task agent id: {:?}", task.agent_id)).await;
                    writer_guard.insert(task);
                }
                {
                    println!("{}", format_args!("insert agent_tasks exit\n"));
                };
            }
        });
        tasks.push(jd);

        

        tasks
            .push(
                tokio::spawn({
                    let agent_tasks = agent_tasks.clone();
                    let tx = tx.clone();
                    async move {
                        let mut idx = 0;
                        loop {
                            idx += 1;
                            for _ in 0..total {
                                let n = rand::random::<u64>() % total;
                                // let _ = tx.blocking_send(format!("will remove task_id {n}"));
                                // println!("will remove task_id {n}");
                                let mut writer_guard = agent_tasks.write().await;
                                let _ = tx.send(format!("will remove task_id {n}")).await;
                                writer_guard.remove_by_task_id(&n);
                            }
                            tokio::time::sleep(tokio::time::Duration::from_micros(1))
                                .await;
                            if idx % 1000 == 0 {
                                {
                                    println!("{}",
                                        format_args!("clear agent_tasks {0} times\n", idx),
                                    );
                                };
                            }
                        }
                        {
                            println!("{}",
                                format_args!("clear dify agent_tasks exit\n"),
                            );
                        };
                    }
                }),
            );
        tasks
            .push(
                tokio::spawn({
                    let agent_tasks = agent_tasks.clone();
                    async move {
                        loop {
                            let mut writer_guard = agent_tasks.write().await;
                            for idx in 0..total {
                                // {
                                //     println!("{}",
                                //         format_args!("modify agent_id {0}\n", idx),
                                //     );
                                // };
                                let _ = tx.send(format!("modify agent_id {idx}")).await;
                                // let _ = tx.blocking_send(format!("modify agent_id {idx}"));
                                writer_guard
                                    .modify_by_agent_id(
                                        &idx,
                                        |task| {
                                            tokio::task::block_in_place(|| {
                                                Handle::current()
                                                    .block_on(async {
                                                        *task.agent_state.write().await = "test_modified"
                                                            .to_string();
                                                    })
                                            })
                                        },
                                    );
                            }
                        }
                        {
                            println!("{}", format_args!("modify agent_tasks exit\n"));
                        };
                    }
                }),
            );
        for task in tasks {
            task.await?;
        }
        Ok(())
    };
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return
    )]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

use crate::component::{Component, Library, NonNullableMode, SourceInfo};
use crate::expr::Constant;
use crate::prim::*;

#[derive(Debug)]
pub struct ComponentBuilder {
    libraries: Vec<Library>,
    sources: Vec<SourceInfo>,
    constants: Vec<Constant>,
    canonical_names: IndexMap<CanonicalName>,
    strings: IndexMap<StrRef>,
}

impl ComponentBuilder {
    pub fn add_string(&mut self, name: StrRef) -> StringRef {
        let idx = self.strings.add(name) as u32;
        ComponentRef::new(idx.into())
    }

    pub fn add_name(&mut self, name: StringRef, parent: CanonicalNameRef) -> CanonicalNameRef {
        let canonical_name = CanonicalName { name, parent };
        let idx = self.canonical_names.add(canonical_name) as u32;
        ComponentRef::new((idx + 1).into())
    }

    pub fn add_source(&mut self, source: SourceInfo) -> UriRef {
        let idx = self.sources.len() as u32;
        self.sources.push(source);
        ComponentRef::new(idx.into())
    }

    pub fn add_constant(&mut self, constant: Constant) -> ConstantRef {
        let idx = self.constants.len() as u32;
        self.constants.push(constant);
        ComponentRef::new(idx.into())
    }

    #[inline]
    pub fn add_name_from<I: IntoIterator<Item = StrRef>>(&mut self, parts: I) -> CanonicalNameRef {
        self.add_parts(parts.into_iter(), ComponentRef::UNDEFINED)
    }

    fn add_parts<I>(&mut self, mut parts: I, parent: CanonicalNameRef) -> CanonicalNameRef
    where
        I: Iterator<Item = StrRef>,
    {
        match parts.next() {
            Some(str) => {
                let name = self.add_string(str);
                let name = self.add_name(name, parent);
                self.add_parts(parts, name)
            }
            None => parent,
        }
    }

    pub fn add_library(&mut self, library: Library) {
        self.libraries.push(library);
    }

    pub fn get_canonical_name(&self, name: CanonicalNameRef) -> CanonicalName {
        let u: u32 = name.into();
        self.canonical_names.values[u as usize - 1].clone()
    }

    pub fn get_str(&self, name: StringRef) -> StrRef {
        let u: u32 = name.into();
        self.strings.values[u as usize].clone()
    }

    pub fn into_component(
        self,
        main_method: CanonicalNameRef,
        non_nullable_mode: NonNullableMode,
    ) -> Component {
        Component {
            problems: vec![],
            libraries: self.libraries,
            source_map: self.sources,
            constants: self.constants,
            canonical_names: self.canonical_names.into_vec(),
            payloads: vec![],
            strings: self.strings.into_vec(),
            main_method,
            non_nullable_mode,
        }
    }
}

impl Default for ComponentBuilder {
    fn default() -> Self {
        let mut strings = IndexMap::default();
        strings.add(StrRef::Borrowed(""));
        Self {
            libraries: vec![],
            sources: vec![],
            constants: vec![],
            canonical_names: IndexMap::default(),
            strings,
        }
    }
}

#[derive(Debug)]
struct IndexMap<A> {
    values: Vec<A>,
    mappings: HashMap<A, usize>,
}

impl<A: Eq + Hash + Clone> IndexMap<A> {
    pub fn add(&mut self, item: A) -> usize {
        match self.mappings.entry(item) {
            Entry::Occupied(enty) => *enty.get(),
            Entry::Vacant(entry) => {
                let next = self.values.len();
                self.values.push(entry.key().clone());
                *entry.insert(next)
            }
        }
    }

    pub fn into_vec(self) -> Vec<A> {
        self.values
    }
}

impl<A> Default for IndexMap<A> {
    fn default() -> Self {
        Self {
            values: vec![],
            mappings: HashMap::new(),
        }
    }
}

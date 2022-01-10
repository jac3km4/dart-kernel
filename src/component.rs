use core::fmt;
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Seek, SeekFrom, Write};
use std::path::Path;

use declio::ctx::Len;
use declio::{Decode, Encode};
use hex_magic::hex;
use wtf8::Wtf8;

use crate::expr::{Constant, Expr};
use crate::flags::{DependencyFlags, LibraryFlags};
use crate::node::{Class, Combinator, Extension, Field, Procedure, Typedef};
use crate::prim::*;
use crate::{codecs, writer};

pub struct ComponentFile<R> {
    source: R,
    index: ComponentIndex,
}

impl ComponentFile<BufReader<File>> {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, declio::Error> {
        let source = BufReader::new(File::open(path)?);
        Self::load(source)
    }
}

impl<R: Seek + Read> ComponentFile<R> {
    pub fn load(mut source: R) -> Result<Self, declio::Error> {
        source.seek(SeekFrom::End(-(ComponentMetadata::SIZE as i64)))?;
        let meta = ComponentMetadata::decode((), ENDIANESS, &mut source)?;

        let index_offset = ComponentIndex::MINIMUM_SIZE
            + meta.library_count as usize * 4
            + ComponentMetadata::SIZE;
        source.seek(SeekFrom::End(-(index_offset as i64)))?;
        let index = ComponentIndex::decode(meta.library_count, ENDIANESS, &mut source)?;

        Ok(Self { source, index })
    }

    pub fn libraries(&mut self) -> Result<Vec<Library>, declio::Error> {
        let mut libs = Vec::with_capacity(self.index.library_offsets.len());

        for offset in &self.index.library_offsets[0..self.index.library_offsets.len() - 1] {
            self.source.seek(SeekFrom::Start((*offset).into()))?;
            libs.push(Library::decode((), ENDIANESS, &mut self.source)?);
        }
        Ok(libs)
    }

    pub fn string_table(&mut self) -> Result<StringTable, declio::Error> {
        self.source
            .seek(SeekFrom::Start(self.index.string_table_offset.into()))?;
        StringTable::decode((), ENDIANESS, &mut self.source)
    }

    pub fn constants(&mut self) -> Result<Vec<Constant>, declio::Error> {
        self.source
            .seek(SeekFrom::Start(self.index.constant_table_offset.into()))?;
        codecs::list::decode((), ENDIANESS, &mut self.source)
    }

    pub fn canonical_names(&mut self) -> Result<Vec<CanonicalName>, declio::Error> {
        self.source
            .seek(SeekFrom::Start(self.index.canonical_names_offset.into()))?;
        codecs::list::decode((), ENDIANESS, &mut self.source)
    }

    pub fn source_map(&mut self) -> Result<Vec<SourceInfo>, declio::Error> {
        self.source
            .seek(SeekFrom::Start(self.index.source_table_offset.into()))?;
        let length = u32::decode((), ENDIANESS, &mut self.source)?;
        <Vec<SourceInfo>>::decode(Len(length as usize), ENDIANESS, &mut self.source)
    }
}

#[derive(Debug, Default)]
pub struct Component {
    pub problems: Vec<Utf8>,
    pub libraries: Vec<Library>,
    pub source_map: Vec<SourceInfo>,
    pub constants: Vec<Constant>,
    pub canonical_names: Vec<CanonicalName>,
    pub payloads: Vec<Vec<u8>>,
    pub strings: Vec<StrRef>,
    pub main_method: CanonicalNameRef,
    pub non_nullable_mode: NonNullableMode,
}

impl Component {
    pub fn encode<W: Write + Seek>(
        &self,
        header: &ComponentHeader,
        out: &mut W,
    ) -> Result<(), declio::Error> {
        writer::write_component(header, self, out)
    }
}

#[derive(Debug, Encode, Decode)]
pub struct ComponentHeader {
    pub magic: HeaderMagic,
    pub version: u32,
    pub sdk_hash: [u8; 10],
}

impl ComponentHeader {
    pub const DART_2_15_1: ComponentHeader = ComponentHeader {
        magic: HeaderMagic,
        version: 74,
        sdk_hash: hex!("31 32 37 38 62 64 35 61 64 62"),
    };

    pub const DART_2_16_0_134: ComponentHeader = ComponentHeader {
        magic: HeaderMagic,
        version: 75,
        sdk_hash: hex!("34 35 35 66 65 39 64 31 38 30"),
    };

    pub const DART_2_17_0_1: ComponentHeader = ComponentHeader {
        magic: HeaderMagic,
        version: 75,
        sdk_hash: hex!("33 35 64 36 36 38 30 30 34 37"),
    };
}

#[derive(Debug, Encode, Decode)]
#[declio(ctx = "library_count: u32")]
pub(crate) struct ComponentIndex {
    pub source_table_offset: u32,
    pub constant_table_offset: u32,
    pub constant_table_index_offset: u32,
    pub canonical_names_offset: u32,
    pub metadata_payloads_offset: u32,
    pub metadata_mappings_offset: u32,
    pub string_table_offset: u32,
    pub component_index_offset: u32,
    pub main_method_reference: u32,
    pub compilation_mode: NonNullableMode,
    #[declio(ctx = "Len((library_count + 1).try_into()?)")]
    pub library_offsets: Vec<u32>,
}

impl ComponentIndex {
    pub const MINIMUM_SIZE: usize = 44;
}

#[derive(Debug, Encode, Decode)]
pub(crate) struct ComponentMetadata {
    pub library_count: u32,
    pub file_size: u32,
}

impl ComponentMetadata {
    pub const SIZE: usize = 8;
}

#[derive(Debug, Default, Decode)]
pub struct Library {
    pub flags: LibraryFlags,
    pub version_major: Uint,
    pub version_minor: Uint,
    pub canonical_name: CanonicalNameRef,
    pub name: StringRef,
    pub file_uri: UriRef,
    #[declio(with = "codecs::list")]
    pub problems: Vec<Utf8>,
    #[declio(with = "codecs::list")]
    pub annotations: Vec<Expr>,
    #[declio(with = "codecs::list")]
    pub dependencies: Vec<LibraryDependency>,
    #[declio(with = "codecs::list")]
    pub additional_exports: Vec<CanonicalNameRef>,
    #[declio(with = "codecs::list")]
    pub library_parts: Vec<LibraryPart>,
    #[declio(with = "codecs::list")]
    pub typedefs: Vec<Typedef>,
    #[declio(with = "codecs::list")]
    pub classes: Vec<Class>,
    #[declio(with = "codecs::list")]
    pub extensions: Vec<Extension>,
    #[declio(with = "codecs::list")]
    pub fields: Vec<Field>,
    #[declio(with = "codecs::list")]
    pub procedures: Vec<Procedure>,
    #[declio(with = "codecs::list")]
    pub source_refs: Vec<Uint>,
}

#[derive(Debug, Default, Encode, Decode)]
pub struct LibraryDependency {
    pub offset: FileOffset,
    pub flags: DependencyFlags,
    #[declio(with = "codecs::list")]
    pub annotations: Vec<Expr>,
    pub target_library: CanonicalNameRef,
    pub name: StringRef,
    #[declio(with = "codecs::list")]
    pub combinators: Vec<Combinator>,
}

#[derive(Debug, Encode, Decode)]
pub struct LibraryPart {
    #[declio(with = "codecs::list")]
    pub annotations: Vec<Expr>,
    pub part_uri: StringRef,
}

#[derive(Encode, Decode)]
pub struct StringTable {
    #[declio(with = "codecs::list")]
    end_offsets: Vec<Uint>,
    #[declio(
        ctx = r#"Len(<Vec<Uint>>::as_slice(end_offsets).last().map(|i| i.0 as usize).unwrap_or(0))"#,
        with = "codecs::bytes"
    )]
    bytes: Vec<u8>,
}

impl StringTable {
    pub fn new<'a, T: IntoIterator<Item = &'a str>>(strs: T) -> Result<Self, declio::Error> {
        let mut end_offsets = vec![];
        let mut bytes = Cursor::new(vec![]);

        for str in strs {
            bytes.write_all(str.as_bytes())?;
            end_offsets.push((bytes.position() as u32).into());
        }
        let res = StringTable {
            end_offsets,
            bytes: bytes.into_inner(),
        };
        Ok(res)
    }

    pub fn get(&self, index: usize) -> Option<&Wtf8> {
        let start = if index == 0 {
            0usize
        } else {
            self.end_offsets.get(index - 1)?.0 as usize
        };
        let end = self.end_offsets.get(index)?.0 as usize;
        let bytes = &self.bytes[start..end];
        Some(Wtf8::from_bytes(bytes))
    }
}

impl fmt::Debug for StringTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        for i in 0..self.end_offsets.len() {
            let elem = self.get(i).unwrap();
            list.entry(&elem);
        }
        list.finish()
    }
}

#[derive(Debug, Default, Encode, Decode)]
pub struct SourceInfo {
    pub uri: Utf8,
    pub source: Utf8,
    #[declio(with = "codecs::list")]
    pub line_starts: Vec<Uint>,
    pub import_uri: Utf8,
    #[declio(with = "codecs::list")]
    pub constructor_coverage: Vec<CanonicalNameRef>,
}

#[derive(Debug, Clone, Copy, Encode, Decode)]
#[declio(id_type = "u32")]
pub enum NonNullableMode {
    #[declio(id = "0")]
    Disabled,
    #[declio(id = "1")]
    Weak,
    #[declio(id = "2")]
    Strong,
    #[declio(id = "3")]
    Agnostic,
}

impl Default for NonNullableMode {
    fn default() -> Self {
        Self::Weak
    }
}

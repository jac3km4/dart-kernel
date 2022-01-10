use std::io;

use declio::ctx::{Endian, Len};
use declio::Encode;

use crate::codecs;
use crate::component::*;
use crate::node::Class;
use crate::prim::{CowRcStr, Uint, ENDIANESS};

pub fn write_component<W: io::Write + io::Seek>(
    header: &ComponentHeader,
    comp: &Component,
    out: &mut W,
) -> Result<(), declio::Error> {
    header.encode((), ENDIANESS, out)?;

    codecs::list::encode(&comp.problems, (), ENDIANESS, out)?;

    let mut library_offsets = write_keeping_offsets(
        &comp.libraries,
        |lib, _, _, out| write_library(lib, out),
        out,
    )?;
    library_offsets.push(out.stream_position()? as u32);

    let source_table_offset = out.stream_position()? as u32;
    (comp.source_map.len() as u32).encode((), ENDIANESS, out)?;
    let source_offsets = write_keeping_offsets(&comp.source_map, Encode::encode, out)?;
    source_offsets.encode(Len(comp.source_map.len()), ENDIANESS, out)?;

    let constant_table_offset = out.stream_position()? as u32;
    Uint(comp.constants.len() as u32).encode((), ENDIANESS, out)?;
    let constant_table_index_offset = out.stream_position()? as u32;
    let constant_offsets = write_keeping_offsets(&comp.constants, Encode::encode, out)?;
    (constant_offsets.len() as u32).encode((), ENDIANESS, out)?;

    let canonical_names_offset = out.stream_position()? as u32;
    codecs::list::encode(&comp.canonical_names, (), ENDIANESS, out)?;

    let metadata_payloads_offset = out.stream_position()? as u32;
    let metadata_offsets = write_keeping_offsets(
        &comp.payloads,
        |payload, _, _, out| out.write_all(payload).map_err(Into::into),
        out,
    )?;
    let metadata_mappings_offset = out.stream_position()? as u32;
    metadata_offsets.encode(Len(metadata_offsets.len()), ENDIANESS, out)?;
    (metadata_offsets.len() as u32).encode((), ENDIANESS, out)?;

    let string_table_offset = out.stream_position()? as u32;
    let string_table = StringTable::new(comp.strings.iter().map(CowRcStr::as_str))?;
    string_table.encode((), ENDIANESS, out)?;

    let component_index_offset = out.stream_position()? as u32;
    let index = ComponentIndex {
        source_table_offset,
        constant_table_offset,
        constant_table_index_offset,
        canonical_names_offset,
        metadata_payloads_offset,
        metadata_mappings_offset,
        string_table_offset,
        component_index_offset,
        main_method_reference: comp.main_method.into(),
        compilation_mode: comp.non_nullable_mode,
        library_offsets,
    };
    let library_count = comp.libraries.len() as u32;
    index.encode(library_count, ENDIANESS, out)?;

    let file_size = out.stream_position()? as u32 + ComponentMetadata::SIZE as u32;
    let metadata = ComponentMetadata {
        library_count,
        file_size,
    };
    metadata.encode((), ENDIANESS, out)
}

fn write_library<W: io::Write + io::Seek>(lib: &Library, out: &mut W) -> Result<(), declio::Error> {
    lib.flags.encode((), ENDIANESS, out)?;
    lib.version_major.encode((), ENDIANESS, out)?;
    lib.version_minor.encode((), ENDIANESS, out)?;
    lib.canonical_name.encode((), ENDIANESS, out)?;
    lib.name.encode((), ENDIANESS, out)?;
    lib.file_uri.encode((), ENDIANESS, out)?;
    codecs::list::encode(&lib.problems, (), ENDIANESS, out)?;
    codecs::list::encode(&lib.annotations, (), ENDIANESS, out)?;
    codecs::list::encode(&lib.dependencies, (), ENDIANESS, out)?;
    codecs::list::encode(&lib.additional_exports, (), ENDIANESS, out)?;
    codecs::list::encode(&lib.library_parts, (), ENDIANESS, out)?;
    codecs::list::encode(&lib.typedefs, (), ENDIANESS, out)?;

    Uint(lib.classes.len() as u32).encode((), ENDIANESS, out)?;
    let mut class_offsets =
        write_keeping_offsets(&lib.classes, |item, _, _, out| write_class(item, out), out)?;
    class_offsets.push(out.stream_position()? as u32);

    codecs::list::encode(&lib.extensions, (), ENDIANESS, out)?;
    codecs::list::encode(&lib.fields, (), ENDIANESS, out)?;

    Uint(lib.procedures.len() as u32).encode((), ENDIANESS, out)?;
    let mut procedure_offsets = write_keeping_offsets(&lib.procedures, Encode::encode, out)?;
    procedure_offsets.push(out.stream_position()? as u32);

    let source_refs_offset = out.stream_position()? as u32;
    codecs::list::encode(&lib.source_refs, (), ENDIANESS, out)?;

    source_refs_offset.encode((), ENDIANESS, out)?;
    write_offsets(&class_offsets, out)?;
    write_offsets(&procedure_offsets, out)
}

fn write_class<W: io::Write + io::Seek>(class: &Class, out: &mut W) -> Result<(), declio::Error> {
    class.tag.encode((), ENDIANESS, out)?;
    class.canonical_name.encode((), ENDIANESS, out)?;
    class.file_uri.encode((), ENDIANESS, out)?;
    class.start_offset.encode((), ENDIANESS, out)?;
    class.definition_range.encode((), ENDIANESS, out)?;
    class.flags.encode((), ENDIANESS, out)?;
    class.name.encode((), ENDIANESS, out)?;
    codecs::list::encode(&class.annotations, (), ENDIANESS, out)?;
    codecs::list::encode(&class.type_params, (), ENDIANESS, out)?;
    codecs::option::encode(&class.super_class, (), ENDIANESS, out)?;
    codecs::option::encode(&class.mixed_in_type, (), ENDIANESS, out)?;
    codecs::list::encode(&class.implemented_classes, (), ENDIANESS, out)?;
    codecs::list::encode(&class.fields, (), ENDIANESS, out)?;
    codecs::list::encode(&class.constructors, (), ENDIANESS, out)?;

    Uint(class.procedures.len() as u32).encode((), ENDIANESS, out)?;
    let mut proc_offsets = write_keeping_offsets(&class.procedures, Encode::encode, out)?;
    proc_offsets.push(out.stream_position()? as u32);

    codecs::list::encode(&class.redirecting_factories, (), ENDIANESS, out)?;

    write_offsets(&proc_offsets, out)
}

fn write_keeping_offsets<A, W, F>(
    items: &[A],
    encode: F,
    out: &mut W,
) -> Result<Vec<u32>, declio::Error>
where
    W: io::Write + io::Seek,
    F: Fn(&A, (), Endian, &mut W) -> Result<(), declio::Error>,
{
    let mut offsets = Vec::with_capacity(items.len());

    for item in items {
        offsets.push(out.stream_position()? as u32);
        encode(item, (), ENDIANESS, out)?;
    }
    Ok(offsets)
}

fn write_offsets<W: io::Write>(offsets: &[u32], out: &mut W) -> Result<(), declio::Error> {
    offsets.encode(Len(offsets.len()), ENDIANESS, out)?;
    (offsets.len() as u32 - 1).encode((), ENDIANESS, out)
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::io::Cursor;

    use super::*;
    use crate::prim::StrRef;

    #[test]
    fn test_roundtrip() -> Result<(), Box<dyn Error>> {
        let mut comp = Component::default();
        comp.strings.push(StrRef::Borrowed("test😵"));

        let mut buf: Cursor<Vec<u8>> = Cursor::new(vec![]);
        comp.encode(&ComponentHeader::DART_2_16_0_134, &mut buf)?;

        buf.set_position(0);
        let mut loaded = ComponentFile::load(buf)?;

        assert_eq!(loaded.libraries()?.len(), 0);
        assert_eq!(loaded.constants()?.len(), 0);

        let str_table = loaded.string_table()?;
        let wtf_str = str_table.get(0).and_then(|wtf| wtf.as_str());
        assert_eq!(wtf_str, Some("test😵"));

        Ok(())
    }
}

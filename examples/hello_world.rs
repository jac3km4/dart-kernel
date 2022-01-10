use std::error::Error;
use std::fs::File;

use dart_kernel::builder::ComponentBuilder;
use dart_kernel::component::{ComponentHeader, Library, NonNullableMode, SourceInfo};
use dart_kernel::expr::{Arguments, Expr, Stmt};
use dart_kernel::flags::ProcedureFlags;
use dart_kernel::names;
use dart_kernel::node::{Function, Procedure, Type};
use dart_kernel::prim::{FileOffset, FileRange, StrRef};

fn main() -> Result<(), Box<dyn Error>> {
    let mut builder = ComponentBuilder::default();
    builder.add_source(SourceInfo::default());

    let lib_name_str = StrRef::Borrowed("hello_world_module");
    let lib_name = builder.add_name_from([lib_name_str.clone()]);
    let main_name = builder.add_name_from([lib_name_str.clone(), names::METHODS, names::MAIN]);
    let print_name =
        builder.add_name_from([names::DART_CORE, names::METHODS, StrRef::Borrowed("print")]);
    let hello_world = builder.add_string(StrRef::Borrowed("Hello world!"));

    let args = Arguments::positional(vec![], vec![Expr::StringLit(hello_world)]);
    let body = Expr::StaticInvoke {
        offset: FileOffset::default(),
        target: print_name,
        arguments: Box::new(args),
    };
    let func = Function {
        body: Some(Box::new(Stmt::Block {
            range: FileRange::default(),
            statements: vec![Stmt::Expr(Box::new(body))],
        })),
        return_type: Type::Void,
        ..Function::default()
    };
    let proc = Procedure {
        canonical_name: main_name,
        function: Box::new(func),
        flags: ProcedureFlags::new().with_is_static(true),
        ..Procedure::default()
    };

    let lib = Library {
        version_major: 2.into(),
        version_minor: 15.into(),
        canonical_name: lib_name,
        procedures: vec![proc],
        ..Library::default()
    };

    builder.add_library(lib);
    builder
        .into_component(main_name, NonNullableMode::Strong)
        .encode(
            &ComponentHeader::DART_2_15_1,
            &mut File::create("hello_world.dill")?,
        )?;

    Ok(())
}

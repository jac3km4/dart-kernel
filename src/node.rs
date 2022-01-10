use declio::ctx::Endian;
use declio::{Decode, Encode};

use crate::codecs;
use crate::expr::{Arguments, Assert, Expr, Stmt, VarDecl};
use crate::flags::{ClassFlags, CombinatorFlags, ConstructorFlags, FieldFlags, ProcedureFlags};
use crate::prim::*;

#[derive(Debug, Default, Encode, Decode)]
pub struct Procedure {
    pub tag: ProcedureTag,
    pub canonical_name: CanonicalNameRef,
    pub file_uri: UriRef,
    pub offset: FileOffset,
    pub definition_range: FileRange,
    pub kind: ProcedureKind,
    pub stub_kind: ProcedureStubKind,
    pub flags: ProcedureFlags,
    pub name: StringRef,
    #[declio(with = "codecs::list")]
    pub annotations: Vec<Expr>,
    pub stub_target: CanonicalNameRef,
    #[declio(with = "codecs::option")]
    pub signature_type: Option<Box<FunctionType>>,
    pub function: Box<Function>,
}

#[derive(Debug, Default, Decode)]
pub struct Class {
    pub tag: ClassTag,
    pub canonical_name: CanonicalNameRef,
    pub file_uri: UriRef,
    pub start_offset: FileOffset,
    pub definition_range: FileRange,
    pub flags: ClassFlags,
    pub name: StringRef,
    #[declio(with = "codecs::list")]
    pub annotations: Vec<Expr>,
    #[declio(with = "codecs::list")]
    pub type_params: Vec<TypeParameter>,
    #[declio(with = "codecs::option")]
    pub super_class: Option<Type>,
    #[declio(with = "codecs::option")]
    pub mixed_in_type: Option<Type>,
    #[declio(with = "codecs::list")]
    pub implemented_classes: Vec<Type>,
    #[declio(with = "codecs::list")]
    pub fields: Vec<Field>,
    #[declio(with = "codecs::list")]
    pub constructors: Vec<Constructor>,
    #[declio(with = "codecs::list")]
    pub procedures: Vec<Procedure>,
    #[declio(with = "codecs::list")]
    pub redirecting_factories: Vec<RedirectingFactory>,
    #[declio(ctx = "procedures.len()")]
    pub index: ClassIndex,
}

#[derive(Debug, Default, Encode, Decode)]
pub struct Extension {
    pub tag: ExtensionTag,
    pub canonical_name: CanonicalNameRef,
    pub name: StringRef,
    #[declio(with = "codecs::list")]
    pub annotations: Vec<Expr>,
    pub file_uri: UriRef,
    pub offset: FileOffset,
    pub flags: u8,
    #[declio(with = "codecs::list")]
    pub type_params: Vec<TypeParameter>,
    pub on_type: Type,
    #[declio(with = "codecs::option")]
    pub show_hide_clause: Option<ExtensionShowClause>,
}

#[derive(Debug, Default, Encode, Decode)]
pub struct Field {
    pub tag: FieldTag,
    pub canonical_name: CanonicalNameRef,
    pub canonical_name_getter: CanonicalNameRef,
    pub canonical_name_setter: CanonicalNameRef,
    pub file_uri: UriRef,
    pub file_range: FileRange,
    pub flags: FieldFlags,
    pub name: StringRef,
    #[declio(with = "codecs::list")]
    pub annotations: Vec<Expr>,
    pub typ: Type,
    #[declio(with = "codecs::option")]
    pub initializer: Option<Box<Expr>>,
}

#[derive(Debug, Default, Encode, Decode)]
pub struct Constructor {
    pub tag: ConstructorTag,
    pub canonical_name: CanonicalNameRef,
    pub file_uri: UriRef,
    pub offset: FileOffset,
    pub definition_range: FileRange,
    pub flags: ConstructorFlags,
    pub name: CanonicalNameRef,
    #[declio(with = "codecs::list")]
    pub annotations: Vec<Expr>,
    pub function: Box<Function>,
    #[declio(with = "codecs::list")]
    pub initializers: Vec<Initializer>,
}

#[derive(Debug, Default, Encode, Decode)]
// TODO
pub struct RedirectingFactory {}

#[derive(Debug, Default, Encode, Decode)]
pub struct Function {
    pub tag: FunctionTag,
    pub file_range: FileRange,
    pub async_marker: u8,
    pub dart_async_marker: u8,
    #[declio(with = "codecs::list")]
    pub type_params: Vec<TypeParameter>,
    pub param_count: Uint,
    pub required_param_count: Uint,
    #[declio(with = "codecs::list")]
    pub positional_params: Vec<VarDecl>,
    #[declio(with = "codecs::list")]
    pub named_params: Vec<VarDecl>,
    pub return_type: Type,
    #[declio(with = "codecs::option")]
    pub future_value_type: Option<Type>,
    #[declio(with = "codecs::option")]
    pub body: Option<Box<Stmt>>,
}

#[derive(Debug, Default, Encode, Decode)]
pub struct Typedef {
    pub canonical_name: CanonicalNameRef,
    pub file_uri: UriRef,
    pub offset: FileOffset,
    pub name: StringRef,
    #[declio(with = "codecs::list")]
    pub annotations: Vec<Expr>,
    #[declio(with = "codecs::list")]
    pub type_params: Vec<TypeParameter>,
    pub typ: Type,
    #[declio(with = "codecs::list")]
    pub type_params_of_function: Vec<TypeParameter>,
    #[declio(with = "codecs::list")]
    pub positional_params: Vec<VarDecl>,
    #[declio(with = "codecs::list")]
    pub named_params: Vec<VarDecl>,
}

#[derive(Debug, Encode, Decode)]
#[declio(id_type = "u8")]
pub enum Type {
    #[declio(id = "98")]
    Never(Nullable),
    #[declio(id = "90")]
    Invalid,
    #[declio(id = "91")]
    Dynamic,
    #[declio(id = "92")]
    Void,
    #[declio(id = "93")]
    GenericInterface {
        nullable: Nullable,
        class: CanonicalNameRef,
        #[declio(with = "codecs::list")]
        type_args: Vec<Type>,
    },
    #[declio(id = "96")]
    Interface {
        nullable: Nullable,
        class: CanonicalNameRef,
    },
    #[declio(id = "94")]
    GenericFunction(Box<FunctionType>),
    #[declio(id = "97")]
    Function {
        nullable: Nullable,
        #[declio(with = "codecs::list")]
        positional_params: Vec<Type>,
        return_type: Box<Type>,
    },
    #[declio(id = "95")]
    TypeParam {
        nullable: Nullable,
        index: Uint,
        #[declio(with = "codecs::option")]
        bound: Option<Box<Type>>,
    },
}

impl Default for Type {
    fn default() -> Self {
        Self::Dynamic
    }
}

#[derive(Debug, Default, Encode, Decode)]
pub struct FunctionType {
    pub nullable: Nullable,
    #[declio(with = "codecs::list")]
    pub type_params: Vec<TypeParameter>,
    pub required_param_count: Uint,
    pub total_param_count: Uint,
    #[declio(with = "codecs::list")]
    pub positional_params: Vec<Type>,
    #[declio(with = "codecs::list")]
    pub named_params: Vec<NamedType>,
    #[declio(with = "codecs::option")]
    pub typedef: Option<TypedefType>,
    pub return_type: Type,
}

#[derive(Debug, Encode, Decode)]
pub struct NamedType {
    pub name: StringRef,
    pub typ: Type,
    pub flags: u8,
}

#[derive(Debug, Encode, Decode)]
pub struct TypeParameter {
    pub flags: u8,
    #[declio(with = "codecs::list")]
    pub annotations: Vec<Expr>,
    pub variance: Variance,
    pub name: StringRef,
    pub bound: Type,
    pub default_type: Type,
}

#[derive(Debug, Default, Encode, Decode)]
pub struct TypedefType {
    pub tag: TypedefTypeTag,
    pub nullable: Nullable,
    pub reference: CanonicalNameRef,
    #[declio(with = "codecs::list")]
    pub type_args: Vec<Type>,
}

#[derive(Debug, Encode, Decode)]
pub struct Combinator {
    pub flags: CombinatorFlags,
    #[declio(with = "codecs::list")]
    pub names: Vec<StringRef>,
}

#[derive(Debug, Default, Encode, Decode)]
// TODO
pub struct ExtensionShowClause {}

#[derive(Debug, Encode, Decode)]
#[declio(id_type = "u8")]
pub enum Initializer {
    #[declio(id = "7")]
    Invalid { is_synthetic: u8 },
    #[declio(id = "8")]
    Field {
        is_synthetic: u8,
        field: CanonicalNameRef,
        value: Box<Expr>,
    },
    #[declio(id = "9")]
    Super {
        is_synthetic: u8,
        offset: FileOffset,
        target: CanonicalNameRef,
        arguments: Box<Arguments>,
    },
    #[declio(id = "10")]
    Redirect {
        is_synthetic: u8,
        offset: FileOffset,
        target: CanonicalNameRef,
        arguments: Box<Arguments>,
    },
    #[declio(id = "11")]
    Local { is_synthetic: u8, var: VarDecl },
    #[declio(id = "12")]
    Assert { is_synthetic: u8, stmt: Assert },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[declio(id_type = "u8")]
pub enum Nullable {
    #[declio(id = "0")]
    True,
    #[declio(id = "1")]
    False,
    #[declio(id = "2")]
    Neither,
    #[declio(id = "3")]
    Legacy,
}

impl Default for Nullable {
    fn default() -> Self {
        Self::False
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[declio(id_type = "u8")]
pub enum ProcedureKind {
    #[declio(id = "0")]
    Method,
    #[declio(id = "1")]
    Getter,
    #[declio(id = "2")]
    Setter,
    #[declio(id = "3")]
    Operator,
    #[declio(id = "4")]
    Factory,
}

impl Default for ProcedureKind {
    fn default() -> Self {
        Self::Method
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[declio(id_type = "u8")]
pub enum ProcedureStubKind {
    #[declio(id = "0")]
    Regular,
    #[declio(id = "1")]
    AbstractForwarder,
    #[declio(id = "2")]
    ConcreteForwarder,
    #[declio(id = "3")]
    NoSuchMethodForwarder,
    #[declio(id = "4")]
    MemberSignature,
    #[declio(id = "5")]
    AbstractMixin,
    #[declio(id = "6")]
    ConcreteMixin,
}

impl Default for ProcedureStubKind {
    fn default() -> Self {
        Self::Regular
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[declio(id_type = "u8")]
pub enum Variance {
    #[declio(id = "0")]
    Unrelated,
    #[declio(id = "1")]
    Covariant,
    #[declio(id = "2")]
    Contravariant,
    #[declio(id = "3")]
    Invariant,
}

#[derive(Debug, Default)]
pub struct ClassIndex;

impl Decode<usize> for ClassIndex {
    fn decode<R>(proc_count: usize, endian: Endian, reader: &mut R) -> Result<Self, declio::Error>
    where
        R: std::io::Read,
    {
        for _ in 0..proc_count + 1 {
            u32::decode((), endian, reader)?;
        }
        let size = u32::decode((), endian, reader)?;
        if size as usize != proc_count {
            Err(declio::Error::new("Index does not match procedure count"))
        } else {
            Ok(ClassIndex)
        }
    }
}

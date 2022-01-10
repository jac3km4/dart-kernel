use declio::{Decode, Encode};

use crate::codecs;
use crate::flags::{DynamicCastFlags, InvocationFlags, VarDeclFlags};
use crate::node::{Function, Type, TypeParameter};
use crate::prim::*;

#[derive(Debug, Encode, Decode)]
#[declio(id_type = "u8")]
pub enum Expr {
    #[declio(id = "19")]
    Invalid {
        offset: FileOffset,
        message: StringRef,
        #[declio(with = "codecs::option")]
        expression: Option<Box<Expr>>,
    },
    #[declio(id = "20")]
    VarGet {
        offset: FileOffset,
        var_decl_position: Uint,
        var: VarRef,
        #[declio(with = "codecs::option")]
        promoted_type: Option<Box<Type>>,
    },
    #[declio(id = "128")]
    SpecializedVarGet0(FileOffset, Uint),
    #[declio(id = "129")]
    SpecializedVarGet1(FileOffset, Uint),
    #[declio(id = "130")]
    SpecializedVarGet2(FileOffset, Uint),
    #[declio(id = "131")]
    SpecializedVarGet3(FileOffset, Uint),
    #[declio(id = "132")]
    SpecializedVarGet4(FileOffset, Uint),
    #[declio(id = "133")]
    SpecializedVarGet5(FileOffset, Uint),
    #[declio(id = "134")]
    SpecializedVarGet6(FileOffset, Uint),
    #[declio(id = "135")]
    SpecializedVarGet7(FileOffset, Uint),
    #[declio(id = "21")]
    VarSet {
        offset: FileOffset,
        var_decl_position: Uint,
        var: VarRef,
        value: Box<Expr>,
    },
    #[declio(id = "136")]
    SpecializedVarSet0(FileOffset, Uint, Box<Expr>),
    #[declio(id = "137")]
    SpecializedVarSet1(FileOffset, Uint, Box<Expr>),
    #[declio(id = "138")]
    SpecializedVarSet2(FileOffset, Uint, Box<Expr>),
    #[declio(id = "139")]
    SpecializedVarSet3(FileOffset, Uint, Box<Expr>),
    #[declio(id = "140")]
    SpecializedVarSet4(FileOffset, Uint, Box<Expr>),
    #[declio(id = "141")]
    SpecializedVarSet5(FileOffset, Uint, Box<Expr>),
    #[declio(id = "142")]
    SpecializedVarSet6(FileOffset, Uint, Box<Expr>),
    #[declio(id = "143")]
    SpecializedVarSet7(FileOffset, Uint, Box<Expr>),
    #[declio(id = "24")]
    SuperPropGet {
        offset: FileOffset,
        name: StringRef,
        interface_target: CanonicalNameRef,
        interface_target_origin: CanonicalNameRef,
    },
    #[declio(id = "24")]
    SuperPropSet {
        offset: FileOffset,
        name: StringRef,
        value: Box<Expr>,
        interface_target: CanonicalNameRef,
        interface_target_origin: CanonicalNameRef,
    },
    #[declio(id = "118")]
    InstanceGet {
        kind: InstanceAccessKind,
        offset: FileOffset,
        receiver: Box<Expr>,
        name: StringRef,
        typ: Box<Type>,
        interface_target: CanonicalNameRef,
        interface_target_origin: CanonicalNameRef,
    },
    #[declio(id = "119")]
    InstanceSet {
        kind: InstanceAccessKind,
        offset: FileOffset,
        receiver: Box<Expr>,
        name: StringRef,
        value: Box<Expr>,
        interface_target: CanonicalNameRef,
        interface_target_origin: CanonicalNameRef,
    },
    #[declio(id = "121")]
    InstanceTearOff {
        kind: InstanceAccessKind,
        offset: FileOffset,
        receiver: Box<Expr>,
        name: StringRef,
        typ: Box<Type>,
        interface_target: CanonicalNameRef,
        interface_target_origin: CanonicalNameRef,
    },
    #[declio(id = "122")]
    DynamicGet {
        kind: DynamicAccessKind,
        offset: FileOffset,
        receiver: Box<Expr>,
        name: StringRef,
    },
    #[declio(id = "123")]
    DynamicSet {
        kind: DynamicAccessKind,
        offset: FileOffset,
        receiver: Box<Expr>,
        name: StringRef,
        value: Box<Expr>,
    },
    #[declio(id = "26")]
    StaticGet {
        offset: FileOffset,
        target: CanonicalNameRef,
    },
    #[declio(id = "27")]
    StaticSet {
        offset: FileOffset,
        target: CanonicalNameRef,
        value: Box<Expr>,
    },
    #[declio(id = "17")]
    StaticTearOff {
        offset: FileOffset,
        target: CanonicalNameRef,
    },
    #[declio(id = "60")]
    ConstructorTearOff {
        offset: FileOffset,
        target: CanonicalNameRef,
    },
    #[declio(id = "84")]
    RedirectingFactoryTearOff {
        offset: FileOffset,
        target: CanonicalNameRef,
    },
    #[declio(id = "83")]
    TypedefTearOff {
        #[declio(with = "codecs::list")]
        type_params: Vec<TypeParameter>,
        expr: Box<Expr>,
        #[declio(with = "codecs::list")]
        type_args: Vec<Type>,
    },
    #[declio(id = "120")]
    InstanceInvoke {
        kind: InstanceAccessKind,
        flags: InvocationFlags,
        offset: FileOffset,
        receiver: Box<Expr>,
        name: StringRef,
        arguments: Box<Arguments>,
        function_type: Box<Type>,
        interface_target: CanonicalNameRef,
        interface_target_origin: CanonicalNameRef,
    },
    #[declio(id = "89")]
    InstanceGetterInvoke {
        kind: InstanceAccessKind,
        flags: InvocationFlags,
        offset: FileOffset,
        receiver: Box<Expr>,
        name: StringRef,
        arguments: Box<Arguments>,
        function_type: Box<Type>,
        interface_target: CanonicalNameRef,
        interface_target_origin: CanonicalNameRef,
    },
    #[declio(id = "124")]
    DynamicInvoke {
        kind: DynamicAccessKind,
        offset: FileOffset,
        receiver: Box<Expr>,
        name: StringRef,
        arguments: Box<Arguments>,
    },
    #[declio(id = "125")]
    FunctionInvoke {
        kind: FunctionAccessKind,
        offset: FileOffset,
        receiver: Box<Expr>,
        arguments: Box<Arguments>,
        function_type: Box<Type>,
    },
    #[declio(id = "126")]
    FunctionTearOff {
        offset: FileOffset,
        receiver: Box<Expr>,
    },
    #[declio(id = "127")]
    LocalFunctionInvoke {
        offset: FileOffset,
        var_decl_position: Uint,
        var_ref: VarRef,
        arguments: Box<Arguments>,
        function_type: Box<Type>,
    },
    #[declio(id = "29")]
    SuperMethodInvoke {
        offset: FileOffset,
        name: StringRef,
        arguments: Box<Arguments>,
        interface_target: CanonicalNameRef,
        interface_target_origin: CanonicalNameRef,
    },
    #[declio(id = "30")]
    StaticInvoke {
        offset: FileOffset,
        target: CanonicalNameRef,
        arguments: Box<Arguments>,
    },
    #[declio(id = "18")]
    ConstStaticInvoke {
        offset: FileOffset,
        target: CanonicalNameRef,
        arguments: Box<Arguments>,
    },
    #[declio(id = "31")]
    ConstructorInvoke {
        offset: FileOffset,
        constructor: CanonicalNameRef,
        arguments: Box<Arguments>,
    },
    #[declio(id = "32")]
    ConstConstructorInvoke {
        offset: FileOffset,
        constructor: CanonicalNameRef,
        arguments: Box<Arguments>,
    },
    #[declio(id = "15")]
    EqualsNull { offset: FileOffset, expr: Box<Expr> },
    #[declio(id = "16")]
    Equals {
        offset: FileOffset,
        left: Box<Expr>,
        right: Box<Expr>,
        function_type: Box<Type>,
        interface_target: CanonicalNameRef,
        interface_target_origin: CanonicalNameRef,
    },
    #[declio(id = "33")]
    Not { operand: Box<Expr> },
    #[declio(id = "117")]
    NullCheck {
        offset: FileOffset,
        operand: Box<Expr>,
    },
    #[declio(id = "34")]
    LogicalOp {
        left: Box<Expr>,
        operator: LogicalOp,
        right: Box<Expr>,
    },
    #[declio(id = "35")]
    Conditional {
        condition: Box<Expr>,
        then: Box<Expr>,
        otherwise: Box<Expr>,
        #[declio(with = "codecs::option")]
        static_type: Option<Box<Type>>,
    },
    #[declio(id = "36")]
    StringConcat {
        offset: FileOffset,
        #[declio(with = "codecs::list")]
        expressions: Vec<Expr>,
    },
    #[declio(id = "111")]
    ListConcat {
        offset: FileOffset,
        type_arg: Box<Type>,
        #[declio(with = "codecs::list")]
        expressions: Vec<Expr>,
    },
    #[declio(id = "112")]
    SetConcat {
        offset: FileOffset,
        type_arg: Box<Type>,
        #[declio(with = "codecs::list")]
        expressions: Vec<Expr>,
    },
    #[declio(id = "113")]
    MapConcat {
        offset: FileOffset,
        key_type: Box<Type>,
        value_type: Box<Type>,
        #[declio(with = "codecs::list")]
        expressions: Vec<Expr>,
    },
    #[declio(id = "113")]
    InstanceCreate(Box<InstanceCreate>),
    #[declio(id = "116")]
    FileUriLit {
        file_uri: UriRef,
        offset: FileOffset,
        expression: Box<Expr>,
    },
    #[declio(id = "37")]
    IsInstanceOf {
        offset: FileOffset,
        flags: u8,
        operand: Box<Expr>,
        typ: Box<Type>,
    },
    #[declio(id = "38")]
    AsInstanceOf {
        offset: FileOffset,
        flags: DynamicCastFlags,
        operand: Box<Expr>,
        typ: Box<Type>,
    },
    #[declio(id = "39")]
    StringLit(StringRef),
    #[declio(id = "144")]
    IntLitMinus3,
    #[declio(id = "145")]
    IntLitMinus2,
    #[declio(id = "146")]
    IntLitMinus1,
    #[declio(id = "147")]
    IntLit0,
    #[declio(id = "148")]
    IntLit1,
    #[declio(id = "149")]
    IntLit2,
    #[declio(id = "150")]
    IntLit3,
    #[declio(id = "151")]
    IntLit4,
    #[declio(id = "55")]
    PosIntLit(Uint),
    #[declio(id = "56")]
    NegIntLit(Uint),
    #[declio(id = "57")]
    BigIntLit(StringRef),
    #[declio(id = "40")]
    DoubleLit(f64),
    #[declio(id = "41")]
    TrueLit,
    #[declio(id = "42")]
    FalseLit,
    #[declio(id = "43")]
    NullLit,
    #[declio(id = "44")]
    SymbolLit(StringRef),
    #[declio(id = "45")]
    TypeLit(Box<Type>),
    #[declio(id = "46")]
    This,
    #[declio(id = "47")]
    Rethrow(FileOffset),
    #[declio(id = "48")]
    Throw(FileOffset, Box<Expr>),
    #[declio(id = "49")]
    ListLit {
        offset: FileOffset,
        type_argument: Box<Type>,
        #[declio(with = "codecs::list")]
        values: Vec<Expr>,
    },
    #[declio(id = "109")]
    SetLit {
        offset: FileOffset,
        type_argument: Box<Type>,
        #[declio(with = "codecs::list")]
        values: Vec<Expr>,
    },
    #[declio(id = "50")]
    MapLit {
        offset: FileOffset,
        key_type: Box<Type>,
        value_type: Box<Type>,
        #[declio(with = "codecs::list")]
        values: Vec<LabeledExpr<Expr>>,
    },
    #[declio(id = "51")]
    Await(Box<Expr>),
    #[declio(id = "52")]
    Function(FileOffset, Box<Function>),
    #[declio(id = "53")]
    Let(FileOffset, Box<VarDecl>, Box<Expr>),
    #[declio(id = "82")]
    Block {
        #[declio(with = "codecs::list")]
        body: Vec<Stmt>,
        value: Box<Expr>,
    },
    #[declio(id = "54")]
    Instantiation {
        expr: Box<Expr>,
        #[declio(with = "codecs::list")]
        type_args: Vec<Type>,
    },
    #[declio(id = "14")]
    LoadLibrary(DependencyRef),
    #[declio(id = "13")]
    CheckLibraryIsLoaded(DependencyRef),
    #[declio(id = "106")]
    Constant {
        offset: FileOffset,
        typ: Box<Type>,
        constant: ConstantRef,
    },
}

#[derive(Debug, Encode, Decode)]
#[declio(id_type = "u8")]
pub enum Stmt {
    #[declio(id = "61")]
    Expr(Box<Expr>),
    #[declio(id = "62")]
    Block {
        range: FileRange,
        #[declio(with = "codecs::list")]
        statements: Vec<Stmt>,
    },
    #[declio(id = "81")]
    AssertBlock {
        #[declio(with = "codecs::list")]
        statements: Vec<Stmt>,
    },
    #[declio(id = "63")]
    Empty,
    #[declio(id = "64")]
    Assert(Assert),
    #[declio(id = "65")]
    Labeled { body: Box<Stmt> },
    #[declio(id = "66")]
    Break { offset: FileOffset, label: Uint },
    #[declio(id = "67")]
    While {
        offset: FileOffset,
        condition: Box<Expr>,
        body: Box<Stmt>,
    },
    #[declio(id = "68")]
    Do {
        offset: FileOffset,
        body: Box<Stmt>,
        condition: Box<Expr>,
    },
    #[declio(id = "69")]
    For {
        offset: FileOffset,
        #[declio(with = "codecs::list")]
        vars: Vec<VarDecl>,
        #[declio(with = "codecs::option")]
        condition: Option<Box<Expr>>,
        #[declio(with = "codecs::list")]
        updates: Vec<Expr>,
        body: Box<Stmt>,
    },
    #[declio(id = "70")]
    ForIn {
        offset: FileOffset,
        body_offset: FileOffset,
        var: Box<VarDecl>,
        iterable: Box<Expr>,
        body: Box<Stmt>,
    },
    #[declio(id = "80")]
    AsyncForIn {
        offset: FileOffset,
        body_offset: FileOffset,
        var: Box<VarDecl>,
        iterable: Box<Expr>,
        body: Box<Stmt>,
    },
    #[declio(id = "71")]
    Switch {
        offset: FileOffset,
        expr: Box<Expr>,
        #[declio(with = "codecs::list")]
        cases: Vec<SwitchCase>,
    },
    #[declio(id = "72")]
    ContinueSwitch {
        offset: FileOffset,
        case_index: Uint,
    },
    #[declio(id = "73")]
    If {
        offset: FileOffset,
        condition: Box<Expr>,
        then: Box<Stmt>,
        otherwise: Box<Stmt>,
    },
    #[declio(id = "74")]
    Return {
        offset: FileOffset,
        #[declio(with = "codecs::option")]
        expr: Option<Box<Expr>>,
    },
    #[declio(id = "75")]
    TryCatch {
        body: Box<Stmt>,
        flags: u8,
        #[declio(with = "codecs::list")]
        catches: Vec<Catch>,
    },
    #[declio(id = "76")]
    TryFinally {
        body: Box<Stmt>,
        finalizer: Box<Stmt>,
    },
    #[declio(id = "77")]
    Yield {
        offset: FileOffset,
        flags: u8,
        expr: Box<Expr>,
    },
    #[declio(id = "78")]
    VarDecl { var: Box<VarDecl> },
    #[declio(id = "79")]
    FunctionDecl {
        offset: FileOffset,
        var: Box<VarDecl>,
        function: Box<Function>,
    },
}

#[derive(Debug, Encode, Decode)]
pub struct Assert {
    pub condition: Box<Expr>,
    pub file_range: FileRange,
    #[declio(with = "codecs::option")]
    pub message: Option<Box<Expr>>,
}

#[derive(Debug, Encode, Decode)]
pub struct InstanceCreate {
    pub offset: FileOffset,
    pub class: CanonicalNameRef,
    #[declio(with = "codecs::list")]
    pub type_args: Vec<Type>,
    #[declio(with = "codecs::list")]
    pub field_values: Vec<LabeledExpr<CanonicalNameRef>>,
    #[declio(with = "codecs::list")]
    pub asserts: Vec<Assert>,
    #[declio(with = "codecs::list")]
    pub unused_args: Vec<Expr>,
}

#[derive(Debug, Encode, Decode)]
pub struct LabeledExpr<A: Encode + Decode> {
    pub label: A,
    pub value: Expr,
}

#[derive(Debug, Encode, Decode)]
#[declio(id_type = "u8")]
pub enum Constant {
    #[declio(id = "0")]
    Null,
    #[declio(id = "1")]
    Bool(u8),
    #[declio(id = "2")]
    Int(IntLit),
    #[declio(id = "3")]
    Double(f64),
    #[declio(id = "4")]
    String(StringRef),
    #[declio(id = "5")]
    Symbol(CanonicalNameRef, StringRef),
    #[declio(id = "6")]
    Map {
        key_type: Box<Type>,
        value_type: Box<Type>,
        #[declio(with = "codecs::list")]
        values: Vec<LabeledConstant<ConstantRef>>,
    },
    #[declio(id = "7")]
    List {
        typ: Box<Type>,
        #[declio(with = "codecs::list")]
        values: Vec<ConstantRef>,
    },
    #[declio(id = "13")]
    Set {
        typ: Box<Type>,
        #[declio(with = "codecs::list")]
        values: Vec<ConstantRef>,
    },
    #[declio(id = "8")]
    Instance {
        class: CanonicalNameRef,
        #[declio(with = "codecs::list")]
        type_args: Vec<Type>,
        #[declio(with = "codecs::list")]
        values: Vec<LabeledConstant<CanonicalNameRef>>,
    },
    #[declio(id = "9")]
    Instantiation {
        tear_off_constant: ConstantRef,
        #[declio(with = "codecs::list")]
        type_args: Vec<Type>,
    },
    #[declio(id = "10")]
    StaticTearOff { static_procedure: CanonicalNameRef },
    #[declio(id = "11")]
    TypeLiteral(Type),
    #[declio(id = "12")]
    Unevaluated(Expr),
    #[declio(id = "14")]
    TypedefTearOff {
        #[declio(with = "codecs::list")]
        parameters: Vec<Type>,
        static_procedure: CanonicalNameRef,
        #[declio(with = "codecs::list")]
        types: Vec<Type>,
    },
    #[declio(id = "15")]
    ConstructorTearOff { static_procedure: CanonicalNameRef },
    #[declio(id = "16")]
    RedirectingFactoryTearOff { static_procedure: CanonicalNameRef },
}

#[derive(Debug, Encode, Decode)]
pub struct LabeledConstant<A: Encode + Decode> {
    pub key: A,
    pub value: ConstantRef,
}

#[derive(Debug, Encode, Decode)]
#[declio(id_type = "u8")]
pub enum IntLit {
    #[declio(id = "144")]
    SpecializedMinus3,
    #[declio(id = "145")]
    SpecializedMinus2,
    #[declio(id = "146")]
    SpecializedMinus1,
    #[declio(id = "147")]
    Specialized0,
    #[declio(id = "148")]
    Specialized1,
    #[declio(id = "149")]
    Specialized2,
    #[declio(id = "150")]
    Specialized3,
    #[declio(id = "151")]
    Specialized4,
    #[declio(id = "55")]
    Pos(Uint),
    #[declio(id = "56")]
    Neg(Uint),
    #[declio(id = "57")]
    Big(StringRef),
}

#[derive(Debug, Encode, Decode)]
pub struct Arguments {
    pub num_args: Uint,
    #[declio(with = "codecs::list")]
    pub types: Vec<Type>,
    #[declio(with = "codecs::list")]
    pub positional: Vec<Expr>,
    #[declio(with = "codecs::list")]
    pub named: Vec<LabeledExpr<StringRef>>,
}

impl Arguments {
    pub fn positional(types: Vec<Type>, values: Vec<Expr>) -> Self {
        Self {
            num_args: Uint(values.len() as u32),
            types,
            positional: values,
            named: vec![],
        }
    }
}

#[derive(Debug, Encode, Decode)]
pub struct Catch {
    pub offset: FileOffset,
    pub guard: Type,
    #[declio(with = "codecs::option")]
    pub exception: Option<VarDecl>,
    #[declio(with = "codecs::option")]
    pub stack_trace: Option<VarDecl>,
    pub body: Box<Stmt>,
}

#[derive(Debug, Encode, Decode)]
pub struct SwitchCase {
    #[declio(with = "codecs::list")]
    pub exprs: Vec<LabeledExpr<FileOffset>>,
    pub is_default: u8,
    pub body: Stmt,
}

#[derive(Debug, Default, Encode, Decode)]
pub struct VarDecl {
    pub offset: FileOffset,
    pub equals_sign_offset: FileOffset,
    #[declio(with = "codecs::list")]
    pub annotations: Vec<Expr>,
    pub flags: VarDeclFlags,
    pub name: StringRef,
    pub typ: Box<Type>,
    #[declio(with = "codecs::option")]
    pub initializer: Option<Box<Expr>>,
}

#[derive(Debug, Encode, Decode)]
#[declio(id_type = "u8")]
pub enum InstanceAccessKind {
    #[declio(id = "0")]
    Instance,
    #[declio(id = "1")]
    Object,
    #[declio(id = "2")]
    Inapplicable,
    #[declio(id = "3")]
    Nullable,
}

#[derive(Debug, Encode, Decode)]
#[declio(id_type = "u8")]
pub enum DynamicAccessKind {
    #[declio(id = "0")]
    Dynamic,
    #[declio(id = "1")]
    Never,
    #[declio(id = "2")]
    Invalid,
    #[declio(id = "3")]
    Unresolved,
}

#[derive(Debug, Encode, Decode)]
#[declio(id_type = "u8")]
pub enum FunctionAccessKind {
    #[declio(id = "0")]
    Function,
    #[declio(id = "1")]
    FunctionType,
    #[declio(id = "2")]
    Inapplicable,
    #[declio(id = "3")]
    Nullable,
}

#[derive(Debug, Encode, Decode)]
#[declio(id_type = "u8")]
pub enum LogicalOp {
    #[declio(id = "0")]
    And,
    #[declio(id = "1")]
    Or,
}

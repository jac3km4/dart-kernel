use std::io;

use declio::ctx::Endian;
use declio::{Decode, Encode};
use modular_bitfield::prelude::*;

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct LibraryFlags {
    pub is_synthetic: bool,
    pub is_non_nullable_by_default: bool,
    pub nnb_mode: B2,
    #[skip]
    remainder: B4,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct DependencyFlags {
    pub is_export: bool,
    pub is_deferred: bool,
    #[skip]
    remainder: B6,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct CombinatorFlags {
    pub is_show: bool,
    #[skip]
    remainder: B7,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct ClassFlags {
    pub is_abstract: bool,
    pub is_enum: bool,
    pub is_anonymous_mixin: bool,
    pub is_eliminated_mixin: bool,
    pub is_mixin_declaration: bool,
    pub has_const_constructor: bool,
    pub is_macro: bool,
    #[skip]
    remainder: B1,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct FieldFlags {
    pub is_internal_impl: bool,
    #[skip]
    remainder: B6,
    pub uint_extension: B1,
    pub is_final: bool,
    pub is_const: bool,
    pub is_static: bool,
    pub is_covariant_by_decl: bool,
    pub is_covariant_by_class: bool,
    pub is_late: bool,
    pub is_extension_member: bool,
    pub is_non_nullable_by_default: bool,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct ConstructorFlags {
    pub is_const: bool,
    pub is_external: bool,
    pub is_synthetic: bool,
    pub is_non_nullable_by_default: bool,
    #[skip]
    remainder: B4,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct ProcedureFlags {
    pub is_static: bool,
    pub is_abstract: bool,
    pub is_external: bool,
    pub is_const: bool,
    pub is_redirecting_factory: bool,
    pub is_extension_member: bool,
    pub is_non_nullable_by_default: bool,
    #[skip]
    remainder: B1,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct InvocationFlags {
    pub is_invariant: bool,
    pub is_bounds_safe: bool,
    #[skip]
    remainder: B6,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct DynamicCastFlags {
    pub is_type_error: bool,
    pub is_covariance_check: bool,
    pub is_for_dynamic: bool,
    pub is_for_non_nullable_by_default: bool,
    #[skip]
    remainder: B4,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct VarDeclFlags {
    pub is_final: bool,
    pub is_const: bool,
    pub is_initializing_formal: bool,
    pub is_covariant_by_decl: bool,
    pub is_convariant_by_class: bool,
    pub is_late: bool,
    pub is_required: bool,
    pub is_lowered: bool,
}

macro_rules! impl_flags_codec {
    ($ty:ty) => {
        impl Encode for $ty {
            #[inline]
            fn encode<W>(&self, _ctx: (), _endian: Endian, out: &mut W) -> Result<(), declio::Error>
            where
                W: io::Write,
            {
                out.write_all(&self.into_bytes())?;
                Ok(())
            }
        }

        impl Decode for $ty {
            #[inline]
            fn decode<R>(ctx: (), endian: Endian, inp: &mut R) -> Result<Self, declio::Error>
            where
                R: io::Read,
            {
                Ok(Self::from_bytes(Decode::decode(ctx, endian, inp)?))
            }
        }

        impl Default for $ty {
            #[inline]
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

impl_flags_codec!(LibraryFlags);
impl_flags_codec!(DependencyFlags);
impl_flags_codec!(CombinatorFlags);
impl_flags_codec!(ClassFlags);
impl_flags_codec!(FieldFlags);
impl_flags_codec!(ConstructorFlags);
impl_flags_codec!(ProcedureFlags);
impl_flags_codec!(InvocationFlags);
impl_flags_codec!(DynamicCastFlags);
impl_flags_codec!(VarDeclFlags);

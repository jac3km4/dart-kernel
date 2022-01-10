use std::hash::Hash;
use std::marker::PhantomData;
use std::rc::Rc;
use std::{fmt, io};

use declio::ctx::{Endian, Len};
use declio::{magic_bytes, Decode, Encode};
use hex_magic::hex;

use crate::codecs;
use crate::component::{LibraryDependency, SourceInfo};
use crate::expr::Constant;

pub const ENDIANESS: Endian = Endian::Big;

magic_bytes! {
    #[derive(Debug)]
    pub HeaderMagic(&hex!("90ABCDEF"));
    #[derive(Debug, Default)]
    pub ClassTag(&hex!("02"));
    #[derive(Debug, Default)]
    pub FunctionTag(&hex!("03"));
    #[derive(Debug, Default)]
    pub FieldTag(&hex!("04"));
    #[derive(Debug, Default)]
    pub ConstructorTag(&hex!("05"));
    #[derive(Debug, Default)]
    pub ProcedureTag(&hex!("06"));
    #[derive(Debug, Default)]
    pub ExtensionTag(&hex!("73"));
    #[derive(Debug, Default)]
    pub TypedefTypeTag(&hex!("57"));
}

#[derive(PartialEq, Eq, Hash, Encode, Decode)]
pub struct ComponentRef<A> {
    index: Uint,
    phantom: PhantomData<A>,
}

impl<A> ComponentRef<A> {
    pub const UNDEFINED: Self = Self::new(Uint(0));

    pub(crate) const fn new(index: Uint) -> Self {
        Self {
            index,
            phantom: PhantomData,
        }
    }
}

impl<A> From<ComponentRef<A>> for u32 {
    fn from(pr: ComponentRef<A>) -> Self {
        pr.index.0 as u32
    }
}

impl<A> Default for ComponentRef<A> {
    fn default() -> Self {
        Self::UNDEFINED
    }
}

impl<A> Clone for ComponentRef<A> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            phantom: self.phantom,
        }
    }
}

impl<A> Copy for ComponentRef<A> {}

impl<A> fmt::Debug for ComponentRef<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ComponentRef").field(&self.index).finish()
    }
}

pub type StringRef = ComponentRef<String>;
pub type CanonicalNameRef = ComponentRef<CanonicalName>;
pub type UriRef = ComponentRef<SourceInfo>;
pub type ConstantRef = ComponentRef<Constant>;
pub type DependencyRef = ComponentRef<LibraryDependency>;
pub type VarRef = Uint;

#[derive(Debug, Default, Clone, Copy, Encode, Decode)]
pub struct FileOffset(pub Uint);

#[derive(Debug, Default, Clone, Encode, Decode)]
pub struct FileRange {
    pub start: FileOffset,
    pub end: FileOffset,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Encode, Decode)]
pub struct CanonicalName {
    pub parent: CanonicalNameRef,
    pub name: StringRef,
}

pub type StrRef = CowRcStr<'static>;

#[derive(Clone, Eq)]
pub enum CowRcStr<'a> {
    Borrowed(&'a str),
    Shared(Rc<String>),
}

impl<'a> CowRcStr<'a> {
    pub fn as_str(&'a self) -> &'a str {
        match self {
            CowRcStr::Borrowed(str) => str,
            CowRcStr::Shared(rc) => rc.as_str(),
        }
    }

    pub fn to_owned<'b>(&'a self) -> CowRcStr<'b> {
        match self {
            CowRcStr::Borrowed(str) => CowRcStr::Shared(Rc::new((*str).to_owned())),
            CowRcStr::Shared(rc) => CowRcStr::Shared(rc.clone()),
        }
    }
}

impl<'a> PartialEq for CowRcStr<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Hash for CowRcStr<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl fmt::Debug for CowRcStr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Borrowed(str) => f.write_fmt(format_args!(r#""{str}""#)),
            Self::Shared(str) => f.write_fmt(format_args!(r#""{str}""#)),
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uint(pub u32);

impl From<u32> for Uint {
    fn from(n: u32) -> Self {
        Uint(n)
    }
}

impl fmt::Debug for Uint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl Decode for Uint {
    fn decode<R: io::Read>(
        ctx: (),
        _endian: Endian,
        reader: &mut R,
    ) -> Result<Self, declio::Error> {
        let b1 = u8::decode(ctx, ENDIANESS, reader)?;
        if b1 & 0b10000000 == 0 {
            Ok(Uint(b1.into()))
        } else if b1 & 0b01000000 == 0 {
            let b2 = u8::decode(ctx, ENDIANESS, reader)?;
            let res = (b1 as u32 & 0b01111111) << 8 | b2 as u32;
            Ok(res.into())
        } else {
            let b2 = u8::decode(ctx, ENDIANESS, reader)?;
            let b3 = u8::decode(ctx, ENDIANESS, reader)?;
            let b4 = u8::decode(ctx, ENDIANESS, reader)?;
            let res =
                (b1 as u32 & 0b00111111) << 24 | (b2 as u32) << 16 | (b3 as u32) << 8 | b4 as u32;
            Ok(res.into())
        }
    }
}

impl Encode for Uint {
    fn encode<W: io::Write>(
        &self,
        ctx: (),
        _endian: Endian,
        writer: &mut W,
    ) -> Result<(), declio::Error> {
        if self.0 < 0b01111111 {
            u8::encode(&(self.0 as u8), ctx, ENDIANESS, writer)
        } else if self.0 < 0b0011_1111_1111_1111 {
            u16::encode(&(self.0 as u16 | 1 << 15), ctx, ENDIANESS, writer)
        } else {
            u32::encode(&(self.0 | 3 << 30), ctx, ENDIANESS, writer)
        }
    }
}

#[derive(Debug, Default)]
pub struct Utf8(pub String);

impl Encode for Utf8 {
    fn encode<W>(&self, ctx: (), endian: Endian, writer: &mut W) -> Result<(), declio::Error>
    where
        W: io::Write,
    {
        let bytes = self.0.as_bytes();
        Uint(bytes.len() as u32).encode(ctx, endian, writer)?;
        codecs::bytes::encode(bytes, ctx, endian, writer)
    }
}

impl Decode for Utf8 {
    fn decode<R>(ctx: (), endian: Endian, reader: &mut R) -> Result<Self, declio::Error>
    where
        R: io::Read,
    {
        let len = Uint::decode(ctx, endian, reader)?;
        let bytes = codecs::bytes::decode(Len(len.0 as usize), endian, reader)?;
        let string = String::from_utf8(bytes)?;
        Ok(Utf8(string))
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[test]
    fn test_roundtrips() -> Result<(), Box<dyn Error>> {
        check_roundtrip(Uint(u32::MIN))?;
        check_roundtrip(Uint(u8::MAX.into()))?;
        check_roundtrip(Uint(u16::MAX.into()))?;
        check_roundtrip(Uint(u8::MAX as u32 + u16::MAX as u32))?;

        check_roundtrip(Uint(12))?;
        check_roundtrip(Uint(4321))?;
        check_roundtrip(Uint(123456))?;
        check_roundtrip(Uint(87654321))?;
        Ok(())
    }

    #[test]
    fn test_encodings() -> Result<(), Box<dyn Error>> {
        let bytes = declio::to_bytes(Uint(12), Endian::Big)?;
        assert_eq!(bytes, vec![0xC]);
        let bytes = declio::to_bytes(Uint(81), Endian::Big)?;
        assert_eq!(bytes, vec![0x51]);
        let bytes = declio::to_bytes(Uint(4321), Endian::Big)?;
        assert_eq!(bytes, vec![0x90, 0xE1]);
        let bytes = declio::to_bytes(Uint(123456), Endian::Big)?;
        assert_eq!(bytes, vec![0xC0, 0x1, 0xE2, 0x40]);
        let bytes = declio::to_bytes(Uint(87654321), Endian::Big)?;
        assert_eq!(bytes, vec![0xC5, 0x39, 0x7F, 0xB1]);
        Ok(())
    }

    fn check_roundtrip(val: Uint) -> Result<(), Box<dyn Error>> {
        let encoded = declio::to_bytes(val, Endian::Big)?;
        let decoded: Uint = declio::from_bytes(&encoded, Endian::Big)?;
        assert_eq!(val, decoded);
        Ok(())
    }
}

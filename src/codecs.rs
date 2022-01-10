pub mod list {
    use std::io;

    use declio::ctx::{Endian, Len};
    use declio::{Decode, Encode, Error};

    use crate::prim::Uint;

    #[inline]
    pub fn encode<W, A>(vec: &[A], ctx: (), endian: Endian, writer: &mut W) -> Result<(), Error>
    where
        W: io::Write,
        A: Encode,
    {
        let len = vec.len();
        Uint(len as u32).encode(ctx, endian, writer)?;
        vec.encode((Len(vec.len()), ctx), endian, writer)
    }

    #[inline]
    pub fn decode<R, A>(ctx: (), endian: Endian, reader: &mut R) -> Result<Vec<A>, Error>
    where
        R: io::Read,
        A: Decode,
    {
        let len = Uint::decode(ctx, endian, reader)?;
        <Vec<A>>::decode(Len(len.0 as usize), endian, reader)
    }
}

pub mod option {
    use std::io;

    use declio::ctx::Endian;
    use declio::{Decode, Encode, Error};

    pub fn encode<W, A>(
        opt: &Option<A>,
        ctx: (),
        endian: Endian,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: io::Write,
        A: Encode,
    {
        match opt {
            Some(val) => {
                1u8.encode(ctx, endian, writer)?;
                val.encode(ctx, endian, writer)
            }
            None => 0u8.encode(ctx, endian, writer),
        }
    }

    pub fn decode<R, A>(ctx: (), endian: Endian, reader: &mut R) -> Result<Option<A>, Error>
    where
        R: io::Read,
        A: Decode,
    {
        match u8::decode(ctx, endian, reader)? {
            0 => Ok(None),
            1 => Ok(Some(A::decode(ctx, endian, reader)?)),
            other => Err(Error::new(format_args!("Invalid option pattern {}", other))),
        }
    }
}

pub mod bytes {
    use std::io;

    use declio::ctx::{Endian, Len};
    use declio::Error;

    #[inline]
    pub fn encode<W, C>(bytes: &[u8], _ctx: C, _endian: Endian, writer: &mut W) -> Result<(), Error>
    where
        W: io::Write,
    {
        writer.write_all(bytes)?;
        Ok(())
    }

    #[inline]
    pub fn decode<R>(ctx: Len, _endian: Endian, reader: &mut R) -> Result<Vec<u8>, Error>
    where
        R: io::Read,
    {
        let mut bytes = vec![0; ctx.0];
        reader.read_exact(&mut bytes)?;
        Ok(bytes)
    }
}

use std::fmt::{Debug, Display, Write};

#[derive(Default)]
pub struct MsgBuf(pub Vec<u8>);

impl<Idx> std::ops::Index<Idx> for MsgBuf
where
    Idx: std::slice::SliceIndex<[u8]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}
#[allow(clippy::len_without_is_empty)]
impl MsgBuf {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<Vec<u8>> for MsgBuf {
    fn from(v: Vec<u8>) -> MsgBuf {
        MsgBuf(v)
    }
}
impl Debug for MsgBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in &self.0 {
            if *b == 1 {
                f.write_str("|")?;
            } else {
                f.write_char(*b as char)?;
            }
        }
        Ok(())
    }
}

impl Display for MsgBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in &self.0 {
            f.write_char(*b as char)?;
        }
        Ok(())
    }
}
use header::Header;
use winnow::PResult;

mod common;
mod header;
mod tags;

#[derive(Debug, PartialEq, Eq)]
pub struct Uslm<'s> {
    header: Header<'s>,
}

impl<'s> Uslm<'s> {
    pub fn parse(input: &mut &'s str) -> PResult<Self> {
        Ok(Uslm {
            header: Header::parse(input)?,
        })
    }
}

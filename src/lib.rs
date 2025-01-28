use header::Header;
use winnow::PResult;

mod attributes;
mod common;
mod doc;
mod header;
mod tags;

#[derive(Debug, PartialEq, Eq)]
pub struct Uslm<'s> {
    header: Header<'s>,
}

impl<'s> Uslm<'s> {
    pub fn parse(input: &mut &'s str) -> PResult<Self> {
        let header = Header::parse(input)?;

        Ok(Uslm { header })
    }
}

use header::Header;
use winnow::ModalResult;

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
    pub fn parse(input: &mut &'s str) -> ModalResult<Self> {
        let header = Header::parse(input)?;

        Ok(Uslm { header })
    }
}

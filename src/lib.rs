use header::Header;
use winnow::PResult;

mod common;
mod doc;
mod header;
mod tags;

#[derive(Debug, PartialEq, Eq)]
pub struct Uslm {
    header: Header,
}

impl Uslm {
    pub fn parse(input: &mut &str) -> PResult<Self> {
        Ok(Uslm {
            header: Header::parse(input)?,
        })
    }
}

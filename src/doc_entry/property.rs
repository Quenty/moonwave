use crate::{diagnostic::Diagnostics, tags::Tag};
use serde::Serialize;

use super::DocEntryParseArguments;

/// A DocEntry for a property of a class
#[derive(Debug, PartialEq, Serialize)]
pub struct PropertyDocEntry<'a> {
    name: String,
    desc: String,
    blah: Tag<'a>,
    lua_type: String,
}

impl<'a> PropertyDocEntry<'a> {
    pub(super) fn parse(_args: DocEntryParseArguments) -> Result<Self, Diagnostics> {
        unimplemented!()
    }
}

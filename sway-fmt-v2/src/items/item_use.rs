use crate::fmt::{Format, FormattedCode, Formatter};
use sway_parse::ItemUse;

impl Format for ItemUse {
    fn format(&self, _formatter: &mut Formatter) -> FormattedCode {
        todo!()
    }
}
// https://docs.microsoft.com/en-us/typography/opentype/spec/vhea

use crate::parser::Stream;
use crate::{Font, TableName, Result};

impl<'a> Font<'a> {
    pub(crate) fn number_of_vmetrics(&self) -> Result<u16> {
        const NUMBER_OF_VMETRICS_OFFSET: usize = 34;
        let data = self.table_data(TableName::VerticalHeader)?;
        Stream::read_at(data, NUMBER_OF_VMETRICS_OFFSET)
    }
}

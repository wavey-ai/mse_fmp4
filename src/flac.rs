use crate::fmp4::Mp4Box;
use crate::{ErrorKind, Result};
use std::io::Write;

#[derive(Debug, Clone)]
pub struct FLACSpecificBox {
    pub metadata_blocks: Vec<FLACMetadataBlock>,
}

impl Mp4Box for FLACSpecificBox {
    const BOX_TYPE: [u8; 4] = *b"dfLa";

    fn box_payload_size(&self) -> Result<u32> {
        Ok(4u32
            + self
                .metadata_blocks
                .iter()
                .map(|block| block.total_size())
                .sum::<u32>())
    }

    fn write_box_payload<W: Write>(&self, mut writer: W) -> Result<()> {
        write_u32!(writer, 0);
        for block in &self.metadata_blocks {
            block.write_to(&mut writer)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct FLACMetadataBlock {
    pub data: Vec<u8>,
}

impl FLACMetadataBlock {
    fn total_size(&self) -> u32 {
        4 + self.data.len() as u32 // 4 bytes for header, plus data
    }

    fn write_to<W: Write>(&self, mut writer: W) -> Result<()> {
        let header: u32 =
            ((true as u32) << 31) | ((0 as u32) << 24) | (self.data.len() as u32 & 0x00FFFFFF);
        write_u32!(writer, header);
        write_all!(writer, &self.data);
        Ok(())
    }
}

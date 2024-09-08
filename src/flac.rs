use crate::fmp4::Mp4Box;
use crate::io::WriteTo;
use crate::Result;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct FlacStreamConfiguration {
    pub min_block_size: u16,
    pub max_block_size: u16,
    pub min_frame_size: u32,
    pub max_frame_size: u32,
    pub sample_rate: u32,
    pub channels: u8,
    pub bits_per_sample: u8,
}

impl Mp4Box for FlacStreamConfiguration {
    const BOX_TYPE: [u8; 4] = *b"dfLa";

    fn box_payload_size(&self) -> Result<u32> {
        Ok(20) // Size of all fields
    }

    fn write_box_payload<W: Write>(&self, mut writer: W) -> Result<()> {
        write_u16!(writer, self.min_block_size);
        write_u16!(writer, self.max_block_size);
        write_u32!(writer, self.min_frame_size);
        write_u32!(writer, self.max_frame_size);
        write_u32!(writer, self.sample_rate);
        write_u8!(writer, self.channels);
        write_u8!(writer, self.bits_per_sample);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct FlacMetadataBlock {
    pub last_metadata_block_flag: bool,
    pub block_type: u8,
    pub length: u32,
    pub data: Vec<u8>,
}

impl Mp4Box for FlacMetadataBlock {
    const BOX_TYPE: [u8; 4] = *b"fLaC";

    fn box_payload_size(&self) -> Result<u32> {
        Ok(5 + self.data.len() as u32) // 1 byte for flags, 4 for length, plus data
    }

    fn write_box_payload<W: Write>(&self, mut writer: W) -> Result<()> {
        let flag_byte = ((self.last_metadata_block_flag as u8) << 7) | self.block_type;
        write_u8!(writer, flag_byte);
        write_u32!(writer, self.length);
        write_all!(writer, &self.data);
        Ok(())
    }
}

extern crate mpeg2ts;
extern crate mse_fmp4;
#[macro_use]
extern crate trackable;

use std::collections::HashMap;
use std::io::Read;
// use mpeg2ts::time::Timestamp;
// use mse_fmp4::fmp4::{self, WriteTo};
use mse_fmp4::avc;
use mse_fmp4::ErrorKind;
use mpeg2ts::pes::{PesPacket, PesPacketReader, ReadPesPacket};
use mpeg2ts::es::{StreamId, StreamType};
use mpeg2ts::ts::{Pid, ReadTsPacket, TsPacket, TsPacketReader, TsPayload};
use trackable::error::ErrorKindExt;

struct AvcStream {
    decoder_configuration_record: Option<avc::AvcDecoderConfigurationRecord>,
    width: Option<usize>,
    height: Option<usize>,
    packets: Vec<PesPacket<Vec<u8>>>,
}

fn read_avc_stream() -> mse_fmp4::Result<AvcStream> {
    let mut stream = AvcStream {
        decoder_configuration_record: None,
        height: None,
        width: None,
        packets: Vec::new(),
    };

    let mut is_first_video = true;
    let reader = MyTsPacketReader {
        inner: TsPacketReader::new(std::io::stdin()),
        pid_to_stream_type: HashMap::new(),
        stream_id_to_pid: HashMap::new(),
    };
    let mut reader = PesPacketReader::new(reader);
    while let Some(pes) = track!(
        reader
            .read_pes_packet()
            .map_err(|e| ErrorKind::Other.takes_over(e))
    )? {
        if !pes.header.stream_id.is_video() {
            continue;
        }

        if is_first_video {
            is_first_video = false;
            let stream_type = track_assert_some!(
                reader
                    .ts_packet_reader()
                    .get_stream_type(pes.header.stream_id),
                ErrorKind::InvalidInput
            );
            track_assert_eq!(stream_type, StreamType::H264, ErrorKind::Unsupported);

            let mut sps = None;
            let mut pps = None;
            let mut sps_info = None;
            let nal_units = track!(avc::ByteStreamFormatNalUnits::new(&pes.data))?;
            for nal_unit in nal_units {
                let nal = track!(avc::NalUnit::read_from(nal_unit))?;
                match nal.nal_unit_type {
                    avc::NalUnitType::SequenceParameterSet => {
                        sps_info = Some(track!(avc::SequenceParameterSet::read_from(
                            &nal_unit[1..]
                        ))?);
                        sps = Some((&nal_unit[1..]).to_owned());
                    }
                    avc::NalUnitType::PictureParameterSet => {
                        pps = Some((&nal_unit[1..]).to_owned());
                    }
                    _ => {}
                }
            }

            let sps_info = track_assert_some!(sps_info, ErrorKind::InvalidInput);
            let sps = track_assert_some!(sps, ErrorKind::InvalidInput);
            let pps = track_assert_some!(pps, ErrorKind::InvalidInput);
            stream.decoder_configuration_record = Some(avc::AvcDecoderConfigurationRecord {
                profile_idc: sps_info.profile_idc,
                constraint_set_flag: sps_info.constraint_set_flag,
                level_idc: sps_info.level_idc,
                sequence_parameter_set: sps,
                picture_parameter_set: pps,
            });
            stream.width = Some(sps_info.width());
            stream.height = Some(sps_info.height());
        }
        stream.packets.push(pes);
    }

    Ok(stream)
}

fn main() {
    // let mut f = fmp4::File::new();
    // f.moov_box.mvhd_box.timescale = Timestamp::RESOLUTION as u32;
    // f.moov_box.mvhd_box.duration = 1 * Timestamp::RESOLUTION as u32; // TODO

    // let mut t = fmp4::TrackBox::new(true);
    // t.tkhd_box.duration = 1 * Timestamp::RESOLUTION as u32; // TODO
    // t.mdia_box.mdhd_box.timescale = Timestamp::RESOLUTION as u32;
    // t.mdia_box.mdhd_box.duration = 1 * Timestamp::RESOLUTION as u32; // TODO

    // // TODO: t.mdia_box.minf_box.stbl_box.stsd_box.sample_entries.push(...);
    // f.moov_box.trak_boxes.push(t);

    // f.moov_box.mvex_box.mehd_box.fragment_duration = 1 * Timestamp::RESOLUTION as u32; // TODO
    // f.moov_box
    //     .mvex_box
    //     .trex_boxes
    //     .push(fmp4::TrackExtendsBox::new(1));
    // track_try_unwrap!(f.write_to(std::io::stdout()));

    let avc_stream = track_try_unwrap!(read_avc_stream());
    println!("# {:?}", avc_stream.decoder_configuration_record);
    println!("# {:?}, {:?}", avc_stream.width, avc_stream.height);
}

struct MyTsPacketReader<R> {
    inner: TsPacketReader<R>,
    pid_to_stream_type: HashMap<Pid, StreamType>,
    stream_id_to_pid: HashMap<StreamId, Pid>,
}
impl<R> MyTsPacketReader<R> {
    fn get_stream_type(&self, stream_id: StreamId) -> Option<StreamType> {
        self.stream_id_to_pid
            .get(&stream_id)
            .and_then(|pid| self.pid_to_stream_type.get(pid))
            .cloned()
    }
}
impl<R: Read> ReadTsPacket for MyTsPacketReader<R> {
    fn read_ts_packet(&mut self) -> mpeg2ts::Result<Option<TsPacket>> {
        if let Some(packet) = track!(self.inner.read_ts_packet())? {
            match packet.payload {
                Some(TsPayload::Pmt(ref pmt)) => for es_info in &pmt.table {
                    self.pid_to_stream_type
                        .insert(es_info.elementary_pid, es_info.stream_type);
                },
                Some(TsPayload::Pes(ref pes)) => {
                    if self.pid_to_stream_type.contains_key(&packet.header.pid) {
                        self.stream_id_to_pid
                            .insert(pes.header.stream_id, packet.header.pid);
                    }
                }
                _ => {}
            }
            Ok(Some(packet))
        } else {
            Ok(None)
        }
    }
}

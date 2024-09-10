//! Fragmented MP4 (ISO BMFF) related constituent elements.
pub use self::common::Mp4Box;
pub use self::initialization::{
    AacSampleEntry, AvcConfigurationBox, AvcSampleEntry, ChunkOffsetBox, DataEntryUrlBox,
    DataInformationBox, DataReferenceBox, EditBox, EditListBox, FLACSampleEntry, FileTypeBox,
    HandlerReferenceBox, InitializationSegment, MediaBox, MediaHeaderBox, MediaInformationBox,
    MovieBox, MovieExtendsBox, MovieExtendsHeaderBox, MovieHeaderBox, Mpeg4EsDescriptorBox,
    SampleDescriptionBox, SampleEntry, SampleSizeBox, SampleTableBox, SampleToChunkBox,
    SoundMediaHeaderBox, TimeToSampleBox, TrackBox, TrackExtendsBox, TrackHeaderBox,
    VideoMediaHeaderBox,
};
pub use self::media::{
    MediaDataBox, MediaSegment, MovieFragmentBox, MovieFragmentHeaderBox, Sample, SampleFlags,
    TrackFragmentBaseMediaDecodeTimeBox, TrackFragmentBox, TrackFragmentHeaderBox, TrackRunBox,
    TrackType,
};

const VIDEO_TRACK_ID: u32 = 1;
const AUDIO_TRACK_ID: u32 = 2;

mod common;
mod initialization;
mod media;

///! TODO

/// TODO
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub enum IoType {
    /// TODO
    Invalid,
    /// TODO
    Read,
    /// TODO
    Write,
    /// TODO
    Unmap,
    /// TODO
    Flush,
    /// TODO
    Reset,
    /// TODO
    NvmeAdmin,
    /// TODO
    NvmeIo,
    /// TODO
    NvmeIoMd,
    /// TODO
    WriteZeros,
    /// TODO
    ZeroCopy,
    /// TODO
    ZoneInfo,
    /// TODO
    ZoneManagement,
    /// TODO
    ZoneAppend,
    /// TODO
    Compare,
    /// TODO
    CompareAndWrite,
    /// TODO
    Abort,
    /// TODO
    SeekHole,
    /// TODO
    SeekData,
    /// TODO
    Copy,
    /// TODO
    IoNumTypes,
}

/// TODO
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
#[non_exhaustive]
pub enum IoStatus {
    /// TODO
    Aborted,
    /// TODO
    FirstFusedFailed,
    /// TODO
    MisCompared,
    /// TODO
    NoMemory,
    /// TODO
    ScsiError,
    /// TODO
    NvmeError,
    /// TODO
    Failed,
    /// TODO
    Pending,
    /// TODO
    Success,
}

impl From<IoType> for u32 {
    fn from(t: IoType) -> Self {
        match t {
            IoType::Invalid => 0,
            IoType::Read => 1,
            IoType::Write => 2,
            IoType::Unmap => 3,
            IoType::Flush => 4,
            IoType::Reset => 5,
            IoType::NvmeAdmin => 6,
            IoType::NvmeIo => 7,
            IoType::NvmeIoMd => 8,
            IoType::WriteZeros => 9,
            IoType::ZeroCopy => 10,
            IoType::ZoneInfo => 11,
            IoType::ZoneManagement => 12,
            IoType::ZoneAppend => 13,
            IoType::Compare => 14,
            IoType::CompareAndWrite => 15,
            IoType::Abort => 16,
            IoType::SeekHole => 17,
            IoType::SeekData => 18,
            IoType::Copy => 19,
            IoType::IoNumTypes => 20,
        }
    }
}

impl From<u32> for IoType {
    fn from(u: u32) -> Self {
        match u {
            0 => Self::Invalid,
            1 => Self::Read,
            2 => Self::Write,
            3 => Self::Unmap,
            4 => Self::Flush,
            5 => Self::Reset,
            6 => Self::NvmeAdmin,
            7 => Self::NvmeIo,
            8 => Self::NvmeIoMd,
            9 => Self::WriteZeros,
            10 => Self::ZeroCopy,
            11 => Self::ZoneInfo,
            12 => Self::ZoneManagement,
            13 => Self::ZoneAppend,
            14 => Self::Compare,
            15 => Self::CompareAndWrite,
            16 => Self::Abort,
            17 => Self::SeekHole,
            18 => Self::SeekData,
            19 => Self::Copy,
            20 => Self::IoNumTypes,
            _ => panic!("invalid IO type"),
        }
    }
}

impl From<i32> for IoStatus {
    fn from(status: i32) -> Self {
        match status {
            -7 => Self::Aborted,
            -6 => Self::FirstFusedFailed,
            -5 => Self::MisCompared,
            -4 => Self::NoMemory,
            -3 => Self::ScsiError,
            -2 => Self::NvmeError,
            -1 => Self::Failed,
            0 => Self::Pending,
            1 => Self::Success,
            _ => panic!("invalid status code"),
        }
    }
}

impl From<IoStatus> for i32 {
    fn from(i: IoStatus) -> Self {
        match i {
            IoStatus::Aborted => -7,
            IoStatus::FirstFusedFailed => -6,
            IoStatus::MisCompared => -5,
            IoStatus::NoMemory => -4,
            IoStatus::ScsiError => -3,
            IoStatus::NvmeError => -2,
            IoStatus::Failed => -1,
            IoStatus::Pending => 0,
            IoStatus::Success => 1,
        }
    }
}

impl From<i8> for IoStatus {
    fn from(status: i8) -> Self {
        (status as i32).into()
    }
}

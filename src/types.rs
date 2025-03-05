use crate::codec::{TryReadFromBytes, TryWriteToBytes};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Leap(u8);

pub const NTP_LEAP_NO_WARNING: Leap = Leap(0);
pub const NTP_LEAP_LAST_MINUTE_HAS_61_SECONDS: Leap = Leap(1);
pub const NTP_LEAP_LAST_MINUTE_HAS_59_SECONDS: Leap = Leap(2);
pub const NTP_LEAP_UNKNOWN: Leap = Leap(3);

impl TryFrom<u8> for Leap {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 3 {
            return Err("Value out of range for leap");
        }

        Ok(Self(value))
    }
}

impl From<Leap> for u8 {
    fn from(value: Leap) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version(u8);

pub const NTP_VERSION_4: Version = Version(4);

impl TryFrom<u8> for Version {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 7 {
            return Err("Value out of range for version");
        }

        Ok(Self(value))
    }
}

impl From<Version> for u8 {
    fn from(value: Version) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mode(u8);
impl TryFrom<u8> for Mode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 7 {
            return Err("Value out of range for mode");
        }

        Ok(Self(value))
    }
}

impl From<Mode> for u8 {
    fn from(value: Mode) -> Self {
        value.0
    }
}

pub const NTP_MODE_RESERVED: Mode = Mode(0);
pub const NTP_MODE_SYMMETRIC_ACTIVE: Mode = Mode(1);
pub const NTP_MODE_SYMMETRIC_PASSIVE: Mode = Mode(2);
pub const NTP_MODE_CLIENT: Mode = Mode(3);
pub const NTP_MODE_SERVER: Mode = Mode(4);
pub const NTP_MODE_BROADCAST: Mode = Mode(5);
pub const NTP_MODE_CONTROL_MESSAGE: Mode = Mode(6);
pub const NTP_MODE_RESERVED_FOR_PRIVATE_USE: Mode = Mode(7);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stratum(u8);

impl From<u8> for Stratum {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<Stratum> for u8 {
    fn from(value: Stratum) -> Self {
        value.0
    }
}

impl TryWriteToBytes for Stratum {
    type Error = &'static str;

    fn try_write_to_bytes(&self, bytes: &mut [u8]) -> Result<usize, Self::Error> {
        self.0.try_write_to_bytes(bytes)
    }
}

impl<'a> TryReadFromBytes<'a> for Stratum {
    type Error = &'static str;

    fn try_read_from_bytes(bytes: &'a [u8]) -> Result<(Self, usize), Self::Error> {
        let (value, size) = u8::try_read_from_bytes(bytes)?;
        Ok((Self(value), size))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Poll(i8);

impl From<i8> for Poll {
    fn from(value: i8) -> Self {
        Self(value)
    }
}

impl From<Poll> for i8 {
    fn from(value: Poll) -> Self {
        value.0
    }
}

impl TryWriteToBytes for Poll {
    type Error = &'static str;

    fn try_write_to_bytes(&self, bytes: &mut [u8]) -> Result<usize, Self::Error> {
        self.0.try_write_to_bytes(bytes)
    }
}

impl<'a> TryReadFromBytes<'a> for Poll {
    type Error = &'static str;

    fn try_read_from_bytes(bytes: &'a [u8]) -> Result<(Self, usize), Self::Error> {
        let (value, size) = i8::try_read_from_bytes(bytes)?;
        Ok((Self(value), size))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Precision(i8);

impl From<i8> for Precision {
    fn from(value: i8) -> Self {
        Self(value)
    }
}

impl From<Precision> for i8 {
    fn from(value: Precision) -> Self {
        value.0
    }
}

impl TryWriteToBytes for Precision {
    type Error = &'static str;

    fn try_write_to_bytes(&self, bytes: &mut [u8]) -> Result<usize, Self::Error> {
        self.0.try_write_to_bytes(bytes)
    }
}

impl<'a> TryReadFromBytes<'a> for Precision {
    type Error = &'static str;

    fn try_read_from_bytes(bytes: &'a [u8]) -> Result<(Self, usize), Self::Error> {
        let (value, size) = i8::try_read_from_bytes(bytes)?;
        Ok((Self(value), size))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RefId([u8; 4]);

impl TryWriteToBytes for RefId {
    type Error = &'static str;

    fn try_write_to_bytes(&self, bytes: &mut [u8]) -> Result<usize, Self::Error> {
        self.0.try_write_to_bytes(bytes)
    }
}

impl<'a> TryReadFromBytes<'a> for RefId {
    type Error = &'static str;

    fn try_read_from_bytes(bytes: &'a [u8]) -> Result<(Self, usize), Self::Error> {
        let (value, size) = <[u8; 4]>::try_read_from_bytes(bytes)?;
        Ok((Self(value), size))
    }
}

impl From<[u8; 4]> for RefId {
    fn from(value: [u8; 4]) -> Self {
        Self(value)
    }
}

impl From<RefId> for [u8; 4] {
    fn from(value: RefId) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Digest([u8; 16]);

impl From<[u8; 16]> for Digest {
    fn from(value: [u8; 16]) -> Self {
        Self(value)
    }
}

impl From<Digest> for [u8; 16] {
    fn from(value: Digest) -> Self {
        value.0
    }
}

impl TryWriteToBytes for Digest {
    type Error = &'static str;

    fn try_write_to_bytes(&self, bytes: &mut [u8]) -> Result<usize, Self::Error> {
        self.0.try_write_to_bytes(bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NtpShort(u32);

impl TryWriteToBytes for NtpShort {
    type Error = &'static str;

    fn try_write_to_bytes(&self, bytes: &mut [u8]) -> Result<usize, Self::Error> {
        self.0.try_write_to_bytes(bytes)
    }
}

impl<'a> TryReadFromBytes<'a> for NtpShort {
    type Error = &'static str;

    fn try_read_from_bytes(bytes: &'a [u8]) -> Result<(Self, usize), Self::Error> {
        let (value, size) = u32::try_read_from_bytes(bytes)?;
        Ok((Self(value), size))
    }
}

impl NtpShort {
    pub fn new(seconds: u16, fraction: u16) -> Self {
        Self(((seconds as u32) << 16) | (fraction as u32))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NtpTimestamp(u64);

impl NtpTimestamp {
    pub fn new(seconds: u32, fraction: u32) -> Self {
        Self(((seconds as u64) << 32) | (fraction as u64))
    }
}

impl TryWriteToBytes for NtpTimestamp {
    type Error = &'static str;

    fn try_write_to_bytes(&self, bytes: &mut [u8]) -> Result<usize, Self::Error> {
        self.0.try_write_to_bytes(bytes)
    }
}

impl<'a> TryReadFromBytes<'a> for NtpTimestamp {
    type Error = &'static str;

    fn try_read_from_bytes(bytes: &'a [u8]) -> Result<(Self, usize), Self::Error> {
        let (value, size) = u64::try_read_from_bytes(bytes)?;
        Ok((Self(value), size))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NtpDate {
    era_number: u32,
    era_offset: u32,
    fraction: u64,
}

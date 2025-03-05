use crate::{
    codec::{TryReadFromBytes, TryWriteToBytes},
    types::{Leap, Mode, NtpShort, NtpTimestamp, Poll, Precision, RefId, Stratum, Version},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NtpPacketHeader {
    pub leap_indicator: Leap,
    pub version_number: Version,
    pub mode: Mode,
    pub stratum: Stratum,
    pub poll: Poll,
    pub precision: Precision,
    pub rootdelay: NtpShort,
    pub rootdisp: NtpShort,
    pub refid: RefId,
    pub reftime: NtpTimestamp,
    pub org: NtpTimestamp,
    pub rec: NtpTimestamp,
    pub xmt: NtpTimestamp,
}

impl TryWriteToBytes for NtpPacketHeader {
    type Error = &'static str;

    fn try_write_to_bytes(&self, bytes: &mut [u8]) -> Result<usize, Self::Error> {
        let mut total_bytes = 0;
        if bytes.is_empty() {
            return Err("Not enough space in buffer");
        }
        bytes[0] = (u8::from(self.leap_indicator) << 6)
            | (u8::from(self.version_number) << 3)
            | u8::from(self.mode);

        total_bytes += 1;
        total_bytes += self.stratum.try_write_to_bytes(&mut bytes[total_bytes..])?;
        total_bytes += self.poll.try_write_to_bytes(&mut bytes[total_bytes..])?;
        total_bytes += self
            .precision
            .try_write_to_bytes(&mut bytes[total_bytes..])?;
        total_bytes += self
            .rootdelay
            .try_write_to_bytes(&mut bytes[total_bytes..])?;
        total_bytes += self
            .rootdisp
            .try_write_to_bytes(&mut bytes[total_bytes..])?;
        total_bytes += self.refid.try_write_to_bytes(&mut bytes[total_bytes..])?;
        total_bytes += self.reftime.try_write_to_bytes(&mut bytes[total_bytes..])?;
        total_bytes += self.org.try_write_to_bytes(&mut bytes[total_bytes..])?;
        total_bytes += self.rec.try_write_to_bytes(&mut bytes[total_bytes..])?;
        total_bytes += self.xmt.try_write_to_bytes(&mut bytes[total_bytes..])?;

        Ok(total_bytes)
    }
}

impl<'a> TryReadFromBytes<'a> for NtpPacketHeader {
    type Error = &'static str;

    fn try_read_from_bytes(bytes: &'a [u8]) -> Result<(Self, usize), Self::Error> {
        let mut total_bytes = 0;

        if bytes.is_empty() {
            return Err("Not enough space in buffer");
        }
        let leap_indicator = Leap::try_from((bytes[0] & 0b11_000_000) >> 6)?;
        let version_number = Version::try_from((bytes[0] & 0b00_111_000) >> 3)?;
        let mode = Mode::try_from(bytes[0] & 0b00_000_111)?;
        total_bytes += 1;
        let (stratum, size) = Stratum::try_read_from_bytes(&bytes[total_bytes..])?;
        total_bytes += size;

        let (poll, size) = Poll::try_read_from_bytes(&bytes[total_bytes..])?;
        total_bytes += size;

        let (precision, size) = Precision::try_read_from_bytes(&bytes[total_bytes..])?;
        total_bytes += size;

        let (rootdelay, size) = NtpShort::try_read_from_bytes(&bytes[total_bytes..])?;
        total_bytes += size;

        let (rootdisp, size) = NtpShort::try_read_from_bytes(&bytes[total_bytes..])?;
        total_bytes += size;

        let (refid, size) = RefId::try_read_from_bytes(&bytes[total_bytes..])?;
        total_bytes += size;

        let (reftime, size) = NtpTimestamp::try_read_from_bytes(&bytes[total_bytes..])?;
        total_bytes += size;

        let (org, size) = NtpTimestamp::try_read_from_bytes(&bytes[total_bytes..])?;
        total_bytes += size;

        let (rec, size) = NtpTimestamp::try_read_from_bytes(&bytes[total_bytes..])?;
        total_bytes += size;

        let (xmt, size) = NtpTimestamp::try_read_from_bytes(&bytes[total_bytes..])?;
        total_bytes += size;

        Ok((
            Self {
                leap_indicator,
                version_number,
                mode,
                stratum,
                poll,
                precision,
                rootdelay,
                rootdisp,
                refid,
                reftime,
                org,
                rec,
                xmt,
            },
            total_bytes,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{NTP_LEAP_NO_WARNING, NTP_MODE_CLIENT, NTP_VERSION_4};

    use super::*;

    #[test]
    fn write_packet_header_zeros_to_bytes() {
        let packet = NtpPacketHeader {
            leap_indicator: NTP_LEAP_NO_WARNING,
            version_number: NTP_VERSION_4,
            mode: NTP_MODE_CLIENT,
            stratum: Stratum::from(0),
            poll: Poll::from(0),
            precision: Precision::from(0),
            rootdelay: NtpShort::new(0, 0),
            rootdisp: NtpShort::new(0, 0),
            refid: RefId::from([0, 0, 0, 0]),
            reftime: NtpTimestamp::new(0, 0),
            org: NtpTimestamp::new(0, 0),
            rec: NtpTimestamp::new(0, 0),
            xmt: NtpTimestamp::new(0, 0),
        };

        let mut buffer = [0u8; 1024];
        let serialized_size = packet.try_write_to_bytes(&mut buffer).unwrap();
        #[rustfmt::skip]
        let expected_bytes = [
            0x23, // mode (3 bits), version (3 bits), leap (2 bits)
            0,            // stratum
            0,            // poll
            0,          // precision (-18 as i8)
            0, 0,0, 0,    // rootdelay
            0, 0, 0, 0, // rootdisp
            0, 0, 0, 0, // refid
            0, 0, 0, 0, 0, 0, 0, 0, // reftime
            0, 0, 0, 0, 0, 0, 0, 0, // org
            0, 0, 0, 0, 0, 0, 0, 0, // rec
            0, 0, 0, 0, 0, 0, 0, 0, // xmt
        ];

        assert_eq!(&buffer[..serialized_size], &expected_bytes);
    }

    #[test]
    fn write_packet_header_different_information_to_bytes() {
        let packet = NtpPacketHeader {
            leap_indicator: NTP_LEAP_NO_WARNING,
            version_number: NTP_VERSION_4,
            mode: NTP_MODE_CLIENT,
            stratum: Stratum::from(1),
            poll: Poll::from(6),
            precision: Precision::from(-18),
            rootdelay: NtpShort::new(1, 0),
            rootdisp: NtpShort::new(0, 100),
            refid: RefId::from([1, 2, 3, 4]),
            reftime: NtpTimestamp::new(100, 500),
            org: NtpTimestamp::new(200, 200),
            rec: NtpTimestamp::new(50, 100),
            xmt: NtpTimestamp::new(10, 1000),
        };

        let mut buffer = [0u8; 1024];
        let serialized_size = packet.try_write_to_bytes(&mut buffer).unwrap();
        #[rustfmt::skip]
        let expected_bytes = [
            0b00_100_011, // mode (3 bits), version (3 bits), leap (2 bits)
            1,            // stratum
            6,            // poll
            238,          // precision (-18 as i8)
            0, 1,0, 0,    // rootdelay
            0, 0, 0, 100, // rootdisp
            1, 2, 3, 4, // refid
            0, 0, 0, 100, 0, 0, 1, 244, // reftime
            0, 0, 0, 200, 0, 0, 0, 200, // org
            0, 0, 0, 50, 0, 0, 0, 100, // rec
            0, 0, 0, 10, 0, 0, 3, 232, // xmt
        ];

        assert_eq!(&buffer[..serialized_size], &expected_bytes);
    }

    #[test]
    fn read_packet_header_zeros_from_bytes() {
        #[rustfmt::skip]
        let bytes = [
            0x23, // mode (3 bits), version (3 bits), leap (2 bits)
            0,            // stratum
            0,            // poll
            0,          // precision (-18 as i8)
            0, 0,0, 0,    // rootdelay
            0, 0, 0, 0, // rootdisp
            0, 0, 0, 0, // refid
            0, 0, 0, 0, 0, 0, 0, 0, // reftime
            0, 0, 0, 0, 0, 0, 0, 0, // org
            0, 0, 0, 0, 0, 0, 0, 0, // rec
            0, 0, 0, 0, 0, 0, 0, 0, // xmt
        ];

        let (packet, _) = NtpPacketHeader::try_read_from_bytes(&bytes).unwrap();

        let expected = NtpPacketHeader {
            leap_indicator: NTP_LEAP_NO_WARNING,
            version_number: NTP_VERSION_4,
            mode: NTP_MODE_CLIENT,
            stratum: Stratum::from(0),
            poll: Poll::from(0),
            precision: Precision::from(0),
            rootdelay: NtpShort::new(0, 0),
            rootdisp: NtpShort::new(0, 0),
            refid: RefId::from([0, 0, 0, 0]),
            reftime: NtpTimestamp::new(0, 0),
            org: NtpTimestamp::new(0, 0),
            rec: NtpTimestamp::new(0, 0),
            xmt: NtpTimestamp::new(0, 0),
        };

        assert_eq!(packet, expected);
    }

    #[test]
    fn read_packet_header_different_information_from_bytes() {
        #[rustfmt::skip]
        let bytes = [
            0b00_100_011, // mode (3 bits), version (3 bits), leap (2 bits)
            1,            // stratum
            6,            // poll
            238,          // precision (-18 as i8)
            0, 1,0, 0,    // rootdelay
            0, 0, 0, 100, // rootdisp
            1, 2, 3, 4, // refid
            0, 0, 0, 100, 0, 0, 1, 244, // reftime
            0, 0, 0, 200, 0, 0, 0, 200, // org
            0, 0, 0, 50, 0, 0, 0, 100, // rec
            0, 0, 0, 10, 0, 0, 3, 232, // xmt
        ];

        let (packet, _) = NtpPacketHeader::try_read_from_bytes(&bytes).unwrap();

        let expected = NtpPacketHeader {
            leap_indicator: NTP_LEAP_NO_WARNING,
            version_number: NTP_VERSION_4,
            mode: NTP_MODE_CLIENT,
            stratum: Stratum::from(1),
            poll: Poll::from(6),
            precision: Precision::from(-18),
            rootdelay: NtpShort::new(1, 0),
            rootdisp: NtpShort::new(0, 100),
            refid: RefId::from([1, 2, 3, 4]),
            reftime: NtpTimestamp::new(100, 500),
            org: NtpTimestamp::new(200, 200),
            rec: NtpTimestamp::new(50, 100),
            xmt: NtpTimestamp::new(10, 1000),
        };

        assert_eq!(packet, expected);
    }
}

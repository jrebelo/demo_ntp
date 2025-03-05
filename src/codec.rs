/// Trait for types that can be serialized to bytes
pub trait TryWriteToBytes {
    type Error;
    /// Attempts to write the implementing type to the provided byte buffer
    ///
    /// # Arguments
    /// * `bytes` - The byte slice to write to
    ///
    /// # Returns
    /// The number of bytes written if successful
    ///
    /// # Errors
    /// Returns an error if the bytes cannot be written
    fn try_write_to_bytes(&self, bytes: &mut [u8]) -> Result<usize, Self::Error>;
}

/// Trait for types that can be deserialized from bytes
pub trait TryReadFromBytes<'a>: Sized {
    type Error;

    /// Attempts to read and construct the implementing type from a byte slice
    ///
    /// # Arguments
    /// * `bytes` - The byte slice to read from
    ///
    /// # Returns
    /// A tuple containing:
    /// - The constructed type if successful
    /// - The number of bytes read
    ///
    /// # Errors
    /// Returns an error if the bytes cannot be parsed into the type
    fn try_read_from_bytes(bytes: &'a [u8]) -> Result<(Self, usize), Self::Error>;
}

impl TryWriteToBytes for u8 {
    type Error = &'static str;

    fn try_write_to_bytes(&self, bytes: &mut [u8]) -> Result<usize, Self::Error> {
        #[allow(clippy::len_zero)]
        if bytes.len() < 1 {
            return Err("Buffer too small");
        }

        bytes[0] = *self;
        Ok(1)
    }
}

impl<'a> TryReadFromBytes<'a> for u8 {
    type Error = &'static str;

    fn try_read_from_bytes(bytes: &'a [u8]) -> Result<(Self, usize), Self::Error> {
        #[allow(clippy::len_zero)]
        if bytes.len() < 1 {
            return Err("Buffer too small");
        }
        Ok((bytes[0], 1))
    }
}

impl TryWriteToBytes for i8 {
    type Error = &'static str;

    fn try_write_to_bytes(&self, bytes: &mut [u8]) -> Result<usize, Self::Error> {
        #[allow(clippy::len_zero)]
        if bytes.len() < 1 {
            return Err("Buffer too small");
        }

        // https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast
        // Casting between two integers of the same size (e.g. i32 -> u32) is a no-op
        // (Rust uses 2’s complement for negative values of fixed integers)
        bytes[0] = *self as u8;
        Ok(1)
    }
}

impl<'a> TryReadFromBytes<'a> for i8 {
    type Error = &'static str;

    fn try_read_from_bytes(bytes: &'a [u8]) -> Result<(Self, usize), Self::Error> {
        #[allow(clippy::len_zero)]
        if bytes.len() < 1 {
            return Err("Buffer too small");
        }
        Ok((bytes[0] as i8, 1))
    }
}

impl TryWriteToBytes for u16 {
    type Error = &'static str;

    fn try_write_to_bytes(&self, bytes: &mut [u8]) -> Result<usize, Self::Error> {
        if bytes.len() < 2 {
            return Err("Buffer too small");
        }
        let value = self.to_be_bytes();
        bytes[0] = value[0];
        bytes[1] = value[1];
        Ok(2)
    }
}

impl<'a> TryReadFromBytes<'a> for u16 {
    type Error = &'static str;

    fn try_read_from_bytes(bytes: &'a [u8]) -> Result<(Self, usize), Self::Error> {
        if bytes.len() < 2 {
            return Err("Buffer too small");
        }
        let value = u16::from_be_bytes([bytes[0], bytes[1]]);
        Ok((value, 2))
    }
}

impl TryWriteToBytes for u32 {
    type Error = &'static str;

    fn try_write_to_bytes(&self, bytes: &mut [u8]) -> Result<usize, Self::Error> {
        if bytes.len() < 4 {
            return Err("Buffer too small");
        }
        let value = self.to_be_bytes();
        bytes[0] = value[0];
        bytes[1] = value[1];
        bytes[2] = value[2];
        bytes[3] = value[3];
        Ok(4)
    }
}

impl<'a> TryReadFromBytes<'a> for u32 {
    type Error = &'static str;

    fn try_read_from_bytes(bytes: &'a [u8]) -> Result<(Self, usize), Self::Error> {
        if bytes.len() < 4 {
            return Err("Buffer too small");
        }
        let value = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        Ok((value, 4))
    }
}

impl TryWriteToBytes for i32 {
    type Error = &'static str;

    fn try_write_to_bytes(&self, bytes: &mut [u8]) -> Result<usize, Self::Error> {
        if bytes.len() < 4 {
            return Err("Buffer too small");
        }
        let value = self.to_be_bytes();
        bytes[0] = value[0];
        bytes[1] = value[1];
        bytes[2] = value[2];
        bytes[3] = value[3];
        Ok(4)
    }
}

impl<'a> TryReadFromBytes<'a> for i32 {
    type Error = &'static str;

    fn try_read_from_bytes(bytes: &'a [u8]) -> Result<(Self, usize), Self::Error> {
        if bytes.len() < 4 {
            return Err("Buffer too small");
        }
        let value = i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        Ok((value, 4))
    }
}

impl TryWriteToBytes for u64 {
    type Error = &'static str;

    fn try_write_to_bytes(&self, bytes: &mut [u8]) -> Result<usize, Self::Error> {
        if bytes.len() < 4 {
            return Err("Buffer too small");
        }
        let value = self.to_be_bytes();
        bytes[0] = value[0];
        bytes[1] = value[1];
        bytes[2] = value[2];
        bytes[3] = value[3];
        bytes[4] = value[4];
        bytes[5] = value[5];
        bytes[6] = value[6];
        bytes[7] = value[7];
        Ok(8)
    }
}

impl<'a> TryReadFromBytes<'a> for u64 {
    type Error = &'static str;

    fn try_read_from_bytes(bytes: &'a [u8]) -> Result<(Self, usize), Self::Error> {
        if bytes.len() < 8 {
            return Err("Buffer too small");
        }
        let value = u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]);
        Ok((value, 8))
    }
}

impl<const N: usize> TryWriteToBytes for [u8; N] {
    type Error = &'static str;

    fn try_write_to_bytes(&self, bytes: &mut [u8]) -> Result<usize, Self::Error> {
        if bytes.len() < N {
            return Err("Buffer too small");
        }
        bytes[0..N].copy_from_slice(self);
        Ok(N)
    }
}

impl<'a, const N: usize> TryReadFromBytes<'a> for [u8; N] {
    type Error = &'static str;

    fn try_read_from_bytes(bytes: &'a [u8]) -> Result<(Self, usize), Self::Error> {
        if bytes.len() < N {
            return Err("Buffer too small");
        }
        let mut array = [0u8; N];
        array.copy_from_slice(&bytes[0..N]);
        Ok((array, N))
    }
}

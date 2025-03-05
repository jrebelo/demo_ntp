pub struct NtpClientBuilder {}

impl Default for NtpClientBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl NtpClientBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build() -> NtpClient {
        NtpClient {}
    }
}

pub struct NtpClient {}

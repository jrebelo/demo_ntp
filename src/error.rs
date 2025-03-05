pub type NtpResult<T> = Result<T, NtpError>;

#[derive(Debug, PartialEq, Eq)]
pub enum NtpError {}

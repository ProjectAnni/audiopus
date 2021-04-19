use crate::ffi;
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Error {
    /// A value failed to match a documented [`Application`].
    ///
    /// [`Application`]: ../enum.Application.html
    InvalidApplication,
    /// A value failed to match a documented [`Bandwidth`].
    ///
    /// [`Bandwidth`]: ../enum.Application.html
    InvalidBandwidth(i32),
    /// A value failed to match a documented [`Bitrate`],
    /// negative values are invalid.
    ///
    /// [`Bitrate`]: ../enum.Bitrate.html
    InvalidBitrate(i32),
    /// A value failed to match a documented [`Signal`].
    ///
    /// [`Signal`]: ../enum.Signal.html
    InvalidSignal(i32),
    /// Complexity was lower than 1 or higher than 10.
    InvalidComplexity(i32),
    /// A value failed to match a documented [`SampleRate`].
    ///
    /// [`SampleRate`]: ../enum.SampleRate.html
    InvalidSampleRate(i32),
    /// A value failed to match a documented [`Channels`].
    ///
    /// [`Channels`]: ../enum.Channels.html
    InvalidChannels(i32),
    /// An error returned from Opus containing an [`ErrorCode`] describing
    /// the cause.
    Opus(ErrorCode),
    /// Opus is not operating empty packets.
    EmptyPacket,
    /// Opus' maximum `Vec` or slice length of `std::i32::MAX` has been
    /// exceeded.
    SignalsTooLarge,
    /// Opus' maximum `Vec` or slice length of `std::i32::MAX` has been
    /// exceeded.
    PacketTooLarge,
    /// A `Vec` representing a mapping exceeded the expected value.
    MappingExpectedLen(usize),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Opus(err) => Some(err),
            _ => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::InvalidApplication => f.write_str("Invalid Application"),
            Error::InvalidBandwidth(bandwidth) => write!(f, "Invalid Bandwitdh: {}", bandwidth),
            Error::InvalidSignal(signal) => write!(f, "Invalid Signal: {}", signal),
            Error::InvalidComplexity(complexity) => write!(f, "Invalid Complexity: {}", complexity),
            Error::InvalidSampleRate(rate) => write!(f, "Invalid Sample Rate: {}", rate),
            Error::InvalidChannels(channels) => write!(f, "Invalid Channels: {}", channels),
            Error::Opus(error_code) => write!(f, "{}", error_code),
            Error::EmptyPacket => f.write_str("Passed packet contained no elements"),
            Error::SignalsTooLarge => f.write_str("Signals' length exceeded `i32::MAX`"),
            Error::PacketTooLarge => f.write_str("Packet's length exceeded `i32::MAX`"),
            Error::InvalidBitrate(rate) => write!(f, "Invalid Bitrate: {}", rate),
            Error::MappingExpectedLen(len) => write!(f, "Wrong channel length, expected: {}", len),
        }
    }
}

impl From<ErrorCode> for Error {
    fn from(error_code: ErrorCode) -> Error {
        Error::Opus(error_code)
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ErrorCode {
    BadArgument = ffi::OPUS_BAD_ARG,
    BufferTooSmall = ffi::OPUS_BUFFER_TOO_SMALL,
    InternalError = ffi::OPUS_INTERNAL_ERROR,
    InvalidPacket = ffi::OPUS_INVALID_PACKET,
    Unimplemented = ffi::OPUS_UNIMPLEMENTED,
    InvalidState = ffi::OPUS_INVALID_STATE,
    AllocFail = ffi::OPUS_ALLOC_FAIL,
    /// Occurs when Opus sends an error value that is not documented.
    /// `0` is unrelated to Opus and just a mere marker by this crate to
    /// differentiate between Opus' errors (all of them are negative).
    Unknown = 0,
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let s = match self {
            ErrorCode::BadArgument => "Passed argument violated Opus' specified requirements",
            ErrorCode::BufferTooSmall => "Passed buffer was too small",
            ErrorCode::InternalError => "Internal error inside Opus occured",
            ErrorCode::InvalidPacket => "Opus received a packet violating requirements",
            ErrorCode::Unimplemented => "Unimplemented code branch was attempted to be executed",
            ErrorCode::InvalidState => "Opus-type instance is in an invalid state",
            ErrorCode::AllocFail => "Opus was unable to allocate memory",
            ErrorCode::Unknown => {
                "Opus returned a non-negative error, this might be a Audiopus or Opus bug"
            }
        };

        write!(f, "{}", s)
    }
}

impl StdError for ErrorCode {}

impl From<i32> for ErrorCode {
    fn from(number: i32) -> ErrorCode {
        match number {
            ffi::OPUS_BAD_ARG => ErrorCode::BadArgument,
            ffi::OPUS_BUFFER_TOO_SMALL => ErrorCode::BufferTooSmall,
            ffi::OPUS_INTERNAL_ERROR => ErrorCode::InternalError,
            ffi::OPUS_INVALID_PACKET => ErrorCode::InvalidPacket,
            ffi::OPUS_UNIMPLEMENTED => ErrorCode::Unimplemented,
            ffi::OPUS_INVALID_STATE => ErrorCode::InvalidState,
            ffi::OPUS_ALLOC_FAIL => ErrorCode::AllocFail,
            _ => ErrorCode::Unknown,
        }
    }
}

/// Checks if the `ffi_return_value` is documented by Opus.
/// Returns `Error` if value is negative.
pub fn try_map_opus_error(ffi_return_value: i32) -> Result<i32> {
    match ffi_return_value {
        v if v < 0 => Err(Error::from(ErrorCode::from(v))),
        _ => Ok(ffi_return_value),
    }
}

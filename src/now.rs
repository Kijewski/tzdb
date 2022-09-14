//! Get the current time in some time zone

use core::fmt;

use iana_time_zone::{get_timezone, GetTimezoneError};
use tz::error::ProjectDateTimeError;
use tz::{DateTime, TimeZoneRef};

#[allow(unreachable_pub)]
mod opaque {
    use core::fmt;

    #[derive(Copy, Clone)]
    pub struct Opaque;

    impl fmt::Debug for Opaque {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("_")
        }
    }
}

/// An error as returned by [`local()`] and similart functions
///
/// # See also:
///
/// * [`local()`] / [`local_or()`]
/// * [`in_named()`] / [`in_named_or()`]
/// * [`in_tz()`]
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum NowError {
    /// Could not get time zone. Only returned by [`local()`].
    TimeZone(GetTimezoneError),
    /// Unknown system time zone. Only returned by [`local()`], and [`in_named()`].
    UnknownTimezone(#[doc(hidden)] opaque::Opaque),
    /// Could not project timestamp.
    ProjectDateTime(ProjectDateTimeError),
    /// Could not get current time.
    Utcnow(utcnow::Error),
}

impl From<GetTimezoneError> for NowError {
    #[inline]
    fn from(err: GetTimezoneError) -> Self {
        Self::TimeZone(err)
    }
}

impl From<ProjectDateTimeError> for NowError {
    #[inline]
    fn from(err: ProjectDateTimeError) -> Self {
        Self::ProjectDateTime(err)
    }
}

impl From<utcnow::Error> for NowError {
    #[inline]
    fn from(err: utcnow::Error) -> Self {
        Self::Utcnow(err)
    }
}

impl fmt::Display for NowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::TimeZone(_) => "could not get time zone",
            Self::UnknownTimezone(_) => "unknown system time zone",
            Self::ProjectDateTime(_) => "could not project timestamp",
            Self::Utcnow(_) => "could not get current time",
        })
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for NowError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::TimeZone(err) => Some(err),
            Self::UnknownTimezone(_) => None,
            Self::ProjectDateTime(err) => Some(err),
            Self::Utcnow(err) => Some(err),
        }
    }
}

/// Get the current time in the local system time zone
///
/// # Errors
///
/// Possible errors include:
///
/// * The current [Unix time](https://en.wikipedia.org/w/index.php?title=Unix_time&oldid=1101650731)
///   could not be determined.
/// * The current Unix time could not be projected into the time zone.
///   Most likely the system time is off, or you are a time traveler trying run this code a few billion years in the future or past.
/// * The local time zone could not be determined.
/// * The local time zone is not a valid [IANA time zone](https://www.iana.org/time-zones).
///
/// # Example
///
/// ```rust
/// # fn main() -> Result<(), tzdb::now::NowError> {
/// // Query the time zone of the local system:
/// let now = tzdb::now::local()?;
/// # Ok(()) }
/// ```
///
/// In most cases you will want to default to a specified time zone if the system timezone
/// could not be determined. Then use e.g.
///
/// ```rust
/// # fn main() -> Result<(), tzdb::now::NowError> {
/// let now = tzdb::now::local_or(tzdb::time_zone::GMT)?;
/// # Ok(()) }
/// ```
///
/// # See also:
///
/// * `local()` / [`local_or()`]
/// * [`in_named()`] / [`in_named_or()`]
/// * [`in_tz()`]
pub fn local() -> Result<DateTime, NowError> {
    in_named(get_timezone()?)
}

/// Get the current time in the local system time zone with a fallback time zone
///
/// # Errors
///
/// Possible errors include:
///
/// * The current [Unix time](https://en.wikipedia.org/w/index.php?title=Unix_time&oldid=1101650731)
///   could not be determined.
/// * The current Unix time could not be projected into the time zone.
///   Most likely the system time is off, or you are a time traveler trying run this code a few billion years in the future or past.
///
/// # Example
///
/// ```rust
/// # fn main() -> Result<(), tzdb::now::NowError> {
/// // Query the time zone of the local system, or use GMT as default:
/// let now = tzdb::now::local_or(tzdb::time_zone::GMT)?;
/// # Ok(()) }
/// ```
///
/// # See also:
///
/// * [`local()`] / `local_or()`
/// * [`in_named()`] / [`in_named_or()`]
/// * [`in_tz()`]
pub fn local_or(default: TimeZoneRef<'_>) -> Result<DateTime, NowError> {
    let tz = get_timezone()
        .ok()
        .and_then(crate::tz_by_name)
        .unwrap_or(default);
    in_tz(tz)
}

/// Get the current time a given time zone
///
/// # Errors
///
/// Possible errors include:
///
/// * The current [Unix time](https://en.wikipedia.org/w/index.php?title=Unix_time&oldid=1101650731)
///   could not be determined.
/// * The current Unix time could not be projected into the time zone.
///   Most likely the system time is off, or you are a time traveler trying run this code a few billion years in the future or past.
///
/// # Example
///
/// ```rust
/// # fn main() -> Result<(), tzdb::now::NowError> {
/// // What is the time in Berlin?
/// let now = tzdb::now::in_tz(tzdb::time_zone::europe::BERLIN)?;
/// # Ok(()) }
/// ```
///
/// # See also:
///
/// * [`local()`] / [`local_or()`]
/// * [`in_named()`] / [`in_named_or()`]
/// * `in_tz()`
pub fn in_tz(time_zone_ref: TimeZoneRef<'_>) -> Result<DateTime, NowError> {
    let now = utcnow::utcnow()?;
    Ok(DateTime::from_timespec(
        now.as_secs(),
        now.subsec_nanos(),
        time_zone_ref,
    )?)
}

/// Get the current time in a given time zone, by name
///
/// # Errors
///
/// Possible errors include:
///
/// * The current [Unix time](https://en.wikipedia.org/w/index.php?title=Unix_time&oldid=1101650731)
///   could not be determined.
/// * The current Unix time could not be projected into the time zone.
///   Most likely the system time is off, or you are a time traveler trying run this code a few billion years in the future or past.
/// * The time zone is not a valid [IANA time zone](https://www.iana.org/time-zones).
///
/// # Example
///
/// ```rust
/// # fn main() -> Result<(), tzdb::now::NowError> {
/// // What is the time in Berlin?
/// let now = tzdb::now::in_named("Europe/Berlin")?;
/// # Ok(()) }
/// ```
///
/// In most cases you will want to default to a specified time zone if the time zone was not found.
/// Then use e.g.
///
/// ```rust
/// # fn main() -> Result<(), tzdb::now::NowError> {
/// let now = tzdb::now::in_named_or(tzdb::time_zone::GMT, "Some/City")?;
/// # Ok(()) }
/// ```
///
/// # See also:
///
/// * [`local()`] / [`local_or()`]
/// * `in_named()` / [`in_named_or()`]
/// * [`in_tz()`]
pub fn in_named(tz: impl AsRef<[u8]>) -> Result<DateTime, NowError> {
    in_tz(crate::tz_by_name(tz).ok_or(NowError::UnknownTimezone(opaque::Opaque))?)
}

/// Get the current time in a given time zone, by name, or default to some static time zone
///
/// # Errors
///
/// Possible errors include:
///
/// * The current [Unix time](https://en.wikipedia.org/w/index.php?title=Unix_time&oldid=1101650731)
///   could not be determined.
/// * The current Unix time could not be projected into the time zone.
///   Most likely the system time is off, or you are a time traveler trying run this code a few billion years in the future or past.
///
/// # Example
///
/// ```rust
/// # fn main() -> Result<(), tzdb::now::NowError> {
/// // What is the time in Some City?
/// let now = tzdb::now::in_named_or(tzdb::time_zone::GMT, "Some/City")?;
/// # Ok(()) }
/// ```
///
/// # See also:
///
/// * [`local()`] / [`local_or()`]
/// * [`in_named()`] / `in_named_or()`
/// * [`in_tz()`]
pub fn in_named_or(default: TimeZoneRef<'_>, tz: impl AsRef<[u8]>) -> Result<DateTime, NowError> {
    in_tz(crate::tz_by_name(tz).unwrap_or(default))
}

#[cfg(feature = "alloc")]
use crate::alloc_prelude::*;
use crate::{
    format::{parse, ParseError, ParseResult, ParsedItems},
    DeferredFormat, Duration,
};

/// An offset from UTC.
///
/// Guaranteed to store values up to ±23:59:59. Any values outside this range
/// may have incidental support that can change at any time without notice. If
/// you need support outside this range, please file an issue with your use
/// case.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(from = "crate::serde::UtcOffset", into = "crate::serde::UtcOffset")
)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UtcOffset {
    /// The number of seconds offset from UTC. Positive is east, negative is
    /// west.
    pub(crate) seconds: i32,
}

impl UtcOffset {
    /// A `UtcOffset` that is UTC.
    ///
    /// ```rust
    /// # use time::{UtcOffset, offset};
    /// assert_eq!(UtcOffset::UTC, offset!(UTC));
    /// ```
    pub const UTC: Self = Self::seconds(0);

    /// Create a `UtcOffset` representing an easterly offset by the number of
    /// hours provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::east_hours(1).as_hours(), 1);
    /// assert_eq!(UtcOffset::east_hours(2).as_minutes(), 120);
    /// ```
    #[inline(always)]
    pub const fn east_hours(hours: u8) -> Self {
        Self::hours(hours as i8)
    }

    /// Create a `UtcOffset` representing a westerly offset by the number of
    /// hours provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::west_hours(1).as_hours(), -1);
    /// assert_eq!(UtcOffset::west_hours(2).as_minutes(), -120);
    /// ```
    #[inline(always)]
    pub const fn west_hours(hours: u8) -> Self {
        Self::hours(-(hours as i8))
    }

    /// Create a `UtcOffset` representing an offset by the number of hours
    /// provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::hours(2).as_minutes(), 120);
    /// assert_eq!(UtcOffset::hours(-2).as_minutes(), -120);
    /// ```
    #[inline(always)]
    pub const fn hours(hours: i8) -> Self {
        Self::seconds(hours as i32 * 3_600)
    }

    /// Create a `UtcOffset` representing an easterly offset by the number of
    /// minutes provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::east_minutes(60).as_hours(), 1);
    /// ```
    #[inline(always)]
    pub const fn east_minutes(minutes: u16) -> Self {
        Self::minutes(minutes as i16)
    }

    /// Create a `UtcOffset` representing a westerly offset by the number of
    /// minutes provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::west_minutes(60).as_hours(), -1);
    /// ```
    #[inline(always)]
    pub const fn west_minutes(minutes: u16) -> Self {
        Self::minutes(-(minutes as i16))
    }

    /// Create a `UtcOffset` representing a offset by the number of minutes
    /// provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::minutes(60).as_hours(), 1);
    /// assert_eq!(UtcOffset::minutes(-60).as_hours(), -1);
    /// ```
    #[inline(always)]
    pub const fn minutes(minutes: i16) -> Self {
        Self::seconds(minutes as i32 * 60)
    }

    /// Create a `UtcOffset` representing an easterly offset by the number of
    /// seconds provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::east_seconds(3_600).as_hours(), 1);
    /// assert_eq!(UtcOffset::east_seconds(1_800).as_minutes(), 30);
    /// ```
    #[inline(always)]
    pub const fn east_seconds(seconds: u32) -> Self {
        Self::seconds(seconds as i32)
    }

    /// Create a `UtcOffset` representing a westerly offset by the number of
    /// seconds provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::west_seconds(3_600).as_hours(), -1);
    /// assert_eq!(UtcOffset::west_seconds(1_800).as_minutes(), -30);
    /// ```
    #[inline(always)]
    pub const fn west_seconds(seconds: u32) -> Self {
        Self::seconds(-(seconds as i32))
    }

    /// Create a `UtcOffset` representing an offset by the number of seconds
    /// provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::seconds(3_600).as_hours(), 1);
    /// assert_eq!(UtcOffset::seconds(-3_600).as_hours(), -1);
    /// ```
    #[inline(always)]
    pub const fn seconds(seconds: i32) -> Self {
        Self { seconds }
    }

    /// Get the number of seconds from UTC the value is. Positive is east,
    /// negative is west.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::UTC.as_seconds(), 0);
    /// assert_eq!(UtcOffset::hours(12).as_seconds(), 43_200);
    /// assert_eq!(UtcOffset::hours(-12).as_seconds(), -43_200);
    /// ```
    #[inline(always)]
    pub const fn as_seconds(self) -> i32 {
        self.seconds
    }

    /// Get the number of minutes from UTC the value is. Positive is east,
    /// negative is west.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::UTC.as_minutes(), 0);
    /// assert_eq!(UtcOffset::hours(12).as_minutes(), 720);
    /// assert_eq!(UtcOffset::hours(-12).as_minutes(), -720);
    /// ```
    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn as_minutes(self) -> i16 {
        (self.as_seconds() / 60) as i16
    }

    /// Get the number of hours from UTC the value is. Positive is east,
    /// negative is west.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::UTC.as_hours(), 0);
    /// assert_eq!(UtcOffset::hours(12).as_hours(), 12);
    /// assert_eq!(UtcOffset::hours(-12).as_hours(), -12);
    /// ```
    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn as_hours(self) -> i8 {
        (self.as_seconds() / 3_600) as i8
    }

    /// Convert a `UtcOffset` to ` Duration`. Useful for implementing operators.
    #[inline(always)]
    pub(crate) const fn as_duration(self) -> Duration {
        Duration::seconds(self.seconds as i64)
    }
}

/// Methods that allow parsing and formatting the `UtcOffset`.
impl UtcOffset {
    /// Format the `UtcOffset` using the provided string.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::hours(2).format("%z"), "+0200");
    /// assert_eq!(UtcOffset::hours(-2).format("%z"), "-0200");
    /// ```
    #[inline(always)]
    pub fn format(self, format: &str) -> String {
        DeferredFormat {
            date: None,
            time: None,
            offset: Some(self),
            format: crate::format::parse_fmt_string(format),
        }
        .to_string()
    }

    /// Attempt to parse the `UtcOffset` using the provided string.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::parse("+0200", "%z"), Ok(UtcOffset::hours(2)));
    /// assert_eq!(UtcOffset::parse("-0200", "%z"), Ok(UtcOffset::hours(-2)));
    /// ```
    #[inline(always)]
    pub fn parse(s: &str, format: &str) -> ParseResult<Self> {
        Self::try_from_parsed_items(parse(s, format)?)
    }

    /// Given the items already parsed, attempt to create a `UtcOffset`.
    #[inline(always)]
    pub(crate) fn try_from_parsed_items(items: ParsedItems) -> ParseResult<Self> {
        items.offset.ok_or(ParseError::InsufficientInformation)
    }
}

#[cfg(target_family = "windows")]
use std::{convert::TryFrom, mem::MaybeUninit};
#[cfg(target_family = "windows")]
use winapi::{
    shared::minwindef,
    um::{minwinbase, timezoneapi, winnt},
};
#[cfg(target_family = "windows")]
impl UtcOffset {
    fn to_systemtime(datetime: time::PrimitiveDateTime) -> minwinbase::SYSTEMTIME {
        let (month, day_of_month) = datetime.month_day();
        minwinbase::SYSTEMTIME {
            wYear: datetime.date.year() as u16,
            wMonth: month as u16,
            wDay: day_of_month as u16,
            wDayOfWeek: 0, // ignored
            wHour: datetime.time.hour() as u16,
            wMinute: datetime.time.minute() as u16,
            wSecond: datetime.time.second() as u16,
            wMilliseconds: datetime.time.millisecond(),
        }
    }

    #[allow(unsafe_code)]
    fn systemtime_to_filetime(systime: &minwinbase::SYSTEMTIME) -> Option<minwindef::FILETIME> {
        let mut ft = MaybeUninit::uninit();
        unsafe {
            if 0 == timezoneapi::SystemTimeToFileTime(systime, ft.as_mut_ptr()) {
                // failed
                None
            } else {
                Some(ft.assume_init())
            }
        }
    }

    #[allow(unsafe_code)]
    fn filetime_to_secs(filetime: &minwindef::FILETIME) -> i64 {
        // FILETIME represents 100-nanosecond intervals
        const FT_TO_SECS: i64 = 10_000_000;

        unsafe {
            let mut large_int: winnt::ULARGE_INTEGER = MaybeUninit::zeroed().assume_init();
            large_int.u_mut().LowPart = filetime.dwLowDateTime;
            large_int.u_mut().HighPart = filetime.dwHighDateTime as i32;
            *large_int.QuadPart() as i64 / FT_TO_SECS
        }
    }

    #[allow(unsafe_code)]
    pub fn local_offset_at(datetime: time::PrimitiveDateTime) -> Self {
        // This function falls back to Self::UTC if any system call fails.
        let systime_utc = Self::to_systemtime(datetime);
        let systime_local = unsafe {
            let mut lt = MaybeUninit::uninit();
            if 0 == timezoneapi::SystemTimeToTzSpecificLocalTime(
                std::ptr::null(), // use system's current timezone
                &systime_utc,
                lt.as_mut_ptr(),
            ) {
                // call failed
                return Self::UTC;
            } else {
                lt.assume_init()
            }
        };

        // Convert SYSTEMTIMEs to FILETIMEs so we can perform arithmentic on them.

        let ft_system = match Self::systemtime_to_filetime(&systime_utc) {
            Some(ft) => ft,
            None => return Self::UTC,
        };
        let ft_local = match Self::systemtime_to_filetime(&systime_local) {
            Some(ft) => ft,
            None => return Self::UTC,
        };

        let diff_secs = Self::filetime_to_secs(&ft_local) - Self::filetime_to_secs(&ft_system);

        i32::try_from(diff_secs)
            .map(|s| Self { seconds: s })
            .unwrap_or(Self::UTC)
    }

    #[allow(unsafe_code)]
    pub fn local() -> Self {
        unsafe {
            let mut tz_info = MaybeUninit::uninit();
            let result = timezoneapi::GetDynamicTimeZoneInformation(tz_info.as_mut_ptr());

            // The current bias for local time translation on this computer, in minutes.
            // The bias is the difference, in minutes, between Coordinated Universal Time
            // (UTC) and local time. All translations between UTC and local time are based
            // on the following formula:
            //
            // UTC = local time + bias
            let bias_mins = match result {
                // Daylight saving time is not used in the current time zone,
                // because there are no transition dates.
                winnt::TIME_ZONE_ID_UNKNOWN => tz_info.assume_init().Bias,
                // The system is operating in the range covered by the
                // StandardDate member of the DYNAMIC_TIME_ZONE_INFORMATION structure.
                winnt::TIME_ZONE_ID_STANDARD => {
                    tz_info.assume_init().Bias + tz_info.assume_init().StandardBias
                }
                // The system is operating in the range covered by the DaylightDate
                // member of the DYNAMIC_TIME_ZONE_INFORMATION structure
                winnt::TIME_ZONE_ID_DAYLIGHT => {
                    tz_info.assume_init().Bias + tz_info.assume_init().DaylightBias
                }
                // call failed somehow, return UTC
                _ => return Self::UTC,
            };
            UtcOffset {
                seconds: bias_mins * -60,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::offset;

    #[test]
    fn hours() {
        assert_eq!(UtcOffset::hours(1).as_seconds(), 3_600);
        assert_eq!(UtcOffset::hours(-1).as_seconds(), -3_600);
        assert_eq!(UtcOffset::hours(23).as_seconds(), 82_800);
        assert_eq!(UtcOffset::hours(-23).as_seconds(), -82_800);
    }

    #[test]
    fn directional_hours() {
        assert_eq!(UtcOffset::east_hours(1), offset!(+1));
        assert_eq!(UtcOffset::west_hours(1), offset!(-1));
    }

    #[test]
    fn minutes() {
        assert_eq!(UtcOffset::minutes(1).as_seconds(), 60);
        assert_eq!(UtcOffset::minutes(-1).as_seconds(), -60);
        assert_eq!(UtcOffset::minutes(1_439).as_seconds(), 86_340);
        assert_eq!(UtcOffset::minutes(-1_439).as_seconds(), -86_340);
    }

    #[test]
    fn directional_minutes() {
        assert_eq!(UtcOffset::east_minutes(1), offset!(+0:01));
        assert_eq!(UtcOffset::west_minutes(1), offset!(-0:01));
    }

    #[test]
    fn seconds() {
        assert_eq!(UtcOffset::seconds(1).as_seconds(), 1);
        assert_eq!(UtcOffset::seconds(-1).as_seconds(), -1);
        assert_eq!(UtcOffset::seconds(86_399).as_seconds(), 86_399);
        assert_eq!(UtcOffset::seconds(-86_399).as_seconds(), -86_399);
    }

    #[test]
    fn directional_seconds() {
        assert_eq!(UtcOffset::east_seconds(1), offset!(+0:00:01));
        assert_eq!(UtcOffset::west_seconds(1), offset!(-0:00:01));
    }

    #[test]
    fn as_hours() {
        assert_eq!(offset!(+1).as_hours(), 1);
        assert_eq!(offset!(+0:59).as_hours(), 0);
        assert_eq!(offset!(-1).as_hours(), -1);
        assert_eq!(offset!(-0:59).as_hours(), -0);
    }

    #[test]
    fn as_minutes() {
        assert_eq!(offset!(+1).as_minutes(), 60);
        assert_eq!(offset!(+0:01).as_minutes(), 1);
        assert_eq!(offset!(+0:00:59).as_minutes(), 0);
        assert_eq!(offset!(-1).as_minutes(), -60);
        assert_eq!(offset!(-0:01).as_minutes(), -1);
        assert_eq!(offset!(-0:00:59).as_minutes(), 0);
    }

    #[test]
    fn as_seconds() {
        assert_eq!(offset!(+1).as_seconds(), 3_600);
        assert_eq!(offset!(+0:01).as_seconds(), 60);
        assert_eq!(offset!(+0:00:01).as_seconds(), 1);
        assert_eq!(offset!(-1).as_seconds(), -3_600);
        assert_eq!(offset!(-0:01).as_seconds(), -60);
        assert_eq!(offset!(-0:00:01).as_seconds(), -1);
    }

    #[test]
    fn as_duration() {
        assert_eq!(offset!(+1).as_duration(), Duration::hours(1));
        assert_eq!(offset!(-1).as_duration(), Duration::hours(-1));
    }

    #[test]
    fn utc_is_zero() {
        assert_eq!(UtcOffset::UTC, offset!(+0));
    }

    #[test]
    fn format() {
        assert_eq!(offset!(+1).format("%z"), "+0100");
        assert_eq!(offset!(-1).format("%z"), "-0100");
        assert_eq!(offset!(+0).format("%z"), "+0000");
        // An offset of exactly zero should always have a positive sign.
        assert_ne!(offset!(-0).format("%z"), "-0000");

        assert_eq!(offset!(+0:01).format("%z"), "+0001");
        assert_eq!(offset!(-0:01).format("%z"), "-0001");

        // Seconds are not displayed, but the sign can still change.
        assert_eq!(offset!(+0:00:01).format("%z"), "+0000");
        assert_eq!(offset!(-0:00:01).format("%z"), "-0000");
    }

    #[test]
    fn parse() {
        assert_eq!(UtcOffset::parse("+0100", "%z"), Ok(offset!(+1)));
        assert_eq!(UtcOffset::parse("-0100", "%z"), Ok(offset!(-1)));
        assert_eq!(UtcOffset::parse("+0000", "%z"), Ok(offset!(+0)));
        assert_eq!(UtcOffset::parse("-0000", "%z"), Ok(offset!(+0)));

        assert_eq!(UtcOffset::parse("+0001", "%z"), Ok(offset!(+0:01)));
        assert_eq!(UtcOffset::parse("-0001", "%z"), Ok(offset!(-0:01)));
    }

    #[test]
    fn local() {
        let _ = UtcOffset::local();
    }

    #[test]
    fn local_offset_at() {
        let now = time::PrimitiveDateTime::now();
        let _ = UtcOffset::local_offset_at(now);
    }
}

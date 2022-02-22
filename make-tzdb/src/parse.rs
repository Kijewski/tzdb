use std::fmt;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) enum TimeZone {
    TimeZone {
        transitions: Vec<Transition>,
        local_time_types: NonEmptyVec<LocalTimeType>,
        leap_seconds: Vec<LeapSecond>,
        extra_rule: Option<TransitionRule>,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) enum Transition {
    Transition {
        unix_leap_time: i64,
        local_time_type_index: usize,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) enum NonEmptyVec<T> {
    NonEmptyVec { first: T, tail: Vec<T> },
}

#[derive(Debug, Deserialize)]
pub(crate) enum LeapSecond {
    LeapSecond {
        unix_leap_time: i64,
        correction: i32,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) enum TransitionRule {
    Fixed(LocalTimeType),
    Alternate {
        std: LocalTimeType,
        dst: LocalTimeType,
        dst_start: RuleDay,
        dst_start_time: i32,
        dst_end: RuleDay,
        dst_end_time: i32,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) enum LocalTimeType {
    LocalTimeType {
        ut_offset: i32,
        is_dst: bool,
        time_zone_designation: Option<String>,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) enum RuleDay {
    Julian1WithoutLeap(u16),
    Julian0WithLeap(u16),
    MonthWeekDay { month: u8, week: u8, week_day: u8 },
}

impl fmt::Display for TimeZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let TimeZone::TimeZone {
            transitions,
            local_time_types,
            leap_seconds,
            extra_rule,
        } = &self;

        writeln!(f, "::tz::statics::StaticTimeZone::new(")?;
        writeln!(f, "    &[")?;
        for transition in transitions {
            writeln!(f, "        {},", transition)?;
        }
        writeln!(f, "    ],")?;
        writeln!(f, "    {}", local_time_types)?;
        writeln!(f, "    &[")?;
        for t in leap_seconds {
            writeln!(f, "        {},", t)?;
        }
        writeln!(f, "    ],")?;
        match extra_rule {
            Some(t) => writeln!(f, "    Some({}),", t)?,
            None => writeln!(f, "    None,")?,
        }
        writeln!(f, ")")?;
        Ok(())
    }
}

impl fmt::Display for Transition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Transition::Transition {
            unix_leap_time,
            local_time_type_index,
        } = &self;
        writeln!(
            f,
            "::tz::Transition::new({}, {})",
            unix_leap_time, local_time_type_index
        )?;
        Ok(())
    }
}

impl fmt::Display for LocalTimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let LocalTimeType::LocalTimeType {
            ut_offset,
            is_dst,
            time_zone_designation,
        } = self;
        writeln!(
            f,
            "::tz::LocalTimeType::new({}, {}, {:?})",
            ut_offset, is_dst, time_zone_designation,
        )?;
        Ok(())
    }
}

impl fmt::Display for LeapSecond {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let LeapSecond::LeapSecond {
            unix_leap_time,
            correction,
        } = self;
        writeln!(
            f,
            "::tz::LeapSecond::new({}, {})",
            unix_leap_time, correction,
        )?;
        Ok(())
    }
}

impl fmt::Display for TransitionRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransitionRule::Fixed(t) => writeln!(f, "::tz::TransitionRule::Fixed({})", t)?,
            TransitionRule::Alternate {
                std,
                dst,
                dst_start,
                dst_start_time,
                dst_end,
                dst_end_time,
            } => {
                writeln!(f, "::tz::TransitionRule::Alternate {{")?;
                writeln!(f, "            std: {},", std)?;
                writeln!(f, "            dst: {},", dst)?;
                writeln!(f, "            dst_start: {},", dst_start)?;
                writeln!(f, "            dst_start_time: {},", dst_start_time)?;
                writeln!(f, "            dst_end: {},", dst_end)?;
                writeln!(f, "            dst_end_time: {},", dst_end_time)?;
                writeln!(f, "        }}")?;
            },
        }
        Ok(())
    }
}

impl fmt::Display for RuleDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuleDay::Julian1WithoutLeap(t) => {
                writeln!(f, "::tz::RuleDay::Julian1WithoutLeap({})", t)?
            },
            RuleDay::Julian0WithLeap(t) => writeln!(f, "::tz::RuleDay::Julian0WithLeap({})", t)?,
            RuleDay::MonthWeekDay {
                month,
                week,
                week_day,
            } => {
                writeln!(f, "::tz::RuleDay::MonthWeekDay {{")?;
                writeln!(f, "            month: {},", month)?;
                writeln!(f, "            week: {},", week)?;
                writeln!(f, "            week_day: {},", week_day)?;
                writeln!(f, "        }}")?;
            },
        }
        Ok(())
    }
}

impl<T: fmt::Display> fmt::Display for NonEmptyVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let NonEmptyVec::NonEmptyVec { first, tail } = self;

        writeln!(f, "    (")?;
        writeln!(f, "        {},", first)?;
        writeln!(f, "        &[")?;
        for t in tail {
            writeln!(f, "        {},", t)?;
        }
        writeln!(f, "        ],")?;
        writeln!(f, "    ),")?;

        Ok(())
    }
}

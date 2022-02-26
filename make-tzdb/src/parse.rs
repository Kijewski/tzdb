use std::fmt;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) enum TimeZone {
    TimeZone {
        transitions: Vec<Transition>,
        local_time_types: Vec<GenericLocalTimeType>,
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
pub(crate) enum LeapSecond {
    LeapSecond {
        unix_leap_time: i64,
        correction: i32,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) enum TransitionRule {
    Fixed(GenericLocalTimeType),
    Alternate(GenericAlternateTime),
}

#[derive(Debug, Deserialize)]
pub(crate) enum GenericAlternateTime {
    GenericAlternateTime {
        std: GenericLocalTimeType,
        dst: GenericLocalTimeType,
        dst_start: RuleDay,
        dst_start_time: i32,
        dst_end: RuleDay,
        dst_end_time: i32,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) enum GenericLocalTimeType {
    GenericLocalTimeType {
        ut_offset: i32,
        is_dst: bool,
        time_zone_designation: Option<String>,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) enum RuleDay {
    Julian1WithoutLeap(Julian1WithoutLeap),
    Julian0WithLeap(Julian0WithLeap),
    MonthWeekDay(MonthWeekDay),
}

#[derive(Debug, Deserialize)]
pub(crate) enum Julian1WithoutLeap {
    Julian1WithoutLeap(u16),
}

#[derive(Debug, Deserialize)]
pub(crate) enum Julian0WithLeap {
    Julian0WithLeap(u16),
}

#[derive(Debug, Deserialize)]
pub(crate) enum MonthWeekDay {
    MonthWeekDay { month: u8, week: u8, week_day: u8 },
}

impl fmt::Display for Transition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Transition::Transition {
            unix_leap_time,
            local_time_type_index,
        } = &self;
        writeln!(
            f,
            "Transition::new({}, {})",
            unix_leap_time, local_time_type_index
        )?;
        Ok(())
    }
}

impl fmt::Display for GenericLocalTimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let GenericLocalTimeType::GenericLocalTimeType {
            ut_offset,
            is_dst,
            time_zone_designation,
        } = self;
        writeln!(
            f,
            "StaticLocalTimeType::new({}, {}, {:?})",
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
        writeln!(f, "LeapSecond::new({}, {})", unix_leap_time, correction,)?;
        Ok(())
    }
}

impl fmt::Display for TransitionRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransitionRule::Fixed(t) => writeln!(f, "StaticTransitionRule::Fixed({})", Unwrap(t))?,
            TransitionRule::Alternate(t) => {
                writeln!(f, "StaticTransitionRule::Alternate({})", Unwrap(t))?;
            },
        }
        Ok(())
    }
}

impl fmt::Display for GenericAlternateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let GenericAlternateTime::GenericAlternateTime {
            std,
            dst,
            dst_start,
            dst_start_time,
            dst_end,
            dst_end_time,
        } = self;
        writeln!(
            f,
            "StaticAlternateTime::new({}, {}, {}, {}, {}, {})",
            Unwrap(std),
            Unwrap(dst),
            dst_start,
            dst_start_time,
            dst_end,
            dst_end_time,
        )
    }
}

impl fmt::Display for MonthWeekDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let MonthWeekDay::MonthWeekDay {
            month,
            week,
            week_day,
        } = self;
        writeln!(f, "MonthWeekDay::new({}, {}, {})", month, week, week_day,)
    }
}

impl fmt::Display for Julian0WithLeap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Julian0WithLeap::Julian0WithLeap(t) = self;
        writeln!(f, "Julian0WithLeap::new({})", t)
    }
}

impl fmt::Display for Julian1WithoutLeap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Julian1WithoutLeap::Julian1WithoutLeap(t) = self;
        writeln!(f, "Julian1WithoutLeap::new({})", t)
    }
}

impl fmt::Display for RuleDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuleDay::Julian0WithLeap(t) => writeln!(f, "RuleDay::Julian0WithLeap({})", Unwrap(t))?,
            RuleDay::Julian1WithoutLeap(t) => {
                writeln!(f, "RuleDay::Julian1WithoutLeap({})", Unwrap(t))?
            },
            RuleDay::MonthWeekDay(t) => {
                writeln!(f, "RuleDay::MonthWeekDay({})", Unwrap(t))?;
            },
        }
        Ok(())
    }
}

pub(crate) struct Unwrap<'a, T: fmt::Display>(pub(crate) &'a T);

impl<'a, T: fmt::Display> fmt::Display for Unwrap<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"match {} {{ Ok(v) => v, Err(e) => panic!("{{}}", e.0) }}"#,
            &self.0
        )
    }
}

pub(crate) struct UnwrapToConst<'a, T>(pub(crate) &'a str, pub(crate) &'a [T]);

impl<'a, T: fmt::Display> fmt::Display for UnwrapToConst<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, r"{{ const V: [{}; {}] = [", &self.0, self.1.len())?;
        for elem in self.1 {
            writeln!(f, "    {},", Unwrap(elem))?;
        }
        writeln!(f, "]; V }}")
    }
}

pub(crate) struct DisplayVec<'a, T>(pub(crate) &'a [T]);

impl<'a, T: fmt::Display> fmt::Display for DisplayVec<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[")?;
        for elem in self.0 {
            writeln!(f, "    {},", elem)?;
        }
        writeln!(f, "]")
    }
}

pub(crate) struct DisplayOption<'a, T>(Option<&'a T>);

impl<'a, T: fmt::Display> fmt::Display for DisplayOption<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(v) => writeln!(f, "Some({})", v),
            None => writeln!(f, "None"),
        }
    }
}

impl fmt::Display for TimeZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let TimeZone::TimeZone {
            transitions,
            local_time_types,
            leap_seconds,
            extra_rule,
        } = self;
        writeln!(
            f,
            "StaticTimeZone::new(&{}, &{}, &{}, {})",
            DisplayVec(transitions),
            UnwrapToConst("StaticLocalTimeType", local_time_types),
            DisplayVec(leap_seconds),
            DisplayOption(extra_rule.as_ref()),
        )
    }
}

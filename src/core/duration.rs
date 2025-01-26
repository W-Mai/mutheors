use std::fmt::{Display, Formatter};

///
/// Duration represents the length of a note.
///
/// `real duration = 6.0 / duration`
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum Duration {
    Whole = 6,
    Half = 12,
    Quarter = 24,
    Eighth = 48,
    Sixteenth = 96,
    ThirtySecond = 192,
    SixtyFourth = 384,
    HundredTwentyEighth = 768,

    // Dotted notes
    WholeDotted = 4,
    HalfDotted = 8,
    QuarterDotted = 16,
    EighthDotted = 32,
    SixteenthDotted = 64,
    ThirtySecondDotted = 128,
    SixtyFourthDotted = 256,
    HundredTwentyEighthDotted = 512,
}

impl From<u16> for Duration {
    fn from(value: u16) -> Self {
        match value {
            6 => Duration::Whole,
            12 => Duration::Half,
            24 => Duration::Quarter,
            48 => Duration::Eighth,
            96 => Duration::Sixteenth,
            192 => Duration::ThirtySecond,
            384 => Duration::SixtyFourth,
            768 => Duration::HundredTwentyEighth,
            // Dotted notes
            4 => Duration::WholeDotted,
            8 => Duration::HalfDotted,
            16 => Duration::QuarterDotted,
            32 => Duration::EighthDotted,
            64 => Duration::SixteenthDotted,
            128 => Duration::ThirtySecondDotted,
            256 => Duration::SixtyFourthDotted,
            512 => Duration::HundredTwentyEighthDotted,
            _ => panic!("Invalid value"),
        }
    }
}

impl From<Duration> for f32 {
    fn from(duration: Duration) -> f32 {
        6.0 / (duration as u16 as f32)
    }
}

impl From<Duration> for f64 {
    fn from(duration: Duration) -> f64 {
        f32::from(duration) as f64
    }
}

impl From<&Duration> for f32 {
    fn from(duration: &Duration) -> f32 {
        f32::from(*duration)
    }
}

impl From<&Duration> for f64 {
    fn from(duration: &Duration) -> f64 {
        f64::from(*duration)
    }
}

impl From<f32> for Duration {
    fn from(value: f32) -> Self {
        let duration = (6.0 / value) as u16;
        Duration::from(duration)
    }
}

impl From<f64> for Duration {
    fn from(value: f64) -> Self {
        Duration::from(value as f32)
    }
}

impl std::ops::Add<Duration> for Duration {
    type Output = f32;

    fn add(self, rhs: Duration) -> Self::Output {
        f32::from(self) + f32::from(rhs)
    }
}

impl std::ops::Add<f32> for Duration {
    type Output = f32;

    fn add(self, rhs: f32) -> Self::Output {
        f32::from(self) + rhs
    }
}

impl std::ops::Add<f64> for Duration {
    type Output = f64;

    fn add(self, rhs: f64) -> Self::Output {
        f64::from(self) + rhs
    }
}

impl std::ops::Add<Duration> for f32 {
    type Output = f32;

    fn add(self, rhs: Duration) -> Self::Output {
        self + f32::from(rhs)
    }
}

impl std::ops::Add<Duration> for f64 {
    type Output = f64;

    fn add(self, rhs: Duration) -> Self::Output {
        self + f64::from(rhs)
    }
}

impl std::ops::Add<&Duration> for f64 {
    type Output = f64;

    fn add(self, rhs: &Duration) -> Self::Output {
        self + f64::from(rhs)
    }
}

impl std::ops::AddAssign<Duration> for f32 {
    fn add_assign(&mut self, rhs: Duration) {
        *self += f32::from(rhs);
    }
}

impl std::ops::AddAssign<Duration> for f64 {
    fn add_assign(&mut self, rhs: Duration) {
        *self += f64::from(rhs);
    }
}

impl std::ops::AddAssign<&Duration> for f64 {
    fn add_assign(&mut self, rhs: &Duration) {
        *self += f64::from(rhs);
    }
}

impl std::ops::AddAssign<f32> for Duration {
    fn add_assign(&mut self, rhs: f32) {
        *self = Duration::from(f32::from(*self) + rhs);
    }
}

impl std::ops::AddAssign<f64> for Duration {
    fn add_assign(&mut self, rhs: f64) {
        *self = Duration::from(f64::from(*self) + rhs);
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let duration_str = match self {
            Duration::Whole => "Whole",
            Duration::Half => "½",
            Duration::Quarter => "¼",
            Duration::Eighth => "Eighth",
            Duration::Sixteenth => "Sixteenth",
            Duration::ThirtySecond => "ThirtySecond",
            Duration::SixtyFourth => "SixtyFourth",
            Duration::HundredTwentyEighth => "HundredTwentyEighth",
            Duration::WholeDotted => "WholeDotted",
            Duration::HalfDotted => "HalfDotted",
            Duration::QuarterDotted => "QuarterDotted",
            Duration::EighthDotted => "EighthDotted",
            Duration::SixteenthDotted => "SixteenthDotted",
            Duration::ThirtySecondDotted => "ThirtySecondDotted",
            Duration::SixtyFourthDotted => "SixtyFourthDotted",
            Duration::HundredTwentyEighthDotted => "HundredTwentyEighthDotted",
        };

        write!(f, "{}", duration_str)
    }
}

impl Duration {
    pub fn with_dot(&self) -> Duration {
        match self {
            Duration::Whole => Duration::WholeDotted,
            Duration::Half => Duration::HalfDotted,
            Duration::Quarter => Duration::QuarterDotted,
            Duration::Eighth => Duration::EighthDotted,
            Duration::Sixteenth => Duration::SixteenthDotted,
            Duration::ThirtySecond => Duration::ThirtySecondDotted,
            Duration::SixtyFourth => Duration::SixtyFourthDotted,
            Duration::HundredTwentyEighth => Duration::HundredTwentyEighthDotted,
            _ => *self,
        }
    }
}

pub mod duration_utils {
    use super::Duration;
    use rand::prelude::*;

    pub fn generate_one_measure(beat: u8) -> Vec<Duration> {
        let beat = beat as f64;
        let mut durations = vec![];
        let mut rng = thread_rng();
        let mut duration_sum = 0.0;
        while duration_sum < beat {
            let duration = *[Duration::Half, Duration::Quarter]
                .choose(&mut rng)
                .unwrap();
            if duration_sum + duration > beat {
                break;
            }
            duration_sum += duration;
            durations.push(duration);
        }

        let remainder = beat - duration_sum;
        if remainder > 0.0 {
            durations.push(remainder.into());
        }
        durations
    }
}

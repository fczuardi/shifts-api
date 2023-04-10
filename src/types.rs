use chrono::NaiveDateTime;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct FacilityId(pub i32);

pub struct WorkerId(pub i32);

pub struct ShiftStartTime(NaiveDateTime);

pub struct ShiftEndTime(NaiveDateTime);

#[derive(sqlx::Type, Debug)]
pub enum WorkerProfession {
    CNA,
    LVN,
    RN,
}

#[derive(Debug)]
pub struct Worker {
    pub profession: WorkerProfession,
    pub is_active: bool,
}

#[derive(Debug, PartialEq)]
pub struct Shift;

#[derive(Debug, PartialEq)]
pub enum IneligibilityReason {
    InactiveFacility,
    InactiveWorker,
}

#[derive(Debug, PartialEq)]
pub enum ShiftListError {
    DatabaseError(String),
    EligibilityError(IneligibilityReason),
}

macro_rules! impl_try_from_for_shift_time {
    ($shift_time:ident) => {
        impl TryFrom<&str> for $shift_time {
            type Error = chrono::ParseError;

            fn try_from(time_str: &str) -> Result<Self, Self::Error> {
                let datetime = NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M")?;
                Ok(Self(datetime))
            }
        }
    };
}
impl_try_from_for_shift_time!(ShiftStartTime);
impl_try_from_for_shift_time!(ShiftEndTime);

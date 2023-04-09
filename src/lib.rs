use chrono::NaiveDateTime;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct FacilityId(i32);

pub struct WorkerId(i32);

pub struct ShiftStartTime(NaiveDateTime);

pub struct ShiftEndTime(NaiveDateTime);

#[derive(Debug, PartialEq)]
pub struct Shift;

#[derive(Debug, PartialEq)]
pub enum IneligibilityReason {
    InactiveFacility,
}

#[derive(Debug, PartialEq)]
pub enum ShiftListError {
    EligibilityError(IneligibilityReason),
}

// Story:
//
//     As a worker, I want to get all available shifts that I'm eligible for to work at a facility.
#[cfg(test)]
mod tests {
    use super::*;

    use env_logger;
    use log::debug;

    // Given an inactive facility, when I request all available shifts
    // within a start and end date, then it will not return a list of shifts
    // from that facility.
    #[test]
    fn test_shifts_of_inactive_facility() -> Result<(), String> {
        env_logger::try_init().ok();

        let worker_id = WorkerId(1);
        let inactive_facility_id = FacilityId(4);
        let start = ShiftStartTime::try_from("2023-01-01 00:00").unwrap();
        let end = ShiftEndTime::try_from("2023-01-31 23:59").unwrap();
        let result = list_eligible_shifts(worker_id, inactive_facility_id, start, end);
        debug!("{:?}", result);
        assert_eq!(
            result,
            Err(ShiftListError::EligibilityError(
                IneligibilityReason::InactiveFacility
            ))
        );
        Ok(())
    }

    // Falicities 4, 6, 9, 10 are inactive on bootstrap db
    #[test]
    fn test_is_facility_active() {
        env_logger::try_init().ok();

        let facility_id = FacilityId(4);
        let result = is_facility_active(facility_id);
        debug!("{:?}", result);
        assert_eq!(result, Ok(false));

        let facility_id = FacilityId(5);
        let result = is_facility_active(facility_id);
        debug!("{:?}", result);
        assert_eq!(result, Ok(true));
    }
}

pub fn list_eligible_shifts(
    _worker: WorkerId,
    facility_id: FacilityId,
    _start: ShiftStartTime,
    _end: ShiftEndTime,
) -> Result<Vec<Shift>, ShiftListError> {
    match is_facility_active(facility_id) {
        Ok(false) => Err(ShiftListError::EligibilityError( IneligibilityReason::InactiveFacility,)),
        _ => unimplemented!(),
    }
}

fn is_facility_active(facility_id: FacilityId) -> Result<bool, String> {
    // TODO: replace this with the actual db query
    Ok(![4, 6, 9, 10].contains(&facility_id.0))
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

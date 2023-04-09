use chrono::NaiveDateTime;
use sqlx::PgPool;
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
    use sqlx::PgPool;

    // Given an inactive facility, when I request all available shifts
    // within a start and end date, then it will not return a list of shifts
    // from that facility.
    #[sqlx::test(fixtures("facilities"))]
    async fn test_shifts_of_inactive_facility(pool: PgPool) -> Result<(), String> {
        env_logger::try_init().ok();

        let worker_id = WorkerId(1);
        let inactive_facility_id = FacilityId(4);
        let start = ShiftStartTime::try_from("2023-01-01 00:00").unwrap();
        let end = ShiftEndTime::try_from("2023-01-31 23:59").unwrap();
        let result = list_eligible_shifts(&pool, worker_id, inactive_facility_id, start, end).await;
        debug!("{:?}", result);
        assert_eq!(
            result,
            Err(ShiftListError::EligibilityError(
                IneligibilityReason::InactiveFacility
            ))
        );
        Ok(())
    }

    #[sqlx::test(fixtures("facilities"))]
    async fn test_is_facility_active(pool: PgPool) {
        env_logger::try_init().ok();

        let inactive_facility_id = FacilityId(4);
        let result = is_facility_active(&pool, inactive_facility_id).await;
        debug!("{:?}", result);
        assert_eq!(result.unwrap(), false);

        let active_facility_id = FacilityId(5);
        let result = is_facility_active(&pool, active_facility_id).await;
        debug!("{:?}", result);
        assert_eq!(result.unwrap(), true);
    }
}

pub async fn list_eligible_shifts(
    pool: &PgPool,
    _worker: WorkerId,
    facility_id: FacilityId,
    _start: ShiftStartTime,
    _end: ShiftEndTime,
) -> Result<Vec<Shift>, ShiftListError> {
    match is_facility_active(pool, facility_id).await {
        Ok(false) => Err(ShiftListError::EligibilityError(
            IneligibilityReason::InactiveFacility,
        )),
        _ => unimplemented!(),
    }
}

async fn is_facility_active(pool: &PgPool, facility_id: FacilityId) -> Result<bool, sqlx::Error> {
    let is_active = sqlx::query!(
        r#"
        SELECT is_active
        FROM "Facility"
        WHERE id = $1
        "#,
        facility_id.0
    )
    .fetch_one(pool)
    .await?
    .is_active;

    Ok(is_active)
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

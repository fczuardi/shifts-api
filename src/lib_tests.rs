// Story:
//
//     As a worker, I want to get all available shifts that I'm eligible for to work at a facility.
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


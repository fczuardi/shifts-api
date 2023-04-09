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

//  - In order for a Worker to be eligible for a shift, the Worker must be active.
#[sqlx::test(fixtures("workers"))]
async fn test_shifts_of_inactive_worker(pool: PgPool) -> Result<(), String> {
    let inactive_worker_id = WorkerId(5);
    let facility_id = FacilityId(5);
    let start = ShiftStartTime::try_from("2023-01-01 00:00").unwrap();
    let end = ShiftEndTime::try_from("2023-01-31 23:59").unwrap();
    let result = list_eligible_shifts(&pool, inactive_worker_id, facility_id, start, end).await;
    debug!("{:?}", result);
    assert_eq!(
        result,
        Err(ShiftListError::EligibilityError(
            IneligibilityReason::InactiveWorker
        ))
    );
    Ok(())
}

#[sqlx::test(fixtures("facilities"))]
async fn test_is_facility_active(pool: PgPool) {
    env_logger::try_init().ok();

    let active_facility_id = FacilityId(5);
    let result = is_facility_active(&pool, active_facility_id).await;
    debug!("{:?}", result);
    assert_eq!(result.unwrap(), true);

    let inactive_facility_id = FacilityId(4);
    let result = is_facility_active(&pool, inactive_facility_id).await;
    debug!("{:?}", result);
    assert_eq!(result.unwrap(), false);
}

#[sqlx::test(fixtures("workers"))]
async fn test_is_worker_active(pool: PgPool) {
    env_logger::try_init().ok();

    let active_worker_id = WorkerId(4);
    let result = is_worker_active(&pool, active_worker_id).await;
    debug!("{:?}", result);
    assert_eq!(result.unwrap(), true);

    let inactive_worker_id = WorkerId(5);
    let result = is_worker_active(&pool, inactive_worker_id).await;
    debug!("{:?}", result);
    assert_eq!(result.unwrap(), false);
}

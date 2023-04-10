use sqlx::PgPool;
use types::{
    FacilityId, IneligibilityReason, Shift, ShiftEndTime, ShiftListError, ShiftStartTime, Worker,
    WorkerId, WorkerProfession,
};

mod types;

#[cfg(test)]
mod tests {
    include!("lib_tests.rs");
}

pub async fn list_eligible_shifts(
    pool: &PgPool,
    worker_id: WorkerId,
    facility_id: FacilityId,
    _start: ShiftStartTime,
    _end: ShiftEndTime,
) -> Result<Vec<Shift>, ShiftListError> {
    match is_facility_active(pool, facility_id).await {
        Ok(false) => {
            return Err(ShiftListError::EligibilityError(
                IneligibilityReason::InactiveFacility,
            ));
        }
        Err(e) => {
            return Err(ShiftListError::DatabaseError(e.to_string()));
        }
        _ => (),
    };

    let profession = match get_worker(pool, worker_id).await {
        Ok(worker) if worker.is_active => worker.profession,
        Ok(_) => {
            return Err(ShiftListError::EligibilityError(
                IneligibilityReason::InactiveWorker,
            ))
        }
        Err(e) => return Err(ShiftListError::DatabaseError(e.to_string())),
    };
    // Filters for the query:
    // - The professions between the Shift and Worker must match.
    //      - profession = worker.profession
    // - The Shift must be active(`is_deleted=False`) and not claimed by someone else.
    //      - NOT is_deleted
    // - The Worker must not have claimed a shift that collides with the shift they are is eligible for.
    //      - worker_id is null
    unimplemented!()
}

async fn get_worker(pool: &PgPool, worker_id: WorkerId) -> Result<Worker, sqlx::Error> {
    let worker = sqlx::query_as!(
        Worker,
        r#"
        SELECT profession as "profession: WorkerProfession", is_active
        FROM "Worker"
        WHERE id = $1
        "#,
        worker_id.0
    )
    .fetch_one(pool)
    .await?;

    Ok(worker)
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

use sqlx::PgPool;
use types::{
    FacilityId, IneligibilityReason, Shift, ShiftEndTime, ShiftId, ShiftListError, ShiftStartTime,
    Worker, WorkerId, WorkerProfession,
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
    start: ShiftStartTime,
    end: ShiftEndTime,
) -> Result<Vec<Shift>, ShiftListError> {
    match is_facility_active(pool, &facility_id).await {
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

    let shifts = sqlx::query_as!(
        Shift,
        r#"
        SELECT id as "id: ShiftId", start as "start: ShiftStartTime", "end" as "end: ShiftEndTime"
        FROM "Shift"
        WHERE NOT is_deleted AND
          facility_id = $1 AND
          "start" >= $2 AND
          "end" <= $3 AND
          profession = $4 AND
          'True'
        "#,
        facility_id.0,
        start.0,
        end.0,
        profession as _,
    )
    .fetch_all(pool)
    .await
    .unwrap();
    Ok(shifts)
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

async fn is_facility_active(pool: &PgPool, facility_id: &FacilityId) -> Result<bool, sqlx::Error> {
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

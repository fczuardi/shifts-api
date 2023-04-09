use sqlx::PgPool;
use types::{
    FacilityId, IneligibilityReason, Shift, ShiftEndTime, ShiftListError, ShiftStartTime, WorkerId,
};

mod types;

#[cfg(test)]
mod tests {
    include!("lib_tests.rs");
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

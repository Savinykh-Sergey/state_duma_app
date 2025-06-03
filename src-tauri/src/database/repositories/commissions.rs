use chrono::Utc;
use uuid::Uuid;
use crate::database::dto::commissions::ReportCommission;
use crate::database::entity::commissions::Commissions;
use crate::database::enums::commission_referral::CommissionDirection;
use crate::database::init::get_pool;
use crate::types::ResultWithStringError;

pub async fn get_commissions() -> ResultWithStringError<Vec<Commissions>> {
    let conn = get_pool().await;

    let result = sqlx::query_as::<_, Commissions>(r#"
            SELECT *
            FROM commissions
            ORDER BY created_at
    "#)
        .fetch_all(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(result)
}

pub async fn get_commission(id: Uuid) -> ResultWithStringError<Commissions> {
    let conn = get_pool().await;

    let result = sqlx::query_as::<_, Commissions>(r#"
            SELECT *
            FROM commissions
            WHERE id = $1
    "#)
        .bind(id)
        .fetch_one(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(result)
}

pub async fn add_commission(
    direction: CommissionDirection,
    chairman_id: Uuid,
    organizer_id: Uuid,
) -> ResultWithStringError<()> {
    let conn = get_pool().await;

    let now = Utc::now().naive_local();

    sqlx::query(r#"
            INSERT INTO commissions (id, direction, chairman_id, organizer_id, created_at, updated_at)
            VALUES (uuid_generate_v4(), $1::commission_direction, $2, $3, $4, $5)
    "#)
        .bind(direction)
        .bind(chairman_id)
        .bind(organizer_id)
        .bind(now)
        .bind(now)
        .execute(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;
    Ok(())
}

pub async fn update_commission(
    id: Uuid,
    direction: CommissionDirection,
    chairman_id: Uuid,
    organizer_id: Uuid,
) -> ResultWithStringError<()> {
    let conn = get_pool().await;

    sqlx::query(r#"
            UPDATE commissions
            SET direction = $1::commission_direction,
                chairman_id = $2,
                organizer_id = $3,
                updated_at = $4
            WHERE id = $5
    "#)
        .bind(direction)
        .bind(chairman_id)
        .bind(organizer_id)
        .bind(Utc::now().naive_local())
        .bind(id)
        .execute(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(())
}

pub async fn delete_commission(id: Uuid) -> ResultWithStringError<()> {
    let conn = get_pool().await;

    sqlx::query(r#"
        DELETE FROM commissions
        WHERE id = $1
    "#)
        .bind(id)
        .execute(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(())
}

pub async fn get_commission_members_by_direction(direction: CommissionDirection) -> ResultWithStringError<Vec<ReportCommission>> {
    let conn = get_pool().await;

    let result = sqlx::query_as::<_, ReportCommission>(r#"
        SELECT members.full_name AS full_name, members.is_chairman AS is_chairman
        FROM commissions
            JOIN commission_members ON commissions.id = commission_members.commission_id
            JOIN members ON commission_members.member_id = members.id
        WHERE commissions.direction = $1
    "#)
        .bind(direction)
        .fetch_all(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(result)
}
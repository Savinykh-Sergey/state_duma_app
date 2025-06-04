use uuid::Uuid;

use crate::database::init::get_pool;
use crate::database::entity::commission_members::CommissionMembers;
use crate::types::ResultWithStringError;

pub async fn add_commission_member(member_id: Uuid, commission_id: Uuid) -> ResultWithStringError<()> {
    let conn = get_pool().await;

    sqlx::query(r#"
        INSERT INTO commission_members (id, member_id, commission_id, created_at, updated_at)
        VALUES (uuid_generate_v4(), $1, $2, NOW(), NOW())
    "#)
        .bind(member_id)
        .bind(commission_id)
        .execute(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(())
}

pub async fn edit_commission_member(member_id: Uuid, commission_id: Uuid) -> ResultWithStringError<()> {
    let conn = get_pool().await;

    sqlx::query(r#"
        UPDATE commission_members
        SET commission_id = $1
        WHERE member_id = $2
    "#)
        .bind(commission_id)
        .bind(member_id)
        .execute(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(())
}

pub async fn get_commission_member(id: Uuid) -> ResultWithStringError<CommissionMembers> {
    let conn = get_pool().await;

    let result = sqlx::query_as::<_, CommissionMembers>(r#"
        SELECT *
        FROM commission_members
        WHERE member_id = $1::uuid
    "#)
        .bind(id)
        .fetch_one(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(result)
}
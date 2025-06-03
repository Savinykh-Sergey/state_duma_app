use uuid::Uuid;
use crate::database::entity::members::Members;
use crate::database::init::get_pool;
use crate::types::ResultWithStringError;

pub async fn get_chairmen() -> ResultWithStringError<Vec<Members>> {
    let conn = get_pool().await;

    let result = sqlx::query_as::<_, Members>(r#"
            SELECT *
            FROM members
            WHERE is_chairman = true
            ORDER BY birthday
    "#)
        .fetch_all(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(result)
}

pub async fn delete_chairmen(id: Uuid) -> ResultWithStringError<()> {
    let conn = get_pool().await;

    sqlx::query(r#"
        UPDATE members
        SET is_chairman = false
        WHERE id = $1
    "#)
        .bind(id)
        .execute(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(())
}


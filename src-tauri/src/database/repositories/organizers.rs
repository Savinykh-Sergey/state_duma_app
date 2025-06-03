use chrono::{NaiveDate, Utc};
use crate::database::entity::organizers::Organizers;
use crate::database::init::get_pool;
use crate::types::ResultWithStringError;
use uuid::Uuid;

pub async fn get_organizers() -> ResultWithStringError<Vec<Organizers>> {
    let conn = get_pool().await;

    let result = sqlx::query_as::<_, Organizers>(
        r#"
            SELECT *
            FROM organizers
            ORDER BY birthday
        "#,
    )
    .fetch_all(conn)
    .await
    .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(result)
}

pub async fn get_organizer(id: Uuid) -> ResultWithStringError<Organizers> {
    let conn = get_pool().await;

    let result = sqlx::query_as::<_, Organizers>(
        r#"
            SELECT * 
            FROM organizers 
            WHERE id = $1
    "#,
    )
    .bind(id)
    .fetch_one(conn)
    .await
    .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(result)
}

pub async fn add_organizer(
    full_name: String,
    birthday: NaiveDate,
    phone: String,
) -> ResultWithStringError<()> {
    let conn = get_pool().await;

    sqlx::query(
        r#"
        INSERT INTO organizers (id, full_name, phone, birthday, created_at, updated_at)
        VALUES (uuid_generate_v4(), $1, $2, $3, NOW(), NOW())
    "#,
    )
    .bind(full_name)
    .bind(phone)
    .bind(birthday)
    .execute(conn)
    .await
    .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(())
}
pub async fn update_organizer(
    id: Uuid,
    full_name: String,
    birthday: NaiveDate,
    phone: String,
) -> ResultWithStringError<()> {
    let conn = get_pool().await;

    sqlx::query(r#"
            UPDATE organizers
            SET full_name = $1,
                birthday = $2,
                phone = $3,
                updated_at = $4
            WHERE id = $5
    "#)
        .bind(full_name)
        .bind(birthday)
        .bind(phone)
        .bind(Utc::now().naive_local())
        .bind(id)
        .execute(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(())
}

pub async fn delete_organizer(id: Uuid) -> ResultWithStringError<()> {
    let conn = get_pool().await;

    sqlx::query(r#"
        DELETE FROM organizers
        WHERE id = $1
    "#)
        .bind(id)
        .execute(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(())
}



use crate::database::dto::members::{ExperienceReport, RetiredReport};
use crate::database::entity::members::Members;
use crate::database::init::get_pool;
use crate::types::ResultWithStringError;
use chrono::{NaiveDate, Utc};
use uuid::Uuid;

pub async fn get_members() -> ResultWithStringError<Vec<Members>> {
    let conn = get_pool().await;

    let result = sqlx::query_as::<_, Members>(r#"
            SELECT *
            FROM members
            ORDER BY birthday
    "#)
        .fetch_all(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(result)
}

pub async fn get_member(query: String) -> ResultWithStringError<Members> {
    let conn = get_pool().await;

    let result = sqlx::query_as::<_, Members>(r#"
            SELECT * 
            FROM members 
            WHERE id::text = $1 OR full_name = $2
    "#)
        .bind(query.clone())
        .bind(query)
        .fetch_one(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(result)
}

pub async fn add_member(
    full_name: String,
    birthday: NaiveDate,
    experience: i32,
    phone: String,
    is_chairman: bool,
) -> ResultWithStringError<()> {
    let conn = get_pool().await;

    let now = Utc::now().naive_local();

    sqlx::query(r#"
            INSERT INTO members (id, full_name, birthday, experience, phone, is_chairman, created_at, updated_at)
            VALUES (uuid_generate_v4(), $1, $2, $3, $4, $5, $6, $7)
    "#)
        .bind(full_name)
        .bind(birthday)
        .bind(experience)
        .bind(phone)
        .bind(is_chairman)
        .bind(now)
        .bind(now)
        .execute(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;
    Ok(())
}

pub async fn update_member(
    id: Uuid,
    full_name: String,
    birthday: NaiveDate,
    experience: i32,
    phone: String,
    is_chairman: bool,
) -> ResultWithStringError<()> {
    let conn = get_pool().await;

    sqlx::query(r#"
            UPDATE members
            SET full_name = $1,
                birthday = $2,
                experience = $3,
                phone = $4,
                is_chairman = $5,
                updated_at = $6
            WHERE id = $7
    "#)
        .bind(full_name)
        .bind(birthday)
        .bind(experience)
        .bind(phone)
        .bind(is_chairman)
        .bind(Utc::now().naive_local())
        .bind(id)
        .execute(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(())
}

pub async fn delete_member(id: Uuid) -> ResultWithStringError<()> {
    let conn = get_pool().await;

    sqlx::query(
        r#"
        DELETE FROM members
        WHERE id = $1
    "#)
        .bind(id)
        .execute(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(())
}

pub async fn report_experienced_members() -> ResultWithStringError<Vec<ExperienceReport>> {
    let conn = get_pool().await;

    let result = sqlx::query_as::<_, ExperienceReport>(r#"
        SELECT full_name, experience
        FROM members
        WHERE experience > 7
        ORDER BY experience DESC
    "#)
        .fetch_all(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(result)
}

pub async fn get_retired_members() -> ResultWithStringError<Vec<RetiredReport>> {
    let conn = get_pool().await;

    let retirement_age = 65;
    let result = sqlx::query_as::<_, RetiredReport>(r#"
        SELECT full_name, birthday
        FROM members
        WHERE date_part('year', age(birthday)) >= $1
        ORDER BY full_name
    "#)
        .bind(retirement_age)
        .fetch_all(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(result)
}
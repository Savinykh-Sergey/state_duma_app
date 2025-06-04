use chrono::Utc;
use uuid::Uuid;
use sqlx::types::chrono::NaiveDate;

use crate::database::entity::meetings::Meetings;
use crate::database::init::get_pool;
use crate::database::dto::meetings::ReportMeetings;
use crate::types::ResultWithStringError;

pub async fn get_meetings() -> ResultWithStringError<Vec<Meetings>> {
    let conn = get_pool().await;
    let result = sqlx::query_as::<_, Meetings>(r#"
        SELECT *
        FROM meetings
        ORDER BY created_at
    "#)
        .fetch_all(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e))?;
    Ok(result)
}

pub async fn get_meeting(id: Uuid) -> ResultWithStringError<Meetings> {
    let conn = get_pool().await;
    let result = sqlx::query_as::<_, Meetings>(r#"
        SELECT *
        FROM meetings
        WHERE id = $1
    "#)
        .bind(id)
        .fetch_one(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e))?;
    Ok(result)
}

pub async fn add_meeting(
    meeting_date: NaiveDate,
    duration: i32,
    commission_id: Uuid,
) -> ResultWithStringError<()> {
    let conn = get_pool().await;
    let now = Utc::now().naive_local();
    sqlx::query(r#"
        INSERT INTO meetings (id, date, duration, commission_id, created_at, updated_at)
        VALUES (uuid_generate_v4(), $1, $2, $3, $4, $5)
    "#)
        .bind(meeting_date)
        .bind(duration)
        .bind(commission_id)
        .bind(now)
        .bind(now)
        .execute(conn)
        .await
        .map_err(|e| format!("Ошибка при добавлении заседания: {}", e))?;
    Ok(())
}

pub async fn update_meeting(
    id: Uuid,
    meeting_date: NaiveDate,
    duration: i32,
    commission_id: Uuid,
) -> ResultWithStringError<()> {
    let conn = get_pool().await;
    sqlx::query(r#"
        UPDATE meetings
        SET date = $1,
            duration = $2,
            commission_id = $3,
            updated_at = $4
        WHERE id = $5
    "#)
        .bind(meeting_date)
        .bind(duration)
        .bind(commission_id)
        .bind(Utc::now().naive_local())
        .bind(id)
        .execute(conn)
        .await
        .map_err(|e| format!("Ошибка при обновлении заседания: {}", e))?;
    Ok(())
}

pub async fn delete_meeting(id: Uuid) -> ResultWithStringError<()> {
    let conn = get_pool().await;
    sqlx::query(r#"
        DELETE FROM meetings
        WHERE id = $1
    "#)
        .bind(id)
        .execute(conn)
        .await
        .map_err(|e| format!("Ошибка при удалении заседания: {}", e))?;
    Ok(())
}

pub async fn report_meetings_by_year(year: i32) -> ResultWithStringError<Vec<ReportMeetings>> {
    let conn = get_pool().await;

    let result = sqlx::query_as::<_, ReportMeetings>(r#"
        SELECT date_part('month', date)::int AS month, COUNT(*)::integer AS count
        FROM meetings
        WHERE date_part('year', date) = $1
        GROUP BY month
        ORDER BY month
    "#)
        .bind(year)
        .fetch_all(conn)
        .await
        .map_err(|e| format!("Ошибка запроса в базу данных: {}", e.to_string()))?;

    Ok(result)
}
use sqlx::PgPool;

pub async fn update_level(
    pool: &PgPool,
    user_id: i64
) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO leveling (user_id, exp) VALUES ($1, 0)
        ON CONFLICT (user_id) DO UPDATE SET exp = leveling.exp + 1
        "#,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_level(pool: &PgPool, user_id: i64) -> anyhow::Result<i64> {
    let row = sqlx::query!(
        r#"
        SELECT exp FROM leveling WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await?;
    if let Some(row) = row {
        return Ok(row.exp)
    }
    Ok(0)
}

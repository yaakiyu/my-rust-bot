use sqlx::PgPool;

pub async fn set_level(
    pool: &PgPool,
    user_id: i64,
    exp: i64,
) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO leveling (user_id, exp) VALUES ($1, $2)
        ON CONFLICT (user_id) DO UPDATE SET exp = $2
        "#,
        user_id,
        exp
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_level(pool: &PgPool, user_id: i64) -> anyhow::Result<Option<i64>> {
    let row = sqlx::query!(
        r#"
        SELECT exp FROM leveling WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|row| row.exp))
}

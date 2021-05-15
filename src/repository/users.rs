use anyhow::*;
use async_trait::async_trait;

use crate::{
    domain::users::{NewUser, Users},
    POOL,
};

pub struct UsersRepository;

#[async_trait]
pub trait ExtUsersRepository {
    /// 注册用户
    async fn create(new_user: &NewUser, password_hash: &str) -> Result<Users>;

    /// 根据用户名查询用户
    async fn find_by_username(username: &str) -> Result<Option<Users>>;

    /// 根据邮箱查询查询用户
    async fn find_by_email(email: &str) -> Result<Option<Users>>;

    /// 根据用户名查询用户2
    async fn find_by_username2(username: &str) -> Result<Users>;

    /// 检查用户是否存在
    async fn exists_by_username(username: &str) -> Result<bool>;

    /// 检查用户是否存在
    async fn exists_by_email(email: &str) -> Result<bool>;
}

#[async_trait]
impl ExtUsersRepository for UsersRepository {
    async fn create(new_user: &NewUser, password_hash: &str) -> Result<Users> {
        let row = sqlx::query_as!(
            Users,
            //language=sql
            "INSERT INTO users(username, nickname, email, password_hash) VALUES ($1, $2, $3, $4) RETURNING *",
            &new_user.username,
            &new_user.nickname,
            &new_user.email,
            password_hash
        )
            .fetch_one(&POOL.clone())
            .await
            .context("创建用户")?;

        Ok(row)
    }

    /// 根据用户名查询用户
    async fn find_by_username(username: &str) -> Result<Option<Users>> {
        let row = sqlx::query_as!(
            Users,
            //language=sql
            "SELECT * FROM users WHERE username = $1",
            username
        )
        .fetch_optional(&POOL.clone())
        .await
        .context("查询用户")?;

        Ok(row)
    }

    async fn find_by_email(email: &str) -> Result<Option<Users>> {
        let row = sqlx::query_as!(
            Users,
            //language=sql
            "SELECT * FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&POOL.clone())
        .await
        .context("查询用户")?;

        Ok(row)
    }

    /// 根据用户名查询用户2
    async fn find_by_username2(username: &str) -> Result<Users> {
        let row = sqlx::query_as!(
            Users,
            //language=sql
            "SELECT * FROM users WHERE username = $1",
            username
        )
        .fetch_one(&POOL.clone())
        .await
        .context("根据用户名查询用户2")?;

        Ok(row)
    }

    /// 检查用户是否存在
    async fn exists_by_username(username: &str) -> Result<bool> {
        let mut conn = POOL.acquire().await?;
        let row = sqlx::query!(
            //language=sql
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)",
            username,
        )
        // .fetch_one(&POOL.clone())
        .fetch_one(&mut conn)
        .await
        .context("检查用户是否存在")?;
        let exists: Option<bool> = row.exists;
        Ok(exists.unwrap_or_default())
    }

    async fn exists_by_email(email: &str) -> Result<bool> {
        let row = sqlx::query!(
            //language=sql
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)",
            email,
        )
        .fetch_one(&POOL.clone())
        .await
        .context("检查邮箱是否存在")?;
        let exists: Option<bool> = row.exists;
        Ok(exists.unwrap_or_default())
    }
}

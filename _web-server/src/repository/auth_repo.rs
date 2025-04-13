#![allow(unused)]
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::*, Error, MySqlPool};

/// 用户基本信息表
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserRepo {
    pub id: u64,                // BIGINT UNSIGNED
    pub email: String,          // VARCHAR(100)
    pub password_hash: String,  // VARCHAR(255)
    pub mobile: Option<String>, // 可空手机号

    pub nickname: String,   // 昵称
    pub avatar_url: String, // 头像地址

    pub created_at: NaiveDateTime,            // 创建时间
    pub updated_at: NaiveDateTime,            // 更新时间
    pub last_login_at: Option<NaiveDateTime>, // 最后登录时间，可空
}

/// 第三方身份映射表
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserIdentityRepo {
    pub id: u64,                  // BIGINT UNSIGNED
    pub user_id: u64,             // 外键: users.id
    pub provider: String,         // 'github'、'wechat' 等
    pub provider_user_id: String, // 第三方用户唯一ID
    pub linked_at: NaiveDateTime, // 绑定时间
}

impl UserRepo {
    /// 根据ID查找用户
    pub async fn find_by_id(pool: &MySqlPool, user_id: u64) -> Result<Option<Self>, Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_optional(pool)
            .await
    }

    /// 插入新用户（返回插入后的完整用户）
    pub async fn insert(
        pool: &MySqlPool,
        email: &str,
        password_hash: &str,
        mobile: Option<&str>,
        nickname: &str,
        avatar_url: &str,
    ) -> Result<u64, Error> {
        let record_id = sqlx::query("INSERT INTO users (email, password_hash, mobile, nickname, avatar_url) VALUES (?, ?, ?, ?, ?)")
            .bind(email)
            .bind(password_hash)
            .bind(mobile)
            .bind(nickname)
            .bind(avatar_url)
            .execute(pool)
            .await?
            .last_insert_id();

        Ok(record_id)
    }

    /// 更新最后登录时间
    pub async fn update_last_login(pool: &MySqlPool, user_id: u64) -> Result<(), Error> {
        sqlx::query("UPDATE users SET last_login_at = NOW() WHERE id = ?")
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

impl UserIdentityRepo {
    /// 直接根据记录主键查找身份
    pub async fn find_by_id(pool: &MySqlPool, identity_id: u64) -> Result<Option<Self>, Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM user_identities WHERE id = ?")
            .bind(identity_id)
            .fetch_optional(pool)
            .await
    }

    /// 根据平台和平台用户ID查找身份
    pub async fn find_by_provider(
        pool: &MySqlPool,
        provider: &str,
        provider_user_id: &str,
    ) -> Result<Option<Self>, Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM user_identities WHERE provider = ? AND provider_user_id = ?",
        )
        .bind(provider)
        .bind(provider_user_id)
        .fetch_optional(pool)
        .await
    }

    /// 根据 user_id + provider 查找绑定的第三方身份
    pub async fn find_by_user_and_provider(
        pool: &MySqlPool,
        user_id: u64,
        provider: &str,
    ) -> Result<Option<Self>, Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM user_identities WHERE user_id = ? AND provider = ?",
        )
        .bind(user_id)
        .bind(provider)
        .fetch_optional(pool)
        .await
    }

    /// 插入新第三方身份映射
    pub async fn insert(
        pool: &MySqlPool,
        user_id: u64,
        provider: &str,
        provider_user_id: &str,
    ) -> Result<u64, Error> {
        let record_id = sqlx::query(
            "INSERT INTO user_identities (user_id, provider, provider_user_id) VALUES (?, ?, ?)",
        )
        .bind(user_id)
        .bind(provider)
        .bind(provider_user_id)
        .execute(pool)
        .await?
        .last_insert_id();

        Ok(record_id)
    }
}

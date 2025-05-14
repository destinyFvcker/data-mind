#![allow(unused)]
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::*, Error, MySqlPool};

pub const GITHUB_PROVIDER: &'static str = "github";
pub const WECHAT_PROVICER: &'static str = "wechat";

/// 用户基本信息表
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserRepo {
    pub id: u64,                              // BIGINT UNSIGNED
    pub email: String,                        // VARCHAR(100)
    pub password_hash: String,                // VARCHAR(255)
    pub mobile: Option<String>,               // 可空手机号
    pub nickname: String,                     // 昵称
    pub avatar_url: String,                   // 头像地址
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

    /// 通过邮箱密码验证该用户是否存在
    pub async fn find_by_email(
        pool: &MySqlPool,
        user_email: &str,
        password: String,
    ) -> Result<bool, Error> {
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM usersWHERE email = ? AND password_hash = ?)",
        )
        .bind(user_email)
        .bind(password)
        .fetch_one(pool)
        .await?;

        Ok(exists)
    }

    /// 根据UserIdentityRepo之中的信息联合查询用户信息
    pub async fn find_by_identity(
        pool: &MySqlPool,
        provider: &str,
        provider_user_id: &str,
    ) -> Result<Option<Self>, Error> {
        let user = sqlx::query_as::<_, Self>(
            r#"
        SELECT u.*
        FROM users u
        JOIN user_identities ui ON u.id = ui.user_id
        WHERE ui.provider = ? AND ui.provider_user_id = ?
        "#,
        )
        .bind(provider)
        .bind(provider_user_id)
        .fetch_optional(pool) // 如果你希望查不到时返回 None，可以用 fetch_optional；用 fetch_one 会报错
        .await?;

        Ok(user)
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

    /// 删除用户
    pub async fn delete(pool: &MySqlPool, user_id: u64) -> Result<u64, Error> {
        let result = sqlx::query(
            r#"
        DELETE FROM users
        WHERE id = ?
        "#,
        )
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
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

    /// 查找一个user_id关联的所有第三方账号身份
    pub async fn find_all(pool: &MySqlPool, user_id: u64) -> Result<Vec<Self>, Error> {
        let identities: Vec<Self> = sqlx::query_as::<_, Self>(
            r#"
            SELECT * FROM user_identities
            WHERE user_id = ?;
        "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(identities)
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

    /// 注销第三方身份映射
    pub async fn delete_by_id(pool: &MySqlPool, identity_id: u64) -> Result<u64, Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM user_identities
            WHERE id = ?
            "#,
        )
        .bind(identity_id)
        .execute(pool)
        .await?;

        return Ok(result.rows_affected());
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use chrono::Utc;
    use sqlx::mysql::{self, MySqlPoolOptions};

    use super::*;

    async fn init_mysql_conn() -> MySqlPool {
        MySqlPoolOptions::new()
            .acquire_timeout(Duration::from_secs(10))
            .connect("mysql://root:rootpassword@127.0.0.1:3306/data_mind")
            .await
            .unwrap()
    }

    #[actix_web::test]
    async fn insert_one_for_test() {
        let mysql_conn = init_mysql_conn().await;

        //
        let user_repo_id = UserRepo::insert(
            &mysql_conn,
            "test@example-1122.com",
            "you_know_what?",
            Some("+8613727572856"),
            "destinyFvcker",
            "test",
        )
        .await
        .unwrap();
    }

    #[actix_web::test]
    async fn test_user_repo() {
        let mysql_conn = init_mysql_conn().await;

        //
        let user_repo_id = UserRepo::insert(
            &mysql_conn,
            "test@example.com",
            "you_know_what?",
            Some("+8613727572856"),
            "destinyFvcker",
            "test",
        )
        .await
        .unwrap();

        let expect_user = UserRepo {
            id: user_repo_id,
            email: "test@example.com".to_string(),
            password_hash: "you_know_what?".to_string(),
            mobile: Some("+8613727572856".to_string()),
            nickname: "destinyFvcker".to_string(),
            avatar_url: "test".to_string(),
            created_at: Utc::now().naive_local(),
            updated_at: Utc::now().naive_local(),
            last_login_at: None,
        };

        let user_repo_re = UserRepo::find_by_id(&mysql_conn, user_repo_id)
            .await
            .unwrap()
            .ok_or("err")
            .unwrap();

        assert_eq!(user_repo_re.id, expect_user.id);
        assert_eq!(user_repo_re.email, expect_user.email);
        assert_eq!(user_repo_re.password_hash, expect_user.password_hash);
        assert_eq!(user_repo_re.mobile, expect_user.mobile);
        assert_eq!(user_repo_re.nickname, expect_user.nickname);
        assert_eq!(user_repo_re.avatar_url, expect_user.avatar_url);

        // 插入关联账号记录
        let github_id = "1231231231";
        let github_identi_id =
            UserIdentityRepo::insert(&mysql_conn, user_repo_re.id, GITHUB_PROVIDER, github_id)
                .await
                .unwrap();
        let wechat_id = "2212311222";
        let wechat_identi_id =
            UserIdentityRepo::insert(&mysql_conn, user_repo_re.id, WECHAT_PROVICER, wechat_id)
                .await
                .unwrap();

        let all_identi = UserIdentityRepo::find_all(&mysql_conn, user_repo_re.id)
            .await
            .unwrap();

        assert_eq!(2, all_identi.len());
        assert!(all_identi
            .iter()
            .find(|item| { item.provider == GITHUB_PROVIDER && item.provider_user_id == github_id })
            .is_some());
        assert!(all_identi
            .iter()
            .find(|item| { item.provider == WECHAT_PROVICER && item.provider_user_id == wechat_id })
            .is_some());

        UserRepo::delete(&mysql_conn, user_repo_id).await.unwrap();
        let all_identi = UserIdentityRepo::find_all(&mysql_conn, user_repo_re.id)
            .await
            .unwrap();
        assert!(all_identi.is_empty())
    }

    #[actix_web::test]
    async fn test_user_identity_repo() {
        let mysql_conn = init_mysql_conn().await;
        //
        let mut expect_user = UserRepo {
            id: 123123123,
            email: "test1@example.com".to_string(),
            password_hash: "you_know_what?".to_string(),
            mobile: Some("+8613727572856".to_string()),
            nickname: "destinyFvcker".to_string(),
            avatar_url: "test".to_string(),
            created_at: Utc::now().naive_local(),
            updated_at: Utc::now().naive_local(),
            last_login_at: None,
        };

        expect_user.id = UserRepo::insert(
            &mysql_conn,
            &expect_user.email,
            &expect_user.password_hash,
            None,
            &expect_user.nickname,
            &expect_user.avatar_url,
        )
        .await
        .unwrap();

        // 插入关联账号记录
        let github_id = "1231231231123";
        let github_identi_id =
            UserIdentityRepo::insert(&mysql_conn, expect_user.id, GITHUB_PROVIDER, github_id)
                .await
                .unwrap();
        let wechat_id = "2212311222123";
        let wechat_identi_id =
            UserIdentityRepo::insert(&mysql_conn, expect_user.id, WECHAT_PROVICER, wechat_id)
                .await
                .unwrap();

        UserIdentityRepo::delete_by_id(&mysql_conn, github_identi_id)
            .await
            .unwrap();

        let all_identi = UserIdentityRepo::find_all(&mysql_conn, expect_user.id)
            .await
            .unwrap();

        assert_eq!(1, all_identi.len());
        assert_eq!(all_identi[0].provider_user_id, wechat_id);

        UserRepo::delete(&mysql_conn, expect_user.id).await.unwrap();
    }
}

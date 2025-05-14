-- 用户基本信息表
CREATE TABLE IF NOT EXISTS users (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    email VARCHAR(100) UNIQUE,
    password_hash VARCHAR(255),  -- TODO 本地注册的密码哈希（第三方登录可为空）
    mobile VARCHAR(20) NULL DEFAULT NULL,
    
    nickname VARCHAR(50),
    avatar_url VARCHAR(255),

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    last_login_at DATETIME NULL DEFAULT NULL
);

-- 第三方身份映射表（OAuth）
CREATE TABLE IF NOT EXISTS user_identities (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    
    user_id BIGINT UNSIGNED NOT NULL,
    provider VARCHAR(50) NOT NULL,              -- 如: 'github', 'wechat'
    provider_user_id VARCHAR(100) NOT NULL,     -- 第三方平台的用户ID
    
    linked_at DATETIME DEFAULT CURRENT_TIMESTAMP,

    UNIQUE KEY uniq_provider_user (provider, provider_user_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- 用户钉钉相关信息集成
CREATE TABLE IF NOT EXISTS dingtalk_robots (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    user_id BIGINT UNSIGNED UNIQUE,
    webhook_address VARCHAR(255) NOT NULL,
    key_signature VARCHAR(100) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    
    FOREIGN KEY (user_id) REFERENCES users(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

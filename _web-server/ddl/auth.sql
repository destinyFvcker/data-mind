-- 创建学生表
CREATE TABLE IF NOT EXISTS Students (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    age INT,
    email VARCHAR(100),
    enrolled_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 插入几条测试数据
INSERT INTO Students (name, age, email) VALUES
('Alice', 20, 'alice@example.com'), -- test
('Bob', 22, 'bob@example.com'),
('Charlie', 19, 'charlie@example.com');
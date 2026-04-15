CREATE TABLE IF NOT EXISTS registry (
    -- 主键ID
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    -- 名称
    name TEXT NOT NULL,
    -- URL地址
    url TEXT NOT NULL,
    -- 当前使用
    is_current INTEGER NOT NULL DEFAULT 0,
    -- 是否为预设
    is_preset INTEGER NOT NULL DEFAULT 0,
    -- 创建时间：默认当前时间，不可修改
    create_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- 修改时间：默认当前时间
    update_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- 备注
    remark TEXT
);

-- 插入数据，判断名称是否重复，不重复则添加
INSERT INTO registry (name, url, is_preset) SELECT 'npm', 'https://registry.npmjs.org/', 1 WHERE NOT EXISTS (SELECT 1 FROM registry WHERE name = 'npm');
INSERT INTO registry (name, url, is_preset) SELECT 'yarn', 'https://registry.yarnpkg.com/', 1 WHERE NOT EXISTS (SELECT 1 FROM registry WHERE name = 'yarn');
INSERT INTO registry (name, url, is_preset) SELECT 'taobao', 'https://registry.npmmirror.com/', 1 WHERE NOT EXISTS (SELECT 1 FROM registry WHERE name = 'taobao');

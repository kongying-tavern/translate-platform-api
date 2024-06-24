
/* --------------- 创建表 --------------- */
-- DROP TABLE IF EXISTS sys_user;
CREATE TABLE sys_user(
    version INTEGER NOT NULL DEFAULT  1,
    create_by VARCHAR,
    create_time TIMESTAMP,
    update_by VARCHAR,
    update_time TIMESTAMP,
    del_flag BOOLEAN NOT NULL DEFAULT  false,
    id VARCHAR NOT NULL,
    username VARCHAR(32) NOT NULL DEFAULT  '',
    password VARCHAR(255) NOT NULL DEFAULT  '',
    role INTEGER NOT NULL DEFAULT  -1,
    timezone VARCHAR(32) NOT NULL DEFAULT  '',
    locale VARCHAR(32) NOT NULL DEFAULT  '',
    PRIMARY KEY (id)
);
COMMENT ON TABLE sys_user IS '用户';
COMMENT ON COLUMN sys_user.version IS '乐观锁';
COMMENT ON COLUMN sys_user.create_by IS '创建人';
COMMENT ON COLUMN sys_user.create_time IS '创建时间';
COMMENT ON COLUMN sys_user.update_by IS '更新人';
COMMENT ON COLUMN sys_user.update_time IS '更新时间';
COMMENT ON COLUMN sys_user.del_flag IS '是否删除';
COMMENT ON COLUMN sys_user.id IS 'ID';
COMMENT ON COLUMN sys_user.username IS '用户名';
COMMENT ON COLUMN sys_user.password IS '密码';
COMMENT ON COLUMN sys_user.role IS '角色';
COMMENT ON COLUMN sys_user.timezone IS '偏好时区';
COMMENT ON COLUMN sys_user.locale IS '偏好语言';

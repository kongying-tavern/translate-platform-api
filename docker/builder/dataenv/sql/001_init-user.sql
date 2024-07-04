-- 新增用户
INSERT INTO "sys_user" (
    "version",
    "create_by",
    "create_time",
    "update_by",
    "update_time",
    "del_flag",
    "username",
    "password",
    "role",
    "timezone",
    "locale"
) VALUES (
    1,
    0,
    NOW(),
    0,
    NOW(),
    FALSE,
    'admin',
    '$2a$10$mfwMS.F7RQ04qkE.2TfsL.s01k8aqvBd1aTT3t5vaLQ0cfNvcqCIq',
    0,
    'UTC+0',
    'en-US'
);

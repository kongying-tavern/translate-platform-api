use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // manager
        //     .create_table(
        //         Table::create()
        //             .table(Post::Table)
        //             .if_not_exists()
        //             .col(
        //                 ColumnDef::new(Post::Id)
        //                     .integer()
        //                     .not_null()
        //                     .auto_increment()
        //                     .primary_key(),
        //             )
        //             .col(ColumnDef::new(Post::Title).string().not_null())
        //             .col(ColumnDef::new(Post::Text).string().not_null())
        //             .to_owned(),
        //     )
        //     .await

        manager
            .create_table(
                Table::create()
                    .table(SysUser::SysUser)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysUser::Version)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(SysUser::CreateId)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(SysUser::CreateTime).timestamp())
                    .col(
                        ColumnDef::new(SysUser::UpdateId)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(SysUser::UpdateTime).timestamp())
                    .col(
                        ColumnDef::new(SysUser::DelFlag)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(SysUser::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(SysUser::Username)
                            .string()
                            .char_len(32)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(SysUser::Password)
                            .string()
                            .char_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(SysUser::Timezone)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(SysUser::Role)
                            .integer()
                            .not_null()
                            .default(-1),
                    )
                    .col(
                        ColumnDef::new(SysUser::Locale)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(SysUser::Id).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SysUser {
    /// 表名
    #[sea_orm(idle = "sys_user")]
    SysUser,
    /// 用户名
    Username,
    /// 密码
    Password,
    /// 偏好时区
    Timezone,
    /// 角色
    Role,
    /// 偏好语言
    Locale,
    /// ID为数据库自增，不会进入迭代器
    Id,
    /// 乐观锁
    Version,
    /// 创建人
    #[sea_orm(iden = "create_id")]
    CreateId,
    /// 创建时间
    #[sea_orm(iden = "create_time")]
    CreateTime,
    /// 更新人
    #[sea_orm(iden = "update_id")]
    UpdateId,
    /// 更新时间
    #[sea_orm(iden = "update_time")]
    UpdateTime,
    /// 是否删除，默认为false
    #[sea_orm(iden = "del_flag")]
    DelFlag,
}

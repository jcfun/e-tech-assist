use rbatis::{
    executor::RBatisTxExecutorGuard,
    py_sql,
    rbdc::{db::ExecResult, Error},
    Rbatis,
};

use crate::models::{
    dto::{
        base::BaseDTO,
        perm::{CreatePermDTO, QueryPermDTO, UpdatePermDTO},
    },
    vo::perm::QueryPermVO,
};

/// 新增权限信息
#[py_sql(
    r#"`insert into t_perm`
    ` (id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, name, parent_id, perm_type, disable_flag, api_path, fe_route, fe_code, resource, description)`
    ` values`
    ` (#{dto.id}, #{dto.operate_time}, #{dto.operator}, #{dto.operator_id}, #{dto.create_time}, #{dto.creator}, #{dto.creator_id}, #{dto.delete_flag}, #{dto.name}, #{dto.parent_id}, #{dto.perm_type}, #{dto.disable_flag}, #{dto.api_path}, #{dto.fe_route}, #{dto.fe_code}, #{dto.resource}, #{dto.description})`"#
)]
pub async fn create_perm(
    tx: &mut RBatisTxExecutorGuard,
    dto: &CreatePermDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 删除权限信息
#[py_sql(
    r#"`update t_perm`
    ` set`
    ` operate_time = #{dto.operate_time}`
    `, operator = #{dto.operator}`
    `, operator_id = #{dto.operator_id}`
    `, delete_flag = '1'`
    ` where id = #{dto.id}`"#
)]
pub async fn delete_perm(
    tx: &mut RBatisTxExecutorGuard,
    dto: &BaseDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 更新权限信息
#[py_sql(
    r#"`update t_perm`
    ` set`
    if dto.operate_time != '':
        ` operate_time = #{dto.operate_time}`
    if dto.operator != '':
        `, operator = #{dto.operator}`
    if dto.operator_id != '':
        `, operator_id = #{dto.operator_id}`
    if dto.name != '':
        `, name = #{dto.name}`
    `, parent_id = #{dto.parent_id}`
    if dto.perm_type != '':
        `, perm_type = #{dto.perm_type}`
    if dto.disable_flag != '':
        `, disable_flag = #{dto.disable_flag}`
    if dto.api_path != '':
        `, api_path = #{dto.api_path}`
    if dto.fe_route != '':
        `, fe_route = #{dto.fe_route}`
    if dto.fe_code != '':
        `, fe_code = #{dto.fe_code}`
    if dto.resource != '':
        `, resource = #{dto.resource}`
    if dto.description != '':
        `, description = #{dto.description}`
    ` where delete_flag = '0'` 
    ` and id = #{dto.id}`"#
)]
pub async fn update_perm(
    tx: &mut RBatisTxExecutorGuard,
    dto: &UpdatePermDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 多条件分页查询权限信息
#[py_sql(
    r#"`select id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, name, parent_id, perm_type, disable_flag, api_path, fe_route, fe_code, resource, description`
    ` from t_perm`
    ` where delete_flag = '0'` 
    if dto.create_time_start != '':
        ` and create_time >= #{dto.create_time_start}`
    if dto.create_time_end != '':
        ` and create_time <= #{dto.create_time_end}`
    if dto.name != '':
        ` and name like '%${dto.name}%'`
    if dto.perm_type != '':
        ` and perm_type = #{dto.perm_type}`
    if dto.disable_flag != '':
        ` and disable_flag = #{dto.disable_flag}`
    ` limit ${page_size}`
    ` offset ${offset}`"#
)]
pub async fn query_perms(
    tx: &mut RBatisTxExecutorGuard,
    dto: &QueryPermDTO,
    page_size: &u64,
    offset: &u64,
) -> Result<Option<Vec<QueryPermVO>>, Error> {
    impled!();
}

/// 分页查询角色信息数量
#[py_sql(
    r#"`select count(*)`
    ` from t_perm`
    ` where delete_flag = '0'` 
    if dto.create_time_start != '':
        ` and create_time >= #{dto.create_time_start}`
    if dto.create_time_end != '':
        ` and create_time <= #{dto.create_time_end}`
    if dto.name != '':
        ` and name like '%${dto.name}%'`
    if dto.perm_type != '':
        ` and perm_type = #{dto.perm_type}`
    if dto.disable_flag != '':
        ` and disable_flag = #{dto.disable_flag}`
    "#
)]
pub async fn query_perms_count(
    tx: &mut RBatisTxExecutorGuard,
    dto: &QueryPermDTO,
) -> Result<u64, Error> {
    impled!();
}

/// 更新权限状态(是否禁用)
#[py_sql(
    r#"`update t_perm`
    ` set`
    if dto.operate_time != '':
        ` operate_time = #{dto.operate_time}`
    if dto.operator != '':
        `, operator = #{dto.operator}`
    if dto.operator_id != '':
        `, operator_id = #{dto.operator_id}`
    if disable_flag != '':
        `, disable_flag = #{disable_flag}`
    ` where delete_flag = '0'` 
    ` and id = #{dto.id}`"#
)]
pub async fn update_disable_flag(
    db: &Rbatis,
    dto: &BaseDTO,
    disable_flag: &String,
) -> Result<ExecResult, Error> {
    impled!();
}

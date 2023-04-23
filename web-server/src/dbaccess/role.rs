use rbatis::{
    executor::RBatisTxExecutorGuard,
    py_sql,
    rbdc::{db::ExecResult, Error},
    Rbatis,
};

use crate::models::{
    dto::{
        base::BaseDTO,
        role::{CreateRoleDTO, QueryRoleDTO, UpdateRoleDTO},
    },
    vo::{perm::QueryPermVO, role::QueryRoleVO},
};
use crate::utils::epc;

/// 新增角色信息
#[py_sql(
    r#"`insert into t_role`
    ` (id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, name, description, disable_flag, code)`
    ` values`
    ` (#{dto.id}, #{dto.operate_time}, #{dto.operator}, #{dto.operator_id}, #{dto.create_time}, #{dto.creator}, #{dto.creator_id}, #{dto.delete_flag}, #{dto.name}, #{dto.description}, #{dto.disable_flag}, #{dto.code})`"#
)]
pub async fn create_role(
    tx: &mut RBatisTxExecutorGuard,
    dto: &CreateRoleDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 新增角色权限关联信息
#[py_sql(
    r#"`insert into t_role_perm`
    ` (id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, role_id, perm_id)`
    ` values`
    trim ',': for perm_id in dto.perm_ids: 
        `, (`
            #{epc::get_snowflake()}, #{dto.operate_time}, #{dto.operator}, #{dto.operator_id}, #{dto.create_time}, #{dto.creator}, #{dto.creator_id}, #{dto.delete_flag}, #{dto.id}, #{perm_id}
        `)`"#
)]
pub async fn create_role_perm(
    tx: &mut RBatisTxExecutorGuard,
    dto: &CreateRoleDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 删除角色信息
#[py_sql(
    r#"`update t_role`
    ` set`
    ` operate_time = #{dto.operate_time}`
    `, operator = #{dto.operator}`
    `, operator_id = #{dto.operator_id}`
    `, delete_flag = '1'`
    ` where id = #{dto.id}`"#
)]
pub async fn delete_role(
    tx: &mut RBatisTxExecutorGuard,
    dto: &BaseDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 更新角色信息
#[py_sql(
    r#"`update t_role`
    ` set`
    if dto.operate_time != '':
        ` operate_time = #{dto.operate_time}`
    if dto.operator != '':
        `, operator = #{dto.operator}`
    if dto.operator_id != '':
        `, operator_id = #{dto.operator_id}`
    if dto.name != '':
        `, name = #{dto.name}`
    if dto.description != '':
        `, description = #{dto.description}`
    if dto.disable_flag != '':
        `, disable_flag = #{dto.disable_flag}`
    if dto.code != '':
        `, code = #{dto.code}`
    ` where delete_flag = '0'` 
    ` and id = #{dto.id}`"#
)]
pub async fn update_role(
    tx: &mut RBatisTxExecutorGuard,
    dto: &UpdateRoleDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 删除角色权限关联信息
#[py_sql(
    r#"`update t_role_perm`
    ` set`
    ` operate_time = #{dto.operate_time}`
    `, operator = #{dto.operator}`
    `, operator_id = #{dto.operator_id}`
    `, delete_flag = '1'`
    ` where role_id = #{dto.id}`"#
)]
pub async fn delete_role_perm(
    tx: &mut RBatisTxExecutorGuard,
    dto: &BaseDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 多条件分页查询角色信息
#[py_sql(
    r#"`select id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, name, description, disable_flag, code`
    ` from t_role`
    ` where delete_flag = '0'` 
    if dto.create_time_start != '':
        ` and create_time >= #{dto.create_time_start}`
    if dto.create_time_end != '':
        ` and create_time <= #{dto.create_time_end}`
    if dto.name != '':
        ` and name like '%${dto.name}%'`
    if dto.disable_flag != '':
        ` and disable_flag = #{dto.disable_flag}`
    ` limit ${page_size}`
    ` offset ${offset}`"#
)]
pub async fn query_roles_fq(
    tx: &mut RBatisTxExecutorGuard,
    dto: &QueryRoleDTO,
    page_size: &u64,
    offset: &u64,
) -> Result<Option<Vec<QueryRoleVO>>, Error> {
    impled!();
}

/// 多条件分页查询角色信息数量
#[py_sql(
    r#"`select count(*)`
    ` from t_role`
    ` where delete_flag = '0'` 
    if dto.create_time_start != '':
        ` and create_time >= #{dto.create_time_start}`
    if dto.create_time_end != '':
        ` and create_time <= #{dto.create_time_end}`
    if dto.name != '':
        ` and name like '%${dto.name}%'`
    if dto.disable_flag != '':
        ` and disable_flag = #{dto.disable_flag}`
    "#
)]
pub async fn query_roles_fq_count(
    tx: &mut RBatisTxExecutorGuard,
    dto: &QueryRoleDTO,
) -> Result<u64, Error> {
    impled!();
}

/// 更新角色状态(是否禁用)
#[py_sql(
    r#"`update t_role`
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

/// 根据角色id查询关联权限信息
#[py_sql(
    r#"`select p.id, p.operate_time, p.operator, p.operator_id, p.create_time, p.creator, p.creator_id, p.delete_flag, p.name, p.parent_id, p.perm_type, p.disable_flag, p.api_path, p.fe_route, p.fe_name, p.fe_code, p.resource, hidden_flag, parent_route, p.description`
    ` from t_perm p join t_role_perm rp on p.id = rp.perm_id`
    ` where rp.delete_flag = '0'` 
    ` and p.delete_flag = '0'` 
    ` and rp.role_id = #{role_id}`
    "#
)]
pub async fn query_perms_by_role_id(
    tx: &mut RBatisTxExecutorGuard,
    role_id: &String,
) -> Result<Option<Vec<QueryPermVO>>, Error> {
    impled!();
}

/// 全量查询
#[py_sql(
    r#"`select id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, name, description, disable_flag, code`
    ` from t_role`"#
)]
pub async fn query_roles(db: &Rbatis) -> Result<Option<Vec<QueryRoleVO>>, Error> {
    impled!();
}

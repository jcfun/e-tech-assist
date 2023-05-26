use crate::models::{
    dto::{
        base::BaseDTO,
        perm::{CreatePermDTO, QueryPermDTO, UpdatePermDTO},
    },
    vo::{perm::QueryPermVO, role::Role2PermVO},
};
use rbatis::{
    executor::RBatisTxExecutorGuard,
    py_sql,
    rbdc::{db::ExecResult, Error},
    Rbatis,
};

/// 新增权限信息
#[py_sql(
    r#"`insert into t_perm`
    ` (id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, name, parent_id, perm_type, disable_flag, api_path, route, route_name, code, parent_route, resource, hidden_flag, description)`
    ` values`
    ` (#{dto.id}, #{dto.operate_time}, #{dto.operator}, #{dto.operator_id}, #{dto.create_time}, #{dto.creator}, #{dto.creator_id}, #{dto.delete_flag}, #{dto.name}, #{dto.parent_id}, #{dto.perm_type}, #{dto.disable_flag}, #{dto.api_path}, #{dto.route}, #{dto.route_name}, #{dto.code}, #{dto.parent_route}, #{dto.resource}, #{dto.hidden_flag}, #{dto.description})`"#
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
    if dto.parent_id != '':
        `, parent_id = #{dto.parent_id}`
    if dto.perm_type != '':
        `, perm_type = #{dto.perm_type}`
    if dto.disable_flag != '':
        `, disable_flag = #{dto.disable_flag}`
    if dto.api_path != '':
        `, api_path = #{dto.api_path}`
    if dto.route != '':
        `, route = #{dto.route}`
    if dto.route_name != '':
        `, route_name = #{dto.route_name}`
    if dto.code != '':
        `, code = #{dto.code}`
    if dto.parent_route != '':
        `, parent_route = #{dto.parent_route}`
    if dto.resource != '':
        `, resource = #{dto.resource}`
    if dto.hidden_flag != '':
        `, hidden_flag = #{dto.hidden_flag}`
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
    r#"`select id, to_char(operate_time, 'YYYY-MM-DD HH24:MI:SS') as operate_time, operator, operator_id, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') as create_time, creator, creator_id, delete_flag, name, parent_id, perm_type, disable_flag, api_path, route, route_name, code, resource, hidden_flag, parent_route, description`
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
    if dto.hidden_flag != '':
        ` and hidden_flag = #{dto.hidden_flag}`
    ` order by create_time desc`
    "#
)]
pub async fn query_perms_fq(
    tx: &mut RBatisTxExecutorGuard,
    dto: &QueryPermDTO,
) -> Result<Option<Vec<QueryPermVO>>, Error> {
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
    ` and (id = #{dto.id}`
    ` or parent_id = #{dto.id})` 
    "#
)]
pub async fn update_disable_flag(
    db: &Rbatis,
    dto: &BaseDTO,
    disable_flag: &String,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 全量查询
#[py_sql(
    r#"`select id, to_char(operate_time, 'YYYY-MM-DD HH24:MI:SS') as operate_time, operator, operator_id, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') as create_time, creator, creator_id, delete_flag, name, parent_id, perm_type, disable_flag, api_path, route, route_name, code, resource, hidden_flag, parent_route, description`
    ` from t_perm where delete_flag = '0'`"#
)]
pub async fn query_perms(
    tx: &mut RBatisTxExecutorGuard,
) -> Result<Option<Vec<QueryPermVO>>, Error> {
    impled!();
}

/// 根据父权限id查询子权限
#[py_sql(
    r#"`select id, to_char(operate_time, 'YYYY-MM-DD HH24:MI:SS') as operate_time, operator, operator_id, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') as create_time, creator, creator_id, delete_flag, name, parent_id, perm_type, disable_flag, api_path, route, route_name, code, resource, hidden_flag, parent_route, description`
    ` from t_perm where delete_flag = '0'` 
    ` and parent_id = #{parent_id}`"#
)]
pub async fn query_perms_by_parent_id(
    tx: &mut RBatisTxExecutorGuard,
    parent_id: &String,
) -> Result<Option<Vec<QueryPermVO>>, Error> {
    impled!();
}

/// 根据权限id查询所有角色权限关联信息
#[py_sql(
    r#"`select id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, role_id, perm_id`
    ` from t_role_perm where delete_flag = '0'` 
    ` and perm_id = #{perm_id}`
    "#
)]
pub async fn query_role_perms_by_perm_id(
    tx: &mut RBatisTxExecutorGuard,
    perm_id: &str,
) -> Result<Vec<Role2PermVO>, Error> {
    impled!();
}

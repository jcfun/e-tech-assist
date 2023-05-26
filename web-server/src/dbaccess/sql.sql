-- pub static _CREATE_FIELDS: &str = r#"(id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, )"#;
-- pub static _CREATE_VALUES: &str = r#"(#{id}, {operate_time}, {operator}, {operator_id}, {create_time}, {creator}, {creator_id}, {delete_flag}, )"#;

-- 以下是部分sql片段的解释
` SELECT`
        ` DISTINCT p.id,`
        ` p.operate_time,`
        ` p.operator,`
        ` p.operator_id,`
        ` p.create_time,`
        ` p.creator,`
        ` p.creator_id,`
        ` p.delete_flag,`
        ` p.name,`
        ` p.parent_id,`
        ` p.perm_type,`
        ` p.disable_flag,`
        ` p.api_path,`
        ` p.route,`
        ` p.route_name,`
        ` p.route_code,`
        ` p.resource,`
        ` hidden_flag,`
        ` parent_route,`
        ` p.description`
    ` FROM`
        ` t_perm p`
    ` LEFT JOIN t_role_perm rp ON`
        ` p.id = rp.perm_id`
    ` WHERE`
        -- 查出关联的权限
        ` (rp.delete_flag = '0'`
            ` AND p.delete_flag = '0'`
            ` AND rp.role_id = #{role_id})`
        -- 如果有父权限，则查出所有子权限，但是这里只能查到一级子权限
        ` OR (p.delete_flag = '0'`
            ` AND p.parent_id IN (`
            ` SELECT`
                ` perm_id`
            ` FROM`
                ` t_role_perm`
            ` WHERE`
                ` role_id = #{role_id}`
                ` AND delete_flag = '0'))`
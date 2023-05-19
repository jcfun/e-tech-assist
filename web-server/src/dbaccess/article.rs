use rbatis::{
    executor::RBatisTxExecutorGuard,
    py_sql,
    rbdc::{db::ExecResult, Error},
};

use crate::models::{
    dto::{
        article::{CreateArticleDTO, QueryArticleDTO, UpdateArticleDTO},
        base::BaseDTO,
    },
    vo::article::{QueryArticleInfoVO, QueryArticleVO},
};

/// 新增文章
#[py_sql(
    r#"`insert into t_article`
    ` (id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, title, content, cover, view_count, like_count, comment_count, collect_count, forward_count, category_id, tag_ids, status, top_flag)`
    ` values`
    ` (#{dto.id}, #{dto.operate_time}, #{dto.operator}, #{dto.operator_id}, #{dto.create_time}, #{dto.creator}, #{dto.creator_id}, #{dto.delete_flag}, #{dto.title}, #{dto.content}, #{dto.cover}, #{dto.view_count}, #{dto.like_count}, #{dto.comment_count}, #{dto.collect_count}, #{dto.forward_count}, #{dto.category_id}, #{dto.tag_ids}, #{dto.status}, #{dto.top_flag})`
    "#
)]
pub async fn create_article(
    tx: &mut RBatisTxExecutorGuard,
    dto: &CreateArticleDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 多条件查询分页查询
#[py_sql(
    r#"`select id, to_char(operate_time, 'YYYY-MM-DD HH24:MI:SS') as operate_time, operator, operator_id, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') as create_time, creator, creator_id, delete_flag, title, cover, content, view_count, like_count, comment_count, category_id, tag_ids, status, collect_count, forward_count, top_flag`
    ` from t_article`
    ` where delete_flag = '0'` 
    if dto.create_time_start != '':
        ` and create_time >= #{dto.create_time_start}`
    if dto.create_time_end != '':
        ` and create_time <= #{dto.create_time_end}`
    if dto.title != '':
        ` and title like '%${dto.title}%'`
    if dto.category_id != '':
        ` and category_id = #{dto.category_id}`
    if dto.tag_id != '':
        ` and tag_id = #{dto.tag_id}`
    if dto.status != '':
        ` and status = #{dto.status}`
    if dto.by_user_id_flag != '':
        ` and creator_id = #{dto.id}`
    if dto.top_flag != '':
        ` and top_flag = #{dto.top_flag}`
    ` order by create_time desc`
    ` limit ${page_size}`
    ` offset ${offset}`
    "#
)]
pub async fn query_articles_fq(
    tx: &mut RBatisTxExecutorGuard,
    dto: &QueryArticleDTO,
    page_size: &usize,
    offset: &usize,
) -> Result<Vec<QueryArticleVO>, Error> {
    impled!();
}

/// 多条件查询分页查询数量
#[py_sql(
    r#"`select count(*)`
    ` from t_article`
    ` where delete_flag = '0'` 
    if dto.create_time_start != '':
        ` and create_time >= #{dto.create_time_start}`
    if dto.create_time_end != '':
        ` and create_time <= #{dto.create_time_end}`
    if dto.title != '':
        ` and title like '%${dto.title}%'`
    if dto.category_id != '':
        ` and category_id = #{dto.category_id}`
    if dto.tag_id != '':
        ` and tag_id = #{dto.tag_id}`
    if dto.status != '':
        ` and status = #{dto.status}`
    if dto.by_user_id_flag != '':
        ` and creator_id = #{dto.id}`
    if dto.top_flag != '':
        ` and top_flag = #{dto.top_flag}`
    "#
)]
pub async fn query_articles_fq_count(
    tx: &mut RBatisTxExecutorGuard,
    dto: &QueryArticleDTO,
) -> Result<usize, Error> {
    impled!();
}

/// 获取置顶文章
#[py_sql(
    r#"`select id, to_char(operate_time, 'YYYY-MM-DD HH24:MI:SS') as operate_time, operator, operator_id, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') as create_time, creator, creator_id, delete_flag, title, cover, content, view_count, like_count, comment_count, category_id, tag_ids, status, collect_count, forward_count, top_flag`
    ` from t_article`
    ` where delete_flag = '0' and top_flag = '1'`
    ` order by create_time desc`
    ` limit 4`
    ` offset 0`
    "#
)]
pub async fn query_top_articles(
    tx: &mut RBatisTxExecutorGuard,
) -> Result<Vec<QueryArticleVO>, Error> {
    impled!();
}

/// 获取热门文章
#[py_sql(
    r#"`select id, to_char(operate_time, 'YYYY-MM-DD HH24:MI:SS') as operate_time, operator, operator_id, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') as create_time, creator, creator_id, delete_flag, title, cover, content, view_count, like_count, comment_count, category_id, tag_ids, status, collect_count, forward_count, top_flag`
    ` from t_article`
    ` where delete_flag = '0'`
    ` order by view_count desc, collect_count desc, like_count desc, comment_count desc, create_time desc`
    ` limit 20`
    ` offset 0`
    "#
)]
pub async fn query_hot_articles(
    tx: &mut RBatisTxExecutorGuard,
) -> Result<Vec<QueryArticleVO>, Error> {
    impled!();
}

/// 根据用户id查询用户文章数
#[py_sql(
    r#"`select count(*)`
    ` from t_article`
    ` where delete_flag = '0' and creator_id = #{id}`
    "#
)]
pub async fn query_article_count_by_user_id(
    tx: &mut RBatisTxExecutorGuard,
    id: &str,
) -> Result<usize, Error> {
    impled!();
}

/// 根据用户id查询文章投稿数据
#[py_sql(
    r#"`select count(*) total_article_count, sum(view_count) total_view_count, sum(like_count) total_like_count, sum(comment_count) total_comment_count, sum(collect_count) total_collect_count, sum(forward_count) total_forward_count`
    ` from t_article`
    ` where delete_flag = '0' and creator_id = #{id}`
    "#
)]
pub async fn query_article_info_by_user_id(
    tx: &mut RBatisTxExecutorGuard,
    id: &str,
) -> Result<QueryArticleInfoVO, Error> {
    impled!();
}

/// 更新文章
#[py_sql(
    r#"`update t_article`
    ` set`
    if dto.operate_time != '':
        ` operate_time = #{dto.operate_time}`
    if dto.operator != '':
        `, operator = #{dto.operator}`
    if dto.operator_id != '':
        `, operator_id = #{dto.operator_id}`
    if dto.title != '':
        `, title = #{dto.title}`
    if dto.content != '':
        `, content = #{dto.content}`
    if dto.cover != '':
        `, cover = #{dto.cover}`
    if dto.status != '':
        `, status = #{dto.status}`
    ` where delete_flag = '0'` 
    ` and id = #{dto.id}`
    ` and creator_id = #{dto.operator_id}`
    "#
)]
pub async fn update_article(
    tx: &mut RBatisTxExecutorGuard,
    dto: &UpdateArticleDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 删除文章
#[py_sql(
    r#"`update t_article`
    ` set`
    ` operate_time = #{dto.operate_time}`
    `, operator = #{dto.operator}`
    `, operator_id = #{dto.operator_id}`
    `, delete_flag = '1'`
    ` where id = #{dto.id}`
    if permit_flag != '1':
        ` and creator_id = #{dto.operator_id}`
    "#
)]
pub async fn delete_article(
    tx: &mut RBatisTxExecutorGuard,
    permit_flag: &str,
    dto: &BaseDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 修改文章状态(草稿、审核、发布)
#[py_sql(
    r#"`update t_article`
    ` set`
    if dto.operate_time != '':
        ` operate_time = #{dto.operate_time}`
    if dto.operator != '':
        `, operator = #{dto.operator}`
    if dto.operator_id != '':
        `, operator_id = #{dto.operator_id}`
    if status != '':
        `, status = #{status}`
    ` where delete_flag = '0'` 
    ` and id = #{dto.id}`
    if permit_flag != '1':
        ` and creator_id = #{dto.operator_id}`
    "#
)]
pub async fn update_article_status(
    tx: &mut RBatisTxExecutorGuard,
    dto: &BaseDTO,
    status: &str,
    permit_flag: &str,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 修改文章置顶状态
#[py_sql(
    r#"`update t_article`
    ` set`
    if dto.operate_time != '':
        ` operate_time = #{dto.operate_time}`
    if dto.operator != '':
        `, operator = #{dto.operator}`
    if dto.operator_id != '':
        `, operator_id = #{dto.operator_id}`
    if top_flag != '':
        `, top_flag = #{top_flag}`
    ` where delete_flag = '0'` 
    ` and id = #{dto.id}`
    if permit_flag != '1':
        ` and creator_id = #{dto.operator_id}`
    "#
)]
pub async fn update_article_top_flag(
    tx: &mut RBatisTxExecutorGuard,
    dto: &BaseDTO,
    top_flag: &str,
    permit_flag: &str,
) -> Result<ExecResult, Error> {
    impled!();
}
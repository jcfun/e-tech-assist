use rbatis::{executor::RBatisTxExecutorGuard, rbdc::{db::ExecResult, Error}, py_sql};

use crate::models::dto::article::CreateArticleDTO;

/// 新增快捷消息日志
#[py_sql(
    r#"`insert into t_article`
    ` (id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, title, content, cover, view_count, like_count, comment_count, collect_count, forward_count, category_id, tag_ids, status)`
    ` values`
    ` (#{dto.id}, #{dto.operate_time}, #{dto.operator}, #{dto.operator_id}, #{dto.create_time}, #{dto.creator}, #{dto.creator_id}, #{dto.delete_flag}, #{dto.title}, #{dto.content}, #{cover}, #{dto.view_count}, #{dto.like_count}, #{dto.comment_count}, #{dto.collect_count}, #{dto.forward_count}, #{dto.category_id}, #{dto.tag_ids}, #{dto.status})`
    "#
)]
pub async fn create_quick_msg(
    tx: &mut RBatisTxExecutorGuard,
    dto: &CreateArticleDTO,
) -> Result<ExecResult, Error> {
    impled!();
}
use axum::extract::Multipart;
use tracing::info;
use uuid::Uuid;

use crate::{
    common::{errors::MyError, res::Res},
    config::init::get_cfg,
    models::vo::upload::UploadVO,
    utils::jwt::Claims,
};

pub async fn image_upload(
    claims: Claims,
    mut multipart: Multipart,
) -> Result<Res<UploadVO>, MyError> {
    let image_types = &get_cfg().mime.image.types;
    let mut upload_vo = UploadVO::default();
    while let Some(field) = multipart.next_field().await? {
        let content_type = field.content_type().unwrap().to_string();
        if !image_types.contains(&content_type) {
            return Ok(Res::from_fail_msg(&format!(
                "文件类型错误，只支持 {} 类型的文件",
                image_types.join(" ")
            )));
        }
        let file_name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        let file_size = data.len();
        info!("成功接收类型为({content_type})的图片({file_name}), 大小({file_size})个字节");
        //生成随机文件名
        let uuid = Uuid::new_v4().to_string();
        let user_id = claims.id.as_ref().unwrap();
        let current_date = chrono::Local::now().date_naive();
        let image_name = format!("{}_{}_{}", user_id, current_date, uuid);
        //提取"/"的index位置
        let index = content_type
            .find("/")
            .map(|i| i)
            .unwrap_or(usize::max_value());
        //文件扩展名
        let ext_name = if index != usize::max_value() {
            &content_type[index + 1..]
        } else {
            "png"
        };
        //最终保存在服务器上的文件名
        let save_image_name = format!("{}/{}.{}", "assets/images", image_name, ext_name);
        //保存上传的文件
        tokio::fs::write(&save_image_name, &data).await?;
        info!("成功保存类型为({ext_name})的图片至({save_image_name}), 大小({file_size})个字节");
        // let ip = &get_cfg().server.ip;
        let port = &get_cfg().server.port;
        let prefix = &get_cfg().api.prefix;
        let version = &get_cfg().api.version;
        upload_vo.url =
            format!("http://localhost:{port}/{prefix}/{version}/{save_image_name}").into();
        upload_vo.alt = file_name.into();
        upload_vo.href =
            format!("http://localhost:{port}/{prefix}/{version}/{save_image_name}").into();
    }
    Ok(Res::from_success("上传成功", upload_vo))
}

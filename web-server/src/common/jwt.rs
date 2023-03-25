use serde::{Serialize, Deserialize};

/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-25 19:44:00
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-03-25 21:19:41
 * @FilePath: /e-tech-assist/web-server/src/common/jwt.rs
 * @Description: 
 */
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: String,
    username: String,
}
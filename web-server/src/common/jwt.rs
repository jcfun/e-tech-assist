/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-25 19:44:00
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-25 20:19:27
 * @FilePath: /e-tech-assist/web-server/src/common/jwt.rs
 * @Description: 
 */
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: String,
    company: String,
}
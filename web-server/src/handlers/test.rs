use crate::utils::epc::get_snowflake;

pub async fn test() {
    let id = get_snowflake();
    println!("id ====> {}", id)
}

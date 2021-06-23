use quicli::prelude::*;
use async_std::fs::File;

/// 字符串首字母大写
pub async fn first_char_to_uppercase(params: String) -> async_std::io::Result<String>{
    let mut v: Vec<char> = params.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    let res = v.into_iter().collect();
    Ok(res)
}

pub async fn file_content(filepath: &String) -> async_std::io::Result<String> {
    let s = read_file(filepath);
    let s = match s {
        Ok(d)=> {
            d
        },
        Err(_e) => {
            File::create(filepath).await.with_context(|_| format!("Could not create/open file {:?}", &filepath)).unwrap();
            String::new()
        }
    };
    Ok(s)
}
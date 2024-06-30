//! 这是一个转换工具, 用于将 Rust struct 转换为 antd 的 Table ColumnsType 和 Interface 的定义
//! 例如有一个rust struct:
//! ``` no run
//! #[derive(Debug, Serialize, Clone)]
//! pub struct MenuList {
//!     pub id: i32,
//!     pub menu_name: String,
//! }
//! ```
//! 转换成antd:
//! ``` typescript
//! export interface MenuList {
//!     id: number;
//!     menu_name: string;
//! }
//!
//! const columns: ColumnsType<MenuList> = [
//!     {
//!       title: 'id',
//!       dataIndex: 'id',
//!     },
//!     {
//!       title: 'menu_name',
//!       dataIndex: 'menu_name',
//!     },
//!   ]
//! ```

use tran::impl_tran;

mod tran;

fn main() {
    // 获取参数文件路径,如果没有参数则遍历当前目录的所有.rs文件
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        tran_directory(".");
    } else {
        for file in args.iter().skip(1) {
            impl_tran(file);
        }
    }
    // impl_tran("/Users/anger/Documents/rust/jo/axum-casbin-admin/src/vo/menu_vo.rs");
}

// 遍历目录的所有.rs文件，并转换
fn tran_directory(path: &str) {
    let mut files = std::fs::read_dir(path).unwrap();
    while let Some(Ok(file)) = files.next() {
        let file_path = file.path();
        if file_path.is_dir() {
            tran_directory(&file_path.to_str().unwrap());
        } else if file_path.is_file() {
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(".rs") {
                impl_tran(format!("{}/{}", path, file_name));
            }
        }
    }
}

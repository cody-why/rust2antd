 # rust2antd
 ### 这是一个转换工具, 用于将 Rust struct 转换为 antd 的 Table ColumnsType 和 Interface 的定义
 
 例如有一个rust struct:
 ``` rust
 #[derive(Debug, Serialize, Clone)]
 pub struct MenuList {
     pub id: i32,
     pub menu_name: String,
 }
```
 转换成antd:
 ``` typescript
 export interface MenuList {
     id: number;
     menu_name: string;
 }

 const columns: ColumnsType<MenuList> = [
     {
       title: t('id'),
       dataIndex: 'id',
     },
     {
       title: t('menu_name'),
       dataIndex: 'menu_name',
     },
   ]
 ```

## 使用方法
 安装 rust2antd:
 ```
 cargo install rust2antd
 ```
 然后在 rust 项目的struct的目录下运行:

 ```
 rust2antd 
 ```
 或者指定输入文件:
 ```
 rust2antd <input_file>
 ```
 其中 `<input_file>` 是 rust struct 的路径, 例如 `src/vo/vo.rs`

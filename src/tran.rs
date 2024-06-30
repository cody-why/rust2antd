pub fn impl_tran(path: impl AsRef<str>) {
    // 1. 读取rust struct 文件
    println!("file: {}", path.as_ref());
    let file = std::fs::read_to_string(path.as_ref()).unwrap();
    let ast = syn::parse_file(&file).unwrap();

    let structs = ast
        .items
        .iter()
        .filter(|item| match item {
            syn::Item::Struct(_s) => true,
            _ => false,
        })
        .collect::<Vec<_>>();

    let mut fields: Vec<(String, String)> = vec![];
    for item in structs {
        fields.push(expan_one(item));
    }

    if fields.len() == 0 {
        return;
    }
    
    println!("// ---------------- interface -----------------\n");
    // 输出data interface
    for (interface, _) in &fields {
        println!("{}\n", interface);
    }

    println!("// ---------------- ColumnsType -----------------\n");
    // 输出antd的Table ColumnsType
    for (_, columns) in &fields {
        println!("{}\n", columns);
    }
}

/// 展开一个struct, 返回接口定义和ColumnsType
fn expan_one(item: &syn::Item) -> (String, String) {
    // 解析struct定义, 得到字段名和类型
    let struct_name = match item {
        syn::Item::Struct(s) => s.ident.to_string(),
        _ => panic!("not found struct"),
    };

    let fields = match item {
        syn::Item::Struct(s) => s.fields.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap().to_string();
            let field_type = match &field.ty {
                syn::Type::Path(p) => {
                    let segment = &p.path.segments[0];
                    if segment.ident == "Option" {
                        if let syn::PathArguments::AngleBracketed(angle_bracketed_args) = &segment.arguments {
                            if let Some(syn::GenericArgument::Type(ty)) = angle_bracketed_args.args.first() {
                                // 这里的 ty 就是 Option 包裹的类型
                                return (field_name, quote::quote! { #ty }.to_string());
                            }
                        }
                    }
                    if segment.ident == "Vec" {
                        if let syn::PathArguments::AngleBracketed(angle_bracketed_args) = &segment.arguments {
                            if let Some(syn::GenericArgument::Type(ty)) = angle_bracketed_args.args.first() {
                                // 这里的 ty 就是 Vec 包裹的类型
                                return (field_name, quote::quote! { #ty[] }.to_string());
                            }
                        }
                    }
                    segment.ident.to_string()
                },
                _ => panic!("Not a path type"),
            };

            (field_name, field_type)
        }),
        _ => panic!("not found struct"),
    }
    .collect::<Vec<_>>();

    if fields.is_empty() {
        return Default::default();
    }

    // 输出接口定义
    let mut column_type = String::new();
    for (field_name, field_type) in &fields {
        column_type.push_str(&format!("  {}: {},\n", field_name, convert_type(field_type)));
    }
    let interface = format!("export interface {} {{\n{}}}", struct_name, column_type);

    //  输出antd的Table ColumnsType
    let mut column_type = String::new();
    for (field_name, _) in &fields {
        column_type.push_str(&format!("  {{ title: t('{}'), dataIndex: '{}' }},\n", field_name, field_name));
    }
    let columns = format!("const {}: ColumnsType<{}> = [\n{}];", "columns", struct_name, column_type);

    // 输出json
    // let mut json_str = String::new();
    // json_str.push_str("{\n");
    // for (field_name, _) in &fields {
    //     json_str.push_str(&format!(r#"  "{}": "",{}"#, field_name, "\n"));
    // }
    // json_str.push('}');
    // println!("const {}_JSON: string = '{}';", struct_name, json_str);

    (interface, columns)
}

// 把数字类型转换成number类型,把其他类型转换成string类型
fn convert_type(field_type: &str) -> String {
    let field_type = field_type.trim();

    if field_type.ends_with("[]") {
        let s = &field_type[0..field_type.len() - 2];
        return format!("{}[]", convert_type(s));
    }

    match field_type {
        "i32" | "i64" | "u32" | "u64" | "f32" | "f64" => "number".to_string(),
        "bool" => "boolean".to_string(),
        _ => "string".to_string(),
    }
}

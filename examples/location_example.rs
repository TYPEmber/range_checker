fn main() {
    let loc = Location {
        latitude: -100.0,
        ..Default::default()
    };

    dbg!(loc.check());
}

use range_checker::Check;

#[derive(Debug, Default, Check)]
pub struct Location {
    // 纬度 [-90.0, 90.0]
    #[range(-90.0..=90.0)]
    pub latitude: f64,
    // 经度 [-180.0, 180.0]
    #[range(-180.0..=180.0)]
    pub longitude: f64,
    // 海拔 [0.0, 10000.0]
    pub altitude: f64,
}

// #[proc_macro_derive(Check0, attributes(range))]
// pub fn derive_range_checker0(input: TokenStream) -> TokenStream {
//     // 使用 syn 将 TokenStream 解析为结构化的语法树
//     let input = parse_macro_input!(input);
//     // 取出 ident 后续备用
//     // 如果我们为 struct Location 派生我们的 Check 宏的话
//     // 此处的 ident 就是 Location
//     let DeriveInput { ident, .. } = input;

//     // 用于后续缓存待检查的条目
//     let mut check_list = vec![];

//     // part 0: 数据准备
//     // 此处声明我们的 Check 派生宏只对结构体做处理
//     if let syn::Data::Struct(syn::DataStruct { fields, .. }) = input.data {
//         // 取出结构体中每一项进行处理
//         for field in fields {
//             // 获取 field 的 ident 以备后用
//             let ident_item = &field.ident.unwrap();

//             let mut ranges = field
//                 .attrs
//                 // 遍历所有的附加属性
//                 .iter()
//                 .map(|attr| (attr, attr.path.get_ident()))
//                 .filter(|(_, ident)| ident.is_some())
//                 .map(|(attr, ident)| (attr, ident.unwrap()))
//                 // 筛选出所有属性名为 range 的属性
//                 .filter(|(_, ident)| (*ident).eq("range"))
//                 // 将其解析为 Range 表达式
//                 .map(|(attr, _)| attr.parse_args::<syn::ExprRange>())
//                 // 返回所有解析成功的 Range 表达式
//                 .filter_map(|attr| attr.ok());

//             // 我们实际上可以利用多个 Range 进行组合，但此处我们只考虑至多只有一个 Range 的情况
//             // 若没有 Range 我们则不对这个 field 做任何额外操作
//             if let Some(range) = ranges.next() {
//                 // 构造判断语句并将其转换为 TokenStream 并缓存到一个 check_list 中
//                 // 这里等效于 (-90.0..=90.0).contains(&self.latitude)
//                 let check_statement = quote! {(#range).contains(&self.#ident_item)};
//                 check_list.push(check_statement);
//             }
//         }
//     }

//     // part 1: impl struct
//     // into() 调用是由于 syn 和 quote 直接调用的是 proc-macro2 库中的相应类型
//     // proc-macro2 只是单纯的对 proc-macro 标准库的包装
//     // 目的是为了绕开 proc-macro 中的类型只能在标注了 proc-macro = true 的 crate 中使用的限制
//     quote!(
//         // #ident 是 quote 定义的捕获派生宏函数内局部变量的语法
//         // 即此处等效于手动实现了 impl Location
//         // 另外，更合理的做法应该是定义一个与宏同名的 trait 再 impl
//         // 例如：impl Check for #ident
//         // 这样可以避免函数重名，不过此处作为基础例子就先省略这一步了
//         impl #ident {
//             fn check(&self) -> Result<(), ()> {
//                 #(
//                     if !(#check_list) {
//                         return Err(());
//                     }
//                 )*
//                 Ok(())
//             }
//         }
//     )
//     .into()
// }
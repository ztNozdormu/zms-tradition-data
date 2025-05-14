// macro_rules! impl_table_record {
//     ($ty:ty, $variant:ident, $table:expr) => {
//         impl crate::db::types::TableRecord for $ty {
//             const TABLE_NAME: &'static str = $table;
//
//             fn to_enum_inserter<'a>(
//                 inserter: &'a crate::db::types::AnyInserter,
//             ) -> Option<&'a tokio::sync::RwLock<clickhouse::inserter::Inserter<Self>>> {
//                 match inserter {
//                     crate::db::types::AnyInserter::$variant(ins) => Some(ins),
//                     _ => None,
//                 }
//             }
//         }
//     };
// }

/// 自动实现 TableRecord，并支持可选 use_buffer
macro_rules! impl_table_record {
    // 默认不启用 buffer 的简写形式
    ($ty:ty, $variant:ident, $table:expr) => {
        impl_table_record!($ty, $variant, $table, false);
    };

    // 完整形式：可指定 use_buffer
    ($ty:ty, $variant:ident, $table:expr, $use_buffer:expr) => {
        impl crate::db::types::TableRecord for $ty {
            const TABLE_NAME: &'static str = $table;

            fn to_enum_inserter<'a>(
                inserter: &'a crate::db::types::AnyInserter,
            ) -> Option<&'a tokio::sync::RwLock<clickhouse::inserter::Inserter<Self>>> {
                match inserter {
                    crate::db::types::AnyInserter::$variant(ins) => Some(ins),
                    _ => None,
                }
            }

            fn use_buffer() -> bool {
                $use_buffer
            }
        }
    };
}


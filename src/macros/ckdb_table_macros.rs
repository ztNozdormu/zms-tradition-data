#[macro_export]
macro_rules! impl_table_record {
    ($ty:ty, $variant:ident, $table:expr) => {
        impl crate::infra::db::types::TableRecord for $ty {
            const TABLE_NAME: &'static str = $table;

            fn get_enum_inserter<'a>(
                inserter: &'a crate::infra::db::types::AnyInserter,
            ) -> Option<&'a tokio::sync::RwLock<clickhouse::inserter::Inserter<Self>>> {
                match inserter {
                    crate::infra::db::types::AnyInserter::$variant(ins) => Some(ins),
                    _ => None,
                }
            }
        }
    };
}

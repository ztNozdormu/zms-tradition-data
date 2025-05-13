macro_rules! impl_table_record {
    ($ty:ty, $variant:ident, $table:expr) => {
        impl crate::db::ckdb::types::TableRecord for $ty {
            const TABLE_NAME: &'static str = $table;

            fn to_enum_inserter<'a>(
                inserter: &'a crate::db::ckdb::types::AnyInserter,
            ) -> Option<&'a tokio::sync::RwLock<clickhouse::inserter::Inserter<Self>>> {
                match inserter {
                    crate::db::ckdb::types::AnyInserter::$variant(ins) => Some(ins),
                    _ => None,
                }
            }
        }
    };
}

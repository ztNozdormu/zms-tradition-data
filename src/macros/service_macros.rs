#[macro_export]
macro_rules! impl_full_service {
    (
        $service:ident,             // eg: CoinRankInfoService
        $repo:ident,               // eg: CoinRankInfoRepository
        $model:ty,                 // eg: CoinRankInfo
        $new_model:ty,             // eg: NewCoinRankInfo
        $update_model:ty           // eg: UpdateCoinRankInfo
    ) => {
        pub struct $service<'a> {
            pub(crate) repo: $repo<'a>,
        }

        impl<'a> $service<'a> {
            pub fn new(conn: &'a mut MysqlConnection) -> Self {
                Self {
                    repo: $repo::new(conn),
                }
            }

            pub fn fetch_all(&mut self) -> AppResult<Vec<$model>> {
                self.repo.get_all()
            }

            pub fn get_by_id(&mut self, id: &str) -> AppResult<Option<$model>> {
                self.repo.get_by_id(id)
            }

            pub fn insert(&mut self, entity: &$new_model) -> AppResult<usize> {
                self.repo.insert(entity)
            }

            pub fn update(&mut self, entity: &$update_model) -> AppResult<usize> {
                self.repo.update(entity)
            }

            pub fn delete(&mut self, id: &str) -> AppResult<usize> {
                self.repo.delete(id)
            }

            // ⬇️ 可以在外部添加额外业务方法，例如 save_coin_rank_info
        }

        impl<'a> std::fmt::Debug for $service<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($service))
                    .field("repo", &"<redacted>")
                    .finish()
            }
        }
    };
}

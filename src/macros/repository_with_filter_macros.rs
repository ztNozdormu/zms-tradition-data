#[macro_export]
macro_rules! impl_repository_with_filter {
    (
        $repo:ident,
        $table:ident,
        $entity:ty,
        $filter:ty,
        @filter_var = $filter_var:ident,
        { $($body:tt)* }
        $(, composite_pk = [$($pk:ident),+])?
    ) => {
        impl<'a> crate::domain::repository::FilterableRepository<$filter, $entity> for $repo<'a> {
            fn filter_paginated(
                &mut self,
                $filter_var: &$filter,
                page: i64,
                per_page: i64,
            ) -> AppResult<Vec<$entity>> {
                use diesel::prelude::*;
                use diesel::dsl::*;

                let q = {
                            $($body)*
                          };

                q.limit(per_page)
                    .offset(page * per_page)
                    .load::<$entity>(self.conn)
                    .map_err(Into::into)
            }

            fn count_filtered(&mut self, $filter_var: &$filter) -> AppResult<i64> {
                use diesel::prelude::*;
                use diesel::dsl::*;

               let q = {
                            $($body)*
                          };
                q.count()
                    .get_result::<i64>(self.conn)
                    .map_err(Into::into)
            }
        }

        $(
            impl<'a> $repo<'a> {
                pub fn get_by_pk(&mut self, $($pk: &str),+) -> AppResult<Option<$entity>> {
                    use diesel::prelude::*;
                    use crate::schema::$table::dsl::*;

                    $table
                        $(.filter($pk.eq($pk)))+
                        .select(<$entity>::as_select())
                        .first(self.conn)
                        .optional()
                        .map_err(AppError::from)
                }

                pub fn delete_by_pk(&mut self, $($pk: &str),+) -> AppResult<usize> {
                    use diesel::prelude::*;
                    use crate::schema::$table::dsl::*;

                    diesel::delete(
                        $table $(.filter($pk.eq($pk)))+
                    ).execute(self.conn)
                     .map_err(AppError::from)
                }
            }
        )?
    };
}

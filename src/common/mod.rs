pub mod log_utils;
pub mod serde_fun;
pub(crate) mod utils;

/// 通用批量转换 trait，支持将 `Vec<T>` 转换为 `Vec<U>`，前提是 `U: From<T>`
pub trait VecConvert<T, U> {
    fn convert_vec(self) -> Vec<U>;
}

impl<T, U> VecConvert<T, U> for Vec<T>
where
    U: From<T>,
{
    fn convert_vec(self) -> Vec<U> {
        self.into_iter().map(U::from).collect()
    }
}

/// 支持引用类型的批量转换
/// 如果你也想支持 &[T] → Vec<U>（例如你不想移动原始数据）

impl<'a, T, U> VecConvert<&'a T, U> for &'a [T]
where
    U: From<&'a T>,
{
    fn convert_vec(self) -> Vec<U> {
        self.iter().map(U::from).collect()
    }
}

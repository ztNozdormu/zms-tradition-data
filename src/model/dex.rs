use crate::impl_table_record;
use crate::model::dex::price::PriceUpdate;

pub mod price;

impl_table_record!(PriceUpdate, PriceUpdate, "price_updates");

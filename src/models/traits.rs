use crate::{
    core::meta::{
        MarketDataNode, MetaDiscountFactor, MetaExchangeRate, MetaForwardRate, MetaMarketDataNode,
    },
    time::date::Date,
};

pub trait Model {
    fn gen_fwd_data(&self, fwd: MetaForwardRate, eval_date: Date) -> f64;
    fn gen_df_data(&self, df: MetaDiscountFactor, eval_date: Date) -> f64;
    fn gen_fx_data(&self, fx: MetaExchangeRate, eval_date: Date) -> f64;

    fn gen_node(&self, eval_date: Date, meta_data: &MetaMarketDataNode) -> MarketDataNode {
        let id = meta_data.id();
        let df = match meta_data.df() {
            Some(df) => Some(self.gen_df_data(df, eval_date)),
            None => None,
        };
        let fwd = match meta_data.fwd() {
            Some(fwd) => Some(self.gen_fwd_data(fwd, eval_date)),
            None => None,
        };
        let fx = match meta_data.fx() {
            Some(fx) => Some(self.gen_fx_data(fx, eval_date)),
            None => None,
        };
        return MarketDataNode::new(id, df, fwd, fx);
    }
}

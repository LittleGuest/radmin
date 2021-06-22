#![allow(unused)]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use syn::{Data, Fields};

#[proc_macro_derive(RBatisModel)]
pub fn rbatis_model_derive(ts: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(ts).unwrap();
    let name = &ast.ident;

    let (im, ty, wh) = ast.generics.split_for_impl();
    let gen = quote! {
        use async_trait::async_trait;

        #[async_trait]
        impl RBatisModel for #name {
            async fn page(pr: PageRequest) -> Result<Page<Self>> {
                let pages: Page<Self> = RB.fetch_page_by_wrapper("", &RB.new_wrapper(), &pr).await?;
                Ok(pages)
            }

            async fn list() -> Result<Vec<Self>> {
                let list: Vec<Self> = RB.list("").await?;
                Ok(list)
            }

            async fn get_one(id: i64) -> Result<Option<Self>> {
                let mut wr = RB.new_wrapper();
                wr.eq("id", id);
                wr.check()?;

                let model = RB.fetch_by_wrapper("", &wr).await?;
                Ok(model)
            }

            async fn save(model: Self) -> Result<Option<i64>> {
                let dbe_result = RB.save("", &model).await?;
                Ok(dbe_result.last_insert_id)
            }

            async fn save_batch(models: &[Self]) -> Result<u64> {
                let dbe_result = RB.save_batch("", &models).await?;
                Ok(dbe_result.rows_affected)
            }

            async fn update(model: Self) -> Result<u64> {
                let affected_rows = RB.update_by_id("", &model).await?;
                Ok(affected_rows)
            }

            async fn remove_by_id(id: i64) -> Result<u64> {
                let mut wr = RB.new_wrapper();
                wr.eq("id", id);
                wr.check()?;

                let affected_rows = RB.remove_by_wrapper::<Self>("", &wr).await?;
                Ok(affected_rows)
            }

            async fn remove_batch_by_ids(ids: &[i64]) -> Result<u64> {
               let mut wr = RB.new_wrapper();
                wr.r#in("id", ids);
                wr.check()?;

                let affected_rows = RB.remove_by_wrapper::<Self>("", &wr).await?;
                Ok(affected_rows)
            }
        }
    };
    gen.into()
}

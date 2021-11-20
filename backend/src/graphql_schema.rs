struct Query;

#[Object(extends)]
impl Query {
    async fn users<'a>(&self, ctx: &'a Context<'_>) -> FieldResult<Vec<Coder>> {
        let pool = ctx.data::<r2d2::Pool>().unwrap();
        let rows = Coder::read_all(&pool).await?;
        Ok(rows)
    }
} 

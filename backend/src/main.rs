use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, EmptySubscription, FieldResult, Object, Request, Response, Schema,
};
use models::User;
use poem::{
    get, handler,
    http::Method,
    listener::TcpListener,
    middleware::Cors,
    web::{Data, Html, Json},
    EndpointExt, IntoResponse, Route, Server,
};
use sqlx::sqlite::SqlitePool;
use ulid::Ulid;

pub mod models;

struct Query;

#[Object(extends)]
impl Query {
    async fn users<'a>(&self, context: &'a Context<'_>) -> FieldResult<Vec<User>> {
        let pool = context.data::<SqlitePool>().unwrap();
        let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY inserted_at DESC")
            .fetch_all(pool)
            .await?;
        Ok(users.to_vec())
    }
}

struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn create_user(
        &self,
        context: &Context<'_>,
        name: String,
        email: Option<String>,
    ) -> FieldResult<User> {
        let ulid = Ulid::new().to_string();
        let pool = context.data::<SqlitePool>().unwrap();
        let inserted = sqlx::query_as::<_, User>(
            "
            INSERT INTO users (id, name, email)
            VALUES (?, ?, ?)
            RETURNING *
        ",
        )
        .bind(&ulid)
        .bind(name)
        .bind(email)
        .fetch_one(pool)
        .await?;
        Ok(inserted)
    }
}

type BankingSchema = Schema<Query, Mutation, EmptySubscription>;

#[handler]
async fn graphql_handler(schema: Data<&BankingSchema>, req: Json<Request>) -> Json<Response> {
    Json(schema.execute(req.0).await)
}

#[handler]
fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let pool =
        SqlitePool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL is not set"))
            .await
            .expect("failed to get a db connection");

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(pool)
        .finish();
    let cors =
        Cors::new()
            .allow_origin("*")
            .allow_methods([Method::POST, Method::GET, Method::OPTIONS]);

    let app = Route::new()
        .at(
            "/graphql",
            get(graphql_playground)
                .options(graphql_handler)
                .post(graphql_handler),
        )
        .data(schema)
        .with(cors);

    println!("Playground: http://localhost:5050");

    let listener = TcpListener::bind("127.0.0.1:5050");
    let server = Server::new(listener).await?;
    server.run(app).await
}

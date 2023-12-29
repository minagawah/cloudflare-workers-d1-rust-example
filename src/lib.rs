use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    code: String,
    created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    name: String,
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_log!(
        "{} {}, located at: {:?}, within: {}",
        req.method().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );

    Router::new()
        .get_async("/", |_, _ctx| async move {
            // A greeting when accessing the root.
            Response::ok("hello")
        })
        .get_async("/users", |_, ctx| async move {
            let d1 = ctx.env.d1("DB_PROD_YENISEYSK")?;
            let statement = d1.prepare("SELECT * FROM users");
            let result = statement.all().await?;
            Response::from_json(&result.results::<User>().unwrap())
        })
        .get_async("/users/:id", |_, ctx| async move {
            let id = ctx.param("id").unwrap();
            let d1 = ctx.env.d1("DB_PROD_YENISEYSK")?;
            let statement = d1.prepare("SELECT * FROM users WHERE id = ?1");
            let query = statement.bind(&[id.into()])?;
            let result = query.first::<User>(None).await?;
            match result {
                Some(user) => Response::from_json(&user),
                None => Response::error("Not found", 404),
            }
        })
        .post_async("/user", |mut req, ctx| async move {
            let payload = req.json::<Payload>().await?;
            let d1 = ctx.env.d1("DB_PROD_YENISEYSK")?;
            let statement =
                d1.prepare("INSERT INTO users (name, code, created_at) VALUES (?1, ?2, ?3)");

            let now = chrono::offset::Utc::now().to_rfc2822();

            let code: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(64)
                .map(char::from)
                .collect();

            let query = statement.bind(&[
                payload.name.clone().into(),
                code.clone().into(),
                now.clone().into(),
            ])?;

            let result = query.run().await?;
            console_log!("result: {:?}", result.success());

            Response::ok(format!("Successfully added: {}", payload.name.clone(),))
        })
        .run(req, env)
        .await
}

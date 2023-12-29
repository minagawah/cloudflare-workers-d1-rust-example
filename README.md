# cloudflare-workers-d1-rust-example

## 1. About

A sample code for Cloudflare Workers using D1 database (and is written in Rust for WASM).

## 2. What Did I Do?

Jobs needed when creating this sample project.

### 2-1. Cloudflare Worker

If you have not yet signed up for Cloudflare, you should go ahead.  
Once you you have your account, you will see *"Workers & Pages"* in the menu,  
and that's where you are creating your workers.

**!!!! Pro Tips !!!!**  
When you first visit *"Workers & Pages"*, you may want to create a worker... Why?  
Because **that's the only way you can pick the sub-domain** for your workers.  
Initially, there is no link to name your sub-domain.
Only way you can pick your sub-domain is to create an arbitrary worker first.
Once you create a temporary worker, you will see a link where you can change your sub-domain.  
You will thank me later for letting you know of this.

### 2-2. Wrangler

Firstly, you need to install Wrangler globally:

```shell
npm install wrangler -g
```

Suppose that you already have a Cloudflare account, you want to grant Wrangler a privilege for accessing your account:

```shell
wrangler login
```

### 2-3. Creating Project

There is [an official page in Cloudflare for setting up a Cloudflare Worker for Rust](https://developers.cloudflare.com/workers/runtime-apis/webassembly/rust/). For this project, I simply follow the steps presented in the page. Make sure that you have `wasm32-unknown-unknown` toolchain so that you have your Rust build target for WASM.

Here is how you create a project:

```shell
npx wrangler generate \
  {YOUR_PROJECT_NAME} \
  https://github.com/cloudflare/workers-sdk/templates/experimental/worker-rust
```

Once the project is generated, here are some additional files I added:

```diff
Cargo.toml
+ .git
+ .gitignore
+ LICENSE.md
+ LICENSE.MIT
+ LICENSE.UNLICENSE
package.json
README.md
+ schema.sql
src
wrangler.toml
```

Since Wrangler v2 does not support some of the D1 query syntax, you need to update Wrangler to v3:

`package.json`
```diff
  "devDependencies": {
-     "wrangler": "^2.13.0"
+     "wrangler": "^3.22.1"
  }
```

`worker@0.0.15` does not provide `d1` feature, and you need an update:

`Cargo.toml`
```diff
- worker = "0.0.15"
+ worker = { version = "0.0.18", features = ["d1"] }
```

I also have other crates installed for some fancy features I have:

`Cargo.toml`
```diff
+ rand = "0.8.5"
+ getrandom = { version = "0.2.11", features = ["js"] }
+ serde_json = "1.0.108"
+ serde = "1.0.193"
+ chrono = "0.4.31"
```

### 2-4. Setting Up D1 Database

They have [an official instruction page for D1 setup](https://developers.cloudflare.com/d1/get-started/), and I simply followed the page.  
Before creating a database, make sure that you do `wrangler login`.

We want to create a new database `prod-yeniseysk` first.

**IMPORTANT:**  
When you create a database using Wrangler, you will see the database in Cloudflare Worker dashboard. It will create the database in PROD, however, you don't have any tables there yet.  
Also, when you run Wrangler to create table(s) (as shown in the following steps), **you are first dealing with the table(s) locally.** Even if you create the table, you don't see it in your Cloudflare dashboard yet. For the actual creation of the table(s), you need to *manually* execute queries later.

Now, let's create `prod-yeniseysk`:

```shell
wrangler d1 create prod-yeniseysk
```

If succeeded, it should emit the message like this:

```shell
✅ Successfully created DB 'prod-yeniseysk' in region APAC
Created your database using D1's new storage backend. The new storage backend is not yet recommended for production workloads, but backs up
your data via point-in-time restore.

[[d1_databases]]
binding = "DB" # i.e. available in your Worker on env.DB
database_name = "prod-yeniseysk"
database_id = "{WHATEVER_THE_DATABASE_ID_ISSUED}"
```

Copy the information, and paste it to your `wrangler.toml`:  
(I changed the binding name to `DB_PROD_YENISEYSK`)

`wrangler.toml`
```diff
+ [[d1_databases]]
+ binding = "DB_PROD_YENISEYSK"
+ database_name = "prod-yeniseysk"
+ database_id = "{WHATEVER_THE_DATABASE_ID_ISSUED}"
```

```shell
vi schema.sql
--
DROP TABLE IF EXISTS users;
CREATE TABLE users (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name TEXT,
  code TEXT,
  created_at TEXT DEFAULT NULL
);
INSERT INTO users (name, code, created_at) VALUES (
  'Joe', 
  'Iu82uORO5FSV1iIVXZAKym3JH3gKYKTdJJ3F6iUHkCQCEMsXl6ZVvAVHov0WHIrr',
  'Wed, 27 Dec 2023 02:57:05 +0000'
);
INSERT INTO users (name, code, created_at) VALUES (
  'Anna',
  'FeoJTtaI5KifCKbgNmIr6Cz6wxvs21OcOywSZvkoqEFtdcQGPGTyfHupli6sFH1r',
  'Wed, 27 Dec 2023 02:57:05 +0000'
);
INSERT INTO users (name, code, created_at) VALUES (
  'Paul',
  'KQKs0739LDBrPH8XJpzvt8Ir4k0vcD4qncBMm1Il1OREIsW0gT8C5arwj0YStXFL',
  'Wed, 27 Dec 2023 02:57:05 +0000'
);
```

Now, you can execute some queries (that you have in `schema.sql`):

```shell
wrangler d1 execute prod-yeniseysk --local --file=./schema.sql
```

```shell
wrangler d1 execute prod-yeniseysk \
  --local \
  --command="SELECT id, name, SUBSTR(code, 1, 15) AS code, created_at FROM users";

┌────┬────────┬─────────────────┬─────────────────────────────────┐
│ id │ name   │ code            │ created_at                      │
├────┼────────┼─────────────────┼─────────────────────────────────┤
│ 1  │ Joe    │ Iu82uORO5FSV1iI │ Wed, 27 Dec 2023 02:57:05 +0000 │
├────┼────────┼─────────────────┼─────────────────────────────────┤
│ 2  │ Anna   │ FeoJTtaI5KifCKb │ Wed, 27 Dec 2023 02:57:05 +0000 │
├────┼────────┼─────────────────┼─────────────────────────────────┤
│ 3  │ Paul   │ KQKs0739LDBrPH8 │ Wed, 27 Dec 2023 02:57:05 +0000 │
└────┴────────┴─────────────────┴─────────────────────────────────┘
```

## 3. Running & Deploying

### 3-1. DEV

Let us run the server:

```shell
npm run dev
```

Now, we can access the APIs:

```shell
GET http://localhost:8787/users

[
  {
    "id": 1,
    "name": "Joe",
    "code": "Iu82uORO5FSV1iIVXZAKym3JH3gKYKTdJJ3F6iUHkCQCEMsXl6ZVvAVHov0WHIrr",
    "created_at": "Wed, 27 Dec 2023 02:57:05 +0000"
  },
  {
    "id": 2,
    "name": "Anna",
    "code": "FeoJTtaI5KifCKbgNmIr6Cz6wxvs21OcOywSZvkoqEFtdcQGPGTyfHupli6sFH1r",
    "created_at": "Wed, 27 Dec 2023 02:57:05 +0000"
  },
  {
    "id": 3,
    "name": "Paul",
    "code": "KQKs0739LDBrPH8XJpzvt8Ir4k0vcD4qncBMm1Il1OREIsW0gT8C5arwj0YStXFL",
    "created_at": "Wed, 27 Dec 2023 02:57:05 +0000"
  }
]
```

Let's see if we can GET only 1 user:

```shell
GET http://localhost:8787/users/1

{
  "id": 1,
  "name": "Joe",
  "code": "Iu82uORO5FSV1iIVXZAKym3JH3gKYKTdJJ3F6iUHkCQCEMsXl6ZVvAVHov0WHIrr",
  "created_at": "Wed, 27 Dec 2023 02:57:05 +0000"
}
```

Now, we will add a new user:

```shell
# Add a new user
curl -XPOST -d '{ "name": "Joseph" }' \
  "http://127.0.0.1:8787/user"

# Check the user added
wrangler d1 execute prod-yeniseysk \
  --local \
  --command="SELECT id, SUBSTR(code, 1, 15) FROM users WHERE name = 'Joseph'";
  
┌────┬─────────────────────┐
│ id │ SUBSTR(code, 1, 15) │
├────┼─────────────────────┤
│ 4  │ qIR7aLvLu7aMYzE     │
└────┴─────────────────────┘
```

### 3-2. PROD

Make sure that you leave out `--local` option this time, and execute `schema.sql` so that you can create `users` table for the PROD:

```shell
wrangler d1 execute prod-yeniseysk --file=./schema.sql
```

Once done, you can deploy:

```shell
npm run deploy
```

## 4. License

Dual-licensed under either of the followings.  
Choose at your option.

- The UNLICENSE ([LICENSE.UNLICENSE](LICENSE.UNLICENSE))
- MIT license ([LICENSE.MIT](LICENSE.MIT))

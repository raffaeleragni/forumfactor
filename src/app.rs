use askama_axum::IntoResponse;
use velvet_web::prelude::*;

pub async fn app() -> App {
    let routes = Router::new()
        .route("/", get(index))
        .route("/topics", get(topics).post(new_topic))
        .route("/posts/:topic_id", get(posts).post(new_reply))
        .authorized_cookie_claims("/login", |_: Claims| Ok(AuthResult::OK));

    #[derive(RustEmbed)]
    #[folder = "static"]
    struct S;

    dotenvy::dotenv().ok();
    let db = sqlite().await;
    sqlx::migrate!().run(&db).await.unwrap();

    App::new()
        .router(routes)
        .login_flow_with_mail(&db)
        .await
        .inject(db)
        .inject(mailer())
        .statics::<S>()
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Claims {
    username: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

async fn index() -> Index {
    Index {}
}

#[derive(Template)]
#[template(path = "topics.html")]
struct Topics {
    topics: Vec<Topic>,
}

#[derive(Debug)]
struct Topic {
    id: i64,
    title: String,
    group: String,
    author: String,
}

async fn topics(Extension(db): Extension<Pool<Sqlite>>) -> Topics {
    let topics = query!(
        "select t.id,t.title,g.title as 'group',t.author from topics t left join groups g on t.group_id = g.id order by g.title,t.title"
    )
    .fetch_all(&db)
    .await
    .unwrap()
    .into_iter()
    .map(|x| Topic {
        id: x.id,
        title: x.title.unwrap_or("".to_string()),
        group: x.group.unwrap_or("".to_string()),
        author: x.author.unwrap_or("".to_string()),
    })
    .collect();
    Topics { topics }
}

#[derive(Template)]
#[template(path = "posts.html")]
struct Posts {
    topic_id: i64,
    title: String,
    posts: Vec<Post>,
}

#[derive(Debug)]
struct Post {
    id: i64,
    post: String,
    author: String,
}

async fn posts(Extension(db): Extension<Pool<Sqlite>>, Path(topic_id): Path<i64>) -> Posts {
    let title = query!("select title from topics where id = ?", topic_id)
        .fetch_one(&db)
        .await
        .unwrap()
        .title;
    let posts = query!(
        "select id,post,author from posts where topic_id=?",
        topic_id
    )
    .fetch_all(&db)
    .await
    .unwrap()
    .into_iter()
    .map(|x| Post {
        id: x.id,
        post: x.post.unwrap_or("".to_string()),
        author: x.author.unwrap_or("".to_string()),
    })
    .collect();
    Posts {
        topic_id,
        title: title.unwrap_or("".to_string()),
        posts,
    }
}

#[derive(Deserialize)]
struct PostATopic {
    group: String,
    title: String,
    post: String,
}

#[derive(Deserialize)]
struct PostAReply {
    post: String,
}

async fn ensure_group_id(db: &Pool<Sqlite>, group: &str) -> i64 {
    let id = query!("select id from groups where title = ?", group)
        .fetch_one(db)
        .await
        .ok()
        .and_then(|r| r.id);
    match id {
        Some(v) => v,
        None => query!("insert into groups (id, title) values (null, ?)", group)
            .execute(db)
            .await
            .unwrap()
            .last_insert_rowid(),
    }
}

async fn new_topic(
    Extension(db): Extension<Pool<Sqlite>>,
    CookieClaims(claims): CookieClaims<Claims>,
    Form(form): Form<PostATopic>,
) -> impl IntoResponse {
    let group_id = ensure_group_id(&db, &form.group).await;
    let topic_id = query!(
        "insert into topics (id, title, group_id, author) values (null, ?, ?, ?)",
        form.title,
        group_id,
        claims.username
    )
    .execute(&db)
    .await
    .unwrap()
    .last_insert_rowid();

    let response = new_reply(
        Extension(db),
        CookieClaims(claims),
        Path(topic_id),
        Form(PostAReply { post: form.post }),
    )
    .await;

    let mut headers = HeaderMap::new();
    headers.insert(
        "ID",
        HeaderValue::from_str(format!("{topic_id}").as_str()).unwrap(),
    );
    (headers, response)
}

async fn new_reply(
    Extension(db): Extension<Pool<Sqlite>>,
    CookieClaims(claims): CookieClaims<Claims>,
    Path(topic_id): Path<i64>,
    Form(form): Form<PostAReply>,
) -> impl IntoResponse {
    query!(
        "insert into posts (id, topic_id, post, author) values (null, ?, ?, ?)",
        topic_id,
        form.post,
        claims.username
    )
    .execute(&db)
    .await
    .unwrap();

    posts(Extension(db), Path(topic_id)).await
}

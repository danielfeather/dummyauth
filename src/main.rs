use axum::{Router, extract::State, response::Html, routing::get};
use minijinja::{Environment, context, path_loader};

#[derive(Clone)]
struct AppState {
    minij: minijinja::Environment<'static>,
}

#[tokio::main]
async fn main() {
    let mut env = Environment::new();

    env.set_loader(path_loader("views"));

    // build our application with a single route
    let app = Router::new()
        .route("/", get(home))
        .with_state(AppState { minij: env });

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home(State(state): State<AppState>) -> Html<String> {
    let templ = state.minij.get_template("home.html").unwrap();

    let res = templ.render(context! {}).unwrap();

    Html(res)
}

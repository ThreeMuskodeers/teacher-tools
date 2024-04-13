use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use grouping::annealing::{
    http::{CreateGroupsRequest, CreateGroupsResponse},
    simulated_annealing,
};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/get-groups", get(create_groups))
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn create_groups(
    create_groups_request: Json<CreateGroupsRequest>,
) -> Result<Json<CreateGroupsResponse>, AppError> {
    let (num_groups, students, relationship_pairs) = create_groups_request.0.into_parts();
    let annealing_result =
        simulated_annealing(students, num_groups, &relationship_pairs, 10.0, 0.1, 1000)?;
    let response = CreateGroupsResponse {
        groups: annealing_result.groups,
        violations: annealing_result.violations,
        objective: annealing_result.objective,
    };
    Ok(Json(response))
}
// // Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

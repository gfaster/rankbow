use std::{collections::{BTreeMap, HashMap}, sync::{atomic::*, Arc}, time::{Duration, SystemTime}};

use axum::{
    extract::Path, http::StatusCode, response::Json, routing::{get, post}, Router
};
use serde::Serialize;
use serde_json::{Value, json};
use tracing_subscriber::FmtSubscriber;
use tracing::{trace, Level};


type Str = Box<str>;
type StdMtx<T> = std::sync::Mutex<T>;

#[derive(Default, Debug)]
struct AppState {
    id_counter: AtomicU64,
    surveys: StdMtx<BTreeMap<u64, Arc<Survey>>>,
}

#[derive(Debug)]
struct Survey {
    title: Str,
    _start: SystemTime,
    end: SystemTime,
    choices: Box<[Str]>,
    votes: StdMtx<Vec<Box<[u32]>>>,
}

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
async fn create(state: Arc<AppState>) -> Json<Value> {
    let start = SystemTime::now();
    let duration = Duration::from_secs(60);
    let survey = Survey {
        title: "New Survey".into(),
        _start: start,
        end: start + duration,
        choices: vec!["A".into(), "B".into(), "C".into(), "D".into(), "E".into()].into(),
        votes: StdMtx::new(Vec::new()),
    };
    let survey = Arc::new(survey);
    let id = state.id_counter.fetch_add(1, Ordering::Relaxed);
    state.surveys.lock().unwrap().insert(id, survey);
    trace!("Created survey id={id}");
    Json(json!({ "id": id }))
}

async fn vote(Path(id): Path<u64>, Json(vote): Json<Value>, state: Arc<AppState>) -> Result<(), StatusCode> {
    let survey = {
        let lock = state.surveys.lock().unwrap();
        Arc::clone(lock.get(&id).ok_or(StatusCode::NOT_FOUND)?)
    };

    if SystemTime::now() > survey.end {
        trace!("Recieved vote for id={id} that is out of date");
        return Err(StatusCode::FORBIDDEN)
    }

    let ranking: Vec<Str> = serde_json::from_value(vote).map_err(|_| StatusCode::BAD_REQUEST)?;

    let ranking: Vec<u32> = ranking.into_iter()
        .map(|s| survey.choices.iter().position(|choice| &**choice == &*s).map(|i| i as u32))
        .collect::<Option<_>>().ok_or(StatusCode::BAD_REQUEST)?;

    trace!("received vote for {ranking:?}");
    survey.votes.lock().unwrap().push(ranking.into());

    Ok(())
}

#[derive(Debug, Serialize)]
struct Results {
    title: Str,
    choices: Box<[Str]>,
    votes: Vec<HashMap<Str, u32>>
}

async fn results(Path(id): Path<u64>, state: Arc<AppState>) -> Result<Json<Results>, StatusCode> {
    let survey = {
        let lock = state.surveys.lock().unwrap();
        Arc::clone(lock.get(&id).ok_or(StatusCode::NOT_FOUND)?)
    };
    let mut votes = Vec::new();
    let mut eliminated: Vec<u32> = Vec::new();
    // list of the raw votes 
    let raw_votes: Box<[Box<[u32]>]> = {
        let lock = survey.votes.lock().unwrap();
        (&**lock).into()
    };
    loop {
        let mut tally = vec![0_u32; survey.choices.len()];
        for raw in &raw_votes {
            if let Some(&best_valid_option) = raw.iter().find(|x| !eliminated.contains(x)) {
                tally[best_valid_option as usize] += 1;
            }
        }
        let this_step_results = tally.iter().enumerate().filter(|&(_, &t)| t != 0).map(|(i, &t)| (survey.choices[i].clone(), t)).collect();
        votes.push(this_step_results);
        if tally.iter().filter(|&&t| t != 0).count() <= 2 {
            break
        }
        // FIXME: this isn't quite correct if there is a tie for second place
        let (min_choice, _) = tally.iter().enumerate().filter(|&(_i, &t)| t != 0).min_by_key(|&(_i, t)| t).expect("just checked that there was nonzero elements");
        debug_assert!(!eliminated.contains(&(min_choice as u32)));
        eliminated.push(min_choice as u32);
    }
    Ok(Json(Results {
        choices: survey.choices.clone(),
        title: survey.title.clone(),
        votes 
    }))
}

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found\n")
}

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder().with_max_level(Level::TRACE).finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    let shared_state = Arc::new(AppState::default());
    let app = Router::new()
        .route("/create", post({
            let shared_state = Arc::clone(&shared_state);
            move || create(shared_state)
        }))
        .route("/poll/{id}/submit", post({
            let shared_state = Arc::clone(&shared_state);
            move |id, body| vote(id, body, shared_state)
        }))
        .route("/poll/{id}/results", get({
            let shared_state = Arc::clone(&shared_state);
            move |id| results(id, shared_state)
        }))
        .route("/json", get(json))
        .fallback(fallback)
    ;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

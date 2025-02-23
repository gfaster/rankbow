use std::{
    collections::{BTreeMap, HashMap}, fmt, sync::{atomic::*, Arc}, time::{Duration, SystemTime}
};

use axum::{
    Router,
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use serde::Serialize;
use serde_json::{Value, json};
use tracing::{Level, trace};
use tracing_subscriber::FmtSubscriber;

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

async fn vote(
    Path(id): Path<u64>,
    Json(vote): Json<Value>,
    state: Arc<AppState>,
) -> Result<(), StatusCode> {
    let survey = {
        let lock = state.surveys.lock().unwrap();
        Arc::clone(lock.get(&id).ok_or(StatusCode::NOT_FOUND)?)
    };

    if SystemTime::now() > survey.end {
        trace!("Recieved vote for id={id} that is out of date");
        return Err(StatusCode::FORBIDDEN);
    }

    let ranking: Vec<Str> = serde_json::from_value(vote).map_err(|_| StatusCode::BAD_REQUEST)?;

    let ranking: Vec<u32> = ranking
        .into_iter()
        .map(|s| {
            survey
                .choices
                .iter()
                .position(|choice| &**choice == &*s)
                .map(|i| i as u32)
        })
        .collect::<Option<_>>()
        .ok_or(StatusCode::BAD_REQUEST)?;

    trace!("received vote for {ranking:?}");
    survey.votes.lock().unwrap().push(ranking.into());

    Ok(())
}

#[derive(Debug, Serialize)]
struct Results {
    title: Str,
    choices: Box<[Str]>,
    votes: Vec<Vec<VoteTallyResult>>,
    rank_fields: Vec<Str>,
}

#[derive(Debug, Serialize)]
struct VoteTallyResult {
    /// ballot option
    title: Str,
    /// eg:
    /// ```txt
    ///
    /// "first choice": 3,
    /// "second choice": 2,
    /// ...
    #[serde(flatten)]
    ranking: HashMap<Str, u32>,
}

fn ordinal_name(ord: u32) -> impl fmt::Display {
    let ret: String = match ord {
        // 0 => "zeroth".into(),
        1 => "top".into(),
        // 2 => "second".into(),
        // 3 => "third".into(),
        // 4 => "fourth".into(),
        // 5 => "fifth".into(),
        _ if ord % 10 == 1 => format!("{ord}st"),
        _ if ord % 10 == 2 => format!("{ord}nd"),
        _ if ord % 10 == 3 => format!("{ord}rd"),
        _ => format!("{ord}th"),
    };
    ret
}

impl VoteTallyResult {
    fn rank_fields(count: u32) -> Vec<Str> {
        (1..=count).map(|i| format!("{} choice", ordinal_name(i)).into_boxed_str()).collect()
    }

    fn make_ranking(option_name: Str, ranks: &[u32]) -> Self {
        let mut ranking = HashMap::new();
        for (&count, key) in ranks.iter().zip(Self::rank_fields(ranks.len() as u32)) {
            ranking.insert(key, count);
        }
        Self {
            title: option_name,
            ranking,
        }
    }
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
        let mut tally =
            vec![vec![0_u32; survey.choices.len()].into_boxed_slice(); survey.choices.len()];
        let mut has_votes = vec![false; survey.choices.len()];
        for raw in &raw_votes {
            if let Some(choice_rank) = raw.iter().position(|x| !eliminated.contains(x)) {
                let best_valid_option = raw[choice_rank] as usize;
                has_votes[best_valid_option] = true;
                tally[best_valid_option][choice_rank] += 1;
            }
        }
        let this_step_results = tally
            .iter()
            .enumerate()
            .filter(|&(i, _t)| has_votes[i])
            .map(|(i, t)| VoteTallyResult::make_ranking(survey.choices[i].clone(), &**t))
            .collect();

        votes.push(this_step_results);
        if has_votes.iter().filter(|&&b| b).count() <= 2 {
            break;
        }
        // FIXME: this isn't quite correct if there is a tie for second place
        let (min_choice, _) = tally
            .iter()
            .enumerate()
            .filter(|&(i, _t)| has_votes[i])
            .min_by_key(|&(_i, t)| t)
            .expect("just checked that there was nonzero elements");
        debug_assert!(!eliminated.contains(&(min_choice as u32)));
        eliminated.push(min_choice as u32);
    }
    Ok(Json(Results {
        choices: survey.choices.clone(),
        title: survey.title.clone(),
        rank_fields: VoteTallyResult::rank_fields(survey.choices.len() as u32),
        votes,
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
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    let shared_state = Arc::new(AppState::default());
    let app = Router::new()
        .route(
            "/create",
            post({
                let shared_state = Arc::clone(&shared_state);
                move || create(shared_state)
            }),
        )
        .route(
            "/poll/{id}/submit",
            post({
                let shared_state = Arc::clone(&shared_state);
                move |id, body| vote(id, body, shared_state)
            }),
        )
        .route(
            "/poll/{id}/results",
            get({
                let shared_state = Arc::clone(&shared_state);
                move |id| results(id, shared_state)
            }),
        )
        .route("/json", get(json))
        .fallback(fallback);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

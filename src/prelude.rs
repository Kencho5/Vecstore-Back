// Standard library
pub use std::{
    collections::{BTreeMap, HashMap},
    env,
    time::Instant,
};

// External crates
pub use anyhow::Result;
pub use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};
pub use dotenv::dotenv;
pub use google_oauth::AsyncClient;
pub use http::{HeaderMap, HeaderValue, Method, StatusCode};
pub use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
pub use paddle_rust_sdk::Paddle;
pub use pwhash::bcrypt;
pub use serde::{Deserialize, Serialize};
pub use serde_json::json;
pub use sqlx::postgres::{PgPool, PgPoolOptions};
pub use std::sync::Arc;
pub use tokenizers::Tokenizer;
pub use tokio::sync::{mpsc, Mutex};
pub use tower_http::cors::CorsLayer;
pub use uuid::Uuid;

// Axum framework
pub use axum::{
    extract::{Multipart, Request, State},
    middleware,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Extension, Json, Router,
};

// Candle ML framework
pub use candle_core::{DType, Device, Tensor};
pub use candle_nn::VarBuilder;
pub use candle_transformers::models::{
    clip,
    vit::{Config, Model},
};

// Pinecone SDK
pub use pinecone_sdk::{
    models::{Kind, Metadata, Namespace, QueryResponse, Value, Vector},
    pinecone::{data::Index, PineconeClientConfig},
};

// Local crate modules
pub use crate::{
    auth::token::*,
    loaders::*,
    middleware::{api_middleware::*, auth_middleware::*},
    routes::{auth::*, dashboard::*, payments::*},
    structs::{app_state::*, auth_struct::*, payment_struct::*, pinecone_struct::*},
    utils::{
        background_task::*, extract_features::*, get_customer_id::*, get_user_and_region::*,
        hash_api_key::*, insert_vectors::*, save_usage::*, search_vectors::*, tokenizer::*,
    },
};

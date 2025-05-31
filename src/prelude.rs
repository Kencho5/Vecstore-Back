// Standard library
pub use std::{collections::BTreeMap, env, time::Instant};

// External crates
pub use anyhow::Result;
pub use chrono::{DateTime, Duration, Utc};
pub use dotenv::dotenv;
pub use google_oauth::AsyncClient;
pub use http::{HeaderMap, HeaderValue, Method, StatusCode};
pub use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
pub use pwhash::bcrypt;
pub use reqwest::{header::HeaderName, Client};
pub use serde::{Deserialize, Serialize};
pub use serde_json::json;
pub use sqlx::postgres::{PgPool, PgPoolOptions};
pub use tokenizers::Tokenizer;
pub use tower_http::cors::CorsLayer;
pub use uuid::Uuid;

// Axum framework
pub use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
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
    pinecone::{PineconeClient, PineconeClientConfig},
};

// External crate declaration
pub extern crate image_base64;

// Local crate modules
pub use crate::{
    auth::token::*,
    loaders::*,
    routes::auth::*,
    structs::{app_state::AppState, auth_struct::*},
    utils::tokenizer::*,
};

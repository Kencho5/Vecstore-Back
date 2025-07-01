// Standard library
pub use std::{
    collections::{BTreeMap, HashMap},
    env,
    time::Instant,
};

// External crates
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
pub use tokio::sync::mpsc;
pub use tower_http::cors::CorsLayer;
pub use uuid::Uuid;

// AWS
pub use aws_config::{BehaviorVersion, Region};
pub use aws_sdk_bedrockruntime::{config::Credentials, Client as BedrockClient};
pub use aws_sdk_rekognition::Client as RekognitionClient;
pub use aws_sdk_sesv2::{
    types::{Body, Content, Destination, EmailContent, Message},
    Client as SesClient, Error,
};

// Axum framework
pub use axum::{
    extract::{Multipart, Request, State},
    middleware,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Extension, Json, Router,
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
    structs::{api_struct::*, app_state::*, auth_struct::*, payment_struct::*, pinecone_struct::*},
    utils::{
        background_task::*, extract_features::*, get_customer_id::*, get_user_and_region::*,
        hash_api_key::*, insert_vectors::*, resize_image::*, save_usage::*, search_vectors::*,
    },
};

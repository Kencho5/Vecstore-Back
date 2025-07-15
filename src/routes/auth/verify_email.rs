use crate::prelude::*;
use crate::structs::email_struct::*;

pub async fn verify_email_handler(
    State(state): State<AppState>,
    Json(payload): Json<VerifyEmailPayload>,
) -> Result<StatusCode, VerifyEmailError> {
    let code: u16 = rand::random_range(1000..9999);
    send_email(state.ses_client, &payload.email, Some(code.to_string()))
        .await
        .map_err(|_| VerifyEmailError::InvalidEmail)?;

    insert_code(state.pool, payload.email, code)
        .await
        .map_err(|_| VerifyEmailError::InsertFailed)?;

    Ok(StatusCode::OK)
}

pub async fn verify_email_code_handler(
    State(state): State<AppState>,
    Json(payload): Json<VerifyCodePayload>,
) -> Result<StatusCode, VerifyEmailError> {
    let result = sqlx::query_as::<_, EmailRecord>(
        "SELECT code, expiry FROM email_codes 
            WHERE email = $1 
            AND code = $2 
            LIMIT 1;",
    )
    .bind(&payload.email)
    .bind(&payload.code)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| VerifyEmailError::InvalidCode)?;

    match result {
        EmailRecord { code, expiry } if code == payload.code => {
            let _ = sqlx::query(
                "DELETE FROM email_codes 
                     WHERE email = $1 AND code = $2;",
            )
            .bind(&payload.email)
            .bind(&payload.code)
            .execute(&state.pool)
            .await;

            if Utc::now().naive_utc() > expiry {
                Err(VerifyEmailError::CodeExpired)
            } else {
                return Ok(StatusCode::OK);
            }
        }
        _ => return Err(VerifyEmailError::InvalidCode),
    }
}

async fn insert_code(pool: PgPool, email: String, code: u16) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO email_codes(email, code, expiry) VALUES($1, $2, NOW() + INTERVAL '5 minutes')",
    )
    .bind(&email)
    .bind(code as i32)
    .execute(&pool)
    .await?;

    Ok(())
}

async fn send_email(
    client: aws_sdk_sesv2::Client,
    recipient: &String,
    code: Option<String>,
) -> Result<(), Error> {
    let mut html: String = include_str!("code.html").to_string();
    if let Some(code) = code {
        html = html.replace("{{verification_code}}", &code);
    }

    let dest: Destination = Destination::builder().to_addresses(recipient).build();
    let subject_content = Content::builder()
        .data("Verify Email")
        .charset("UTF-8")
        .build()
        .expect("building Content");
    let body_content = Content::builder()
        .data(html)
        .charset("UTF-8")
        .build()
        .expect("building Content");
    let body = Body::builder().html(body_content).build();

    let msg = Message::builder()
        .subject(subject_content)
        .body(body)
        .build();

    let email_content = EmailContent::builder().simple(msg).build();

    client
        .send_email()
        .from_email_address("Vecstore <info@vecstore.app>")
        .destination(dest)
        .content(email_content)
        .send()
        .await?;

    Ok(())
}

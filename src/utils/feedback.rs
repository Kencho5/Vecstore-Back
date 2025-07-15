use crate::prelude::*;

pub async fn send_feedback_email(
    client: aws_sdk_sesv2::Client,
    recipient: String,
) -> Result<(), Error> {
    let html: String = include_str!("feedback.html").to_string();

    let dest: Destination = Destination::builder().to_addresses(recipient).build();
    let subject_content = Content::builder()
        .data("Your Vecstore experience - we'd love your feedback")
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

use actix_web::{
    cookie::{time::Duration, Cookie},
    http::header::ContentType,
    HttpResponse,
};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn newsletter_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut error_html = String::new();

    for m in flash_messages.iter() {
        writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    let idempotency_key = uuid::Uuid::new_v4();

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .cookie(Cookie::build("_flash", "").max_age(Duration::ZERO).finish())
        .body(format!(
            r#"<!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta http-equiv="content-type" content="text/html; charset=utf-8">
                    <title>Newsletter</title>
                </head>
                <body>
                    {error_html}
                    <form action="/admin/newsletters" method="post">
                        <label>Title
                            <input
                                type="text"
                                placeholder="Enter Title"
                                name="title"
                            >
                        </label>
                        <br>
                        <label>Plain text content:<br>
                            <textarea
                                placeholder="Enter the content in plain text"
                                name="text_content"
                                rows="20"
                                cols="50"
                            ></textarea>
                        </label>
                        <br>
                        <label>HTML content:<br>
                            <textarea
                                placeholder="Enter the content in HTML format"
                                name="html_content"
                                rows="20"
                                cols="50"
                            ></textarea>
                        </label>
                        <br>
                        <input hidden type="text" name="idempotency_key" value="{idempotency_key}">
                        <button type="submit">Post</button>
                    </form>
                    <p><a href="/admin/dashboard">&lt;- Back</a></p>
                </body>
            </html>"#,
        ))
}

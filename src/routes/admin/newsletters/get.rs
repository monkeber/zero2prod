use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn submit_new_issue_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Submit New Issue</title>
</head>
<body>
    {msg_html}
    <form action="/admin/newsletters" method="post">
        <label>Title
            <input
                placeholder="Enter a title"
                name="title"
            >
        </label>
        <br>
        <label>Html Body:</label>
        <br>
        <textarea 
            placeholder="Enter html body"
            name="html_body"
            rows="5"
            cols="20"
        ></textarea>
        <br>
        <label>Text Body:</label>
        <br>
        <textarea
            placeholder="Enter text body"
            name="text_body"
            rows="5"
            cols="20"
        ></textarea>
        <br>
        <button type="submit">Submit</button>
    </form>
    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>"#,
        )))
}

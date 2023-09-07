use actix_web::http::header::ContentType;
use actix_web::HttpResponse;

pub async fn submit_new_issue_form() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Submit New Issue</title>
</head>
<body>
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
    ))
}

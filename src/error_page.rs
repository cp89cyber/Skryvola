pub fn render_start_page() -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Skryvola</title>
    <style>
      body {{
        margin: 0;
        font-family: sans-serif;
        background: #f7f7f7;
        color: #1f1f1f;
      }}
      main {{
        max-width: 720px;
        margin: 64px auto;
        padding: 0 24px;
      }}
      h1 {{
        margin-bottom: 12px;
      }}
      p, li {{
        line-height: 1.5;
      }}
      code {{
        background: #ececec;
        padding: 2px 6px;
        border-radius: 4px;
      }}
    </style>
  </head>
  <body>
    <main>
      <h1>Skryvola</h1>
      <p>An extremely basic Linux-only browser shell.</p>
      <p>Type a URL into the address bar and press Enter.</p>
      <ul>
        <li><code>example.com</code></li>
        <li><code>https://example.com</code></li>
        <li><code>localhost:3000</code></li>
      </ul>
    </main>
  </body>
</html>"#
    )
}

pub fn render_error_page(title: &str, detail: &str, attempted: Option<&str>) -> String {
    let escaped_title = escape_html(title);
    let escaped_detail = escape_html(detail);
    let attempted_block = attempted
        .map(escape_html)
        .map(|value| format!("<p><strong>Attempted:</strong> <code>{value}</code></p>"))
        .unwrap_or_default();

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>{escaped_title}</title>
    <style>
      body {{
        margin: 0;
        font-family: sans-serif;
        background: #fff7f7;
        color: #1f1f1f;
      }}
      main {{
        max-width: 720px;
        margin: 64px auto;
        padding: 0 24px;
      }}
      h1 {{
        color: #7a1212;
      }}
      code {{
        background: #f3e5e5;
        padding: 2px 6px;
        border-radius: 4px;
      }}
    </style>
  </head>
  <body>
    <main>
      <h1>{escaped_title}</h1>
      <p>{escaped_detail}</p>
      {attempted_block}
    </main>
  </body>
</html>"#
    )
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

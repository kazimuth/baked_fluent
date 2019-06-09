use actix_web::{
    dev::Payload, http::header, web, App, FromRequest, HttpMessage, HttpRequest, HttpResponse,
    HttpServer, Result,
};
use baked_fluent::{impl_localize, localize, Localize};

impl_localize! {
    #[path("tests/i18n")]
    // disable auto-implementation of `actix_web::FromRequest` so that we can add custom logic
    #[custom_from_request(true)]
    pub struct Localizer(_);
}

impl FromRequest for Localizer {
    type Error = ();
    type Future = Result<Self, Self::Error>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // check for a cookie "user_language", to override the Accept-Language header.
        // you could also store this in a database, use an IP-to-locale lookup service,
        // etc.
        let user_pref = req.cookie("user_language");
        let user_pref = user_pref.iter().map(|c| c.value()).collect::<Vec<_>>();

        // *also* use the Accept-Language header as a fallback.
        let accept_language = req
            .headers()
            .get(header::ACCEPT_LANGUAGE)
            .map(|h| h.to_str().unwrap());

        Ok(Localizer::new(&user_pref[..], accept_language))
    }
}

fn index((loc, info): (Localizer, web::Path<(String, isize)>)) -> Result<HttpResponse> {
    let resp = HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")

        .body(format!(r#"
    <script>
    function setCookie(name,value,days) {{
        var expires = "";
        if (days) {{
            var date = new Date();
            date.setTime(date.getTime() + (days*24*60*60*1000));
            expires = "; expires=" + date.toUTCString();
        }}
        document.cookie = name + "=" + (value || "")  + expires + "; path=/";
    }}
    </script>

    <p>{}</p>
    <button onclick="setCookie('user_language', 'en', 1); location.reload();">Set locale to English</button>
    <button onclick="setCookie('user_language', 'es', 1); location.reload();">Establecer locale a español</button>
    "#, localize!(
        loc,
        greeting,
        name = &info.0[..],
        friends = info.1
    )?));
    Ok(resp)
}

fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    println!("Open http://localhost:8080/Jamie/12/ in your browser");
    HttpServer::new(|| App::new().service(web::resource("/{name}/{friend_count}/").to(index)))
        .bind("localhost:8080")?
        .run()
}

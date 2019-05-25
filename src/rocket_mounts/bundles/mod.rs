const STATIC_RESOURCES_CACHE_MAX_AGE: u32 = 259200;

use crate::rocket_include_static_resources::{EtagIfNoneMatch, StaticResponse};
use crate::rocket_cache_response::CacheResponse;

fn static_response(etag_if_none_match: EtagIfNoneMatch, id: &'static str) -> CacheResponse<StaticResponse> {
    let responder = static_response!(etag_if_none_match, id);

    CacheResponse::public_only_release(responder, STATIC_RESOURCES_CACHE_MAX_AGE, false)
}

#[get("/css/bundle.min.css")]
fn css_bundle(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "css-bundle")
}

#[get("/js/bundle.min.js")]
fn js_bundle(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "js-bundle")
}

#[get("/css/font-roboto-mono.min.css")]
fn font_roboto_mono(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "font-roboto-mono")
}

#[get("/fonts/RobotoMono-Bold.woff2")]
fn roboto_mono_bold(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "RobotoMono-Bold")
}

#[get("/fonts/RobotoMono-Light.woff2")]
fn roboto_mono_light(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "RobotoMono-Light")
}

#[get("/fonts/RobotoMono-Medium.woff2")]
fn roboto_mono_medium(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "RobotoMono-Medium")
}

#[get("/fonts/RobotoMono-Regular.woff2")]
fn roboto_mono_regular(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "RobotoMono-Regular")
}

#[get("/fonts/fa-brands-400.eot")]
fn fa_brands_400_eot(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "fa-brands-400-eot")
}

#[get("/fonts/fa-brands-400.svg")]
fn fa_brands_400_svg(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "fa-brands-400-svg")
}

#[get("/fonts/fa-brands-400.ttf")]
fn fa_brands_400_ttf(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "fa-brands-400-ttf")
}

#[get("/fonts/fa-brands-400.woff")]
fn fa_brands_400_woff(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "fa-brands-400-woff")
}

#[get("/fonts/fa-brands-400.woff2")]
fn fa_brands_400_woff2(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "fa-brands-400-woff2")
}

#[get("/fonts/fa-regular-400.eot")]
fn fa_regular_400_eot(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "fa-regular-400-eot")
}

#[get("/fonts/fa-regular-400.svg")]
fn fa_regular_400_svg(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "fa-regular-400-svg")
}

#[get("/fonts/fa-regular-400.ttf")]
fn fa_regular_400_ttf(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "fa-regular-400-ttf")
}

#[get("/fonts/fa-regular-400.woff")]
fn fa_regular_400_woff(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "fa-regular-400-woff")
}

#[get("/fonts/fa-regular-400.woff2")]
fn fa_regular_400_woff2(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "fa-regular-400-woff2")
}

#[get("/fonts/fa-solid-900.eot")]
fn fa_solid_900_eot(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "fa-solid-900-eot")
}

#[get("/fonts/fa-solid-900.svg")]
fn fa_solid_900_svg(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "fa-solid-900-svg")
}

#[get("/fonts/fa-solid-900.ttf")]
fn fa_solid_900_ttf(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "fa-solid-900-ttf")
}

#[get("/fonts/fa-solid-900.woff")]
fn fa_solid_900_woff(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "fa-solid-900-woff")
}

#[get("/fonts/fa-solid-900.woff2")]
fn fa_solid_900_woff2(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "fa-solid-900-woff2")
}

#[get("/favicon.ico")]
fn favicon(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "favicon")
}

#[get("/favicon-192.png")]
fn favicon_192(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "favicon-192")
}

#[get("/images/preload.svg")]
fn preload(etag_if_none_match: EtagIfNoneMatch) -> CacheResponse<StaticResponse> {
    static_response(etag_if_none_match, "preload")
}

pub fn rocket_handler(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .attach(StaticResponse::fairing(|resources| {
            static_resources_initialize!(
                resources,

                "css-bundle", "front-end/css/bundle.min.css",
                "js-bundle", "front-end/js/bundle.min.js",

                "font-roboto-mono", "front-end/css/font-roboto-mono.min.css",

                "RobotoMono-Bold", "front-end/fonts/RobotoMono-Bold.woff2",
                "RobotoMono-Light", "front-end/fonts/RobotoMono-Light.woff2",
                "RobotoMono-Medium", "front-end/fonts/RobotoMono-Medium.woff2",
                "RobotoMono-Regular", "front-end/fonts/RobotoMono-Regular.woff2",

                "fa-brands-400-eot", "front-end/fonts/fa-brands-400.eot",
                "fa-brands-400-svg", "front-end/fonts/fa-brands-400.svg",
                "fa-brands-400-ttf", "front-end/fonts/fa-brands-400.ttf",
                "fa-brands-400-woff", "front-end/fonts/fa-brands-400.woff",
                "fa-brands-400-woff2", "front-end/fonts/fa-brands-400.woff2",
                "fa-regular-400-eot", "front-end/fonts/fa-regular-400.eot",
                "fa-regular-400-svg", "front-end/fonts/fa-regular-400.svg",
                "fa-regular-400-ttf", "front-end/fonts/fa-regular-400.ttf",
                "fa-regular-400-woff", "front-end/fonts/fa-regular-400.woff",
                "fa-regular-400-woff2", "front-end/fonts/fa-regular-400.woff2",
                "fa-solid-900-eot", "front-end/fonts/fa-solid-900.eot",
                "fa-solid-900-svg", "front-end/fonts/fa-solid-900.svg",
                "fa-solid-900-ttf", "front-end/fonts/fa-solid-900.ttf",
                "fa-solid-900-woff", "front-end/fonts/fa-solid-900.woff",
                "fa-solid-900-woff2", "front-end/fonts/fa-solid-900.woff2",

                "favicon", "front-end/favicon.ico",
                "favicon-192", "front-end/favicon-192.png",

                "preload", "front-end/images/preload.svg",
            );
        }))
        .mount("/", routes![css_bundle, js_bundle])
        .mount("/", routes![font_roboto_mono])
        .mount("/", routes![roboto_mono_bold, roboto_mono_light, roboto_mono_medium, roboto_mono_regular])
        .mount("/", routes![fa_brands_400_eot, fa_brands_400_svg, fa_brands_400_ttf, fa_brands_400_woff, fa_brands_400_woff2])
        .mount("/", routes![fa_regular_400_eot, fa_regular_400_svg, fa_regular_400_ttf, fa_regular_400_woff, fa_regular_400_woff2])
        .mount("/", routes![fa_solid_900_eot, fa_solid_900_svg, fa_solid_900_ttf, fa_solid_900_woff, fa_solid_900_woff2])
        .mount("/", routes![favicon, favicon_192])
        .mount("/", routes![preload])
}
#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;

/*
fn format_url_vec_concat(c: &mut Criterion) {
    let index = "test_idx".to_string();
    let name = "test_alias".to_string();

    c.bench_function("format_url_vec_concat",
        |b| b.iter(|| {
            let url = vec!["/", &index[..], "/_alias/", &name[..]].concat();
            url
        }));
}

fn format_url_array_concat(c: &mut Criterion) {
    let index = "test_idx".to_string();
    let name = "test_alias".to_string();

    c.bench_function("format_url_array_concat",
        |b| b.iter(|| {
            let url = ["/", &index[..], "/_alias/", &name[..]].concat();
            url
        }));
}

fn format_url_macro(c: &mut Criterion) {
    let index = "test_idx".to_string();
    let name = "test_alias".to_string();

    c.bench_function("format_url_macro",
        |b| b.iter(|| {
            format!("/{}/_alias/{}", index, name);
        }));
}

fn format_url_concat(c: &mut Criterion) {
    let index = "test_idx".to_string();
    let name = "test_alias".to_string();

    c.bench_function("format_url_concat",
        |b| b.iter(|| {
            let mut url = "/".to_string();
            url = url + &index[..] + "/_alias/" + &name[..];
            url
        }));
}

fn format_url_push(c: &mut Criterion) {
    let index = "test_idx".to_string();
    let name = "test_alias".to_string();

    c.bench_function("format_url_push",
        |b| b.iter(|| {
            let mut url = String::with_capacity(1 + "/_alias/".len()
                + index.len() + name.len());
            url.push_str("/");
            url.push_str(&index);
            url.push_str("/_alias/");
            url.push_str(&name);
            url
        }));
}

fn format_url_push_no_capacity(c: &mut Criterion) {
    let index = "test_idx".to_string();
    let name = "test_alias".to_string();

    c.bench_function("format_url_push_no_capacity",
        |b| b.iter(|| {
            let mut url = String::new();
            url.push_str("/");
            url.push_str(&index);
            url.push_str("/_alias/");
            url.push_str(&name);
            url
        }));
}

fn format_url_array_concat_str(c: &mut Criterion) {
    let index = "test_idx";
    let name = "test_alias";

    c.bench_function("format_url_array_concat_str",
        |b| b.iter(|| {
            let url = [
                "/", index, "/_alias/", name
            ].concat();
            url
        }));
}
*/

fn format_url_benches(c: &mut Criterion) {
    let index = "test_idx";
    let name = "test_alias";

    c.bench_function("format_url_vec_concat",
        |b| b.iter(|| {
            let url = vec!["/", &index[..], "/_alias/", &name[..]].concat();
            url
        }));

    c.bench_function("format_url_array_concat",
        |b| b.iter(|| {
            let url = ["/", &index[..], "/_alias/", &name[..]].concat();
            url
        }));

    c.bench_function("format_url_macro",
        |b| b.iter(|| {
            format!("/{}/_alias/{}", index, name);
        }));

    c.bench_function("format_url_concat",
        |b| b.iter(|| {
            let mut url = "/".to_string();
            url = url + &index[..] + "/_alias/" + &name[..];
            url
        }));

    c.bench_function("format_url_push",
        |b| b.iter(|| {
            let mut url = String::with_capacity(1 + "/_alias/".len()
                + index.len() + name.len());
            url.push_str("/");
            url.push_str(&index);
            url.push_str("/_alias/");
            url.push_str(&name);
            url
        }));

    c.bench_function("format_url_push_no_capacity",
        |b| b.iter(|| {
            let mut url = String::new();
            url.push_str("/");
            url.push_str(&index);
            url.push_str("/_alias/");
            url.push_str(&name);
            url
        }));

    c.bench_function("format_url_array_concat_str",
        |b| b.iter(|| {
            let url = [
                "/", index, "/_alias/", name
            ].concat();
            url
        }));

}

criterion_group!(benches, format_url_benches);
    //format_url_array_concat, format_url_vec_concat, format_url_macro,
    //format_url_concat, format_url_push, format_url_push_no_capacity,
    //format_url_array_concat_str);
criterion_main!(benches);

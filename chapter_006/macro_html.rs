macro_rules! out_html {
    ($w:expr, $e:expr) => ($w.push_str($e));
    ($w:expr, $e:expr, $($arg:tt)*) => ($w.push_str(&format!($e, $($arg)*)));
}

macro_rules! out_html_tag {
    () => {()};

    ($e:tt) => {print!("{}", $e)};

    ($tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
        print!("<{}>", stringify!($tag));
        out_html_tag!($($inner)*);
        println!("</{}>", stringify!($tag));
        out_html_tag!($($rest)*);
    }};
}

macro_rules! out_html_tag_refactor {

    () => {};

    ($e:literal $($rest:tt)*) => {
        print!("{}", $e);
        out_html_tag_refactor!($($rest)*);
    };

    ($tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
        print!("<{}>", stringify!($tag));
        out_html_tag_refactor!($($inner)*);
        print!("</{}>", stringify!($tag));
        out_html_tag_refactor!($($rest)*);
    }};
}

fn main() {
    let mut out = String::new();
    out_html!(out, "<html>");
    out_html!(out, "<head>");
    out_html!(out, "<title>{}</title>", "Macros are the best!");
    out_html!(out, "</head>");
    out_html!(out, "<body>");
    out_html!(out, "Hello, World!");
    out_html!(out, "</body>");
    out_html!(out, "</html>");
    println!("{}", out);

    out_html_tag!(html[
        head[
            title["Macros are the best!"]
        ]
        body[
            h1["Macros are the best!"]
            p["Hello, World!"]
        ]
    ]);

    out_html_tag_refactor!(html[
        head[
            title["Macros are the best!"]
        ]
        body[
            h1["Macros are the best!"]
            p["Hello, World!"]
        ]
    ]);
}

use pulldown_cmark::{Event, Options, Parser};
use sqlite3_ext::{
    Connection, FromValue, Result, ValueRef, function::Context, sqlite3_ext_fn, sqlite3_ext_main,
};

#[sqlite3_ext_fn(n_args = 1)]
fn md_strip(ctx: &mut Context, args: &mut [&mut ValueRef]) -> Result<()> {
    ctx.set_result(strip_with_cmark(args[0].get_str()?.to_owned()))
}

#[sqlite3_ext_fn(n_args = 2)]
fn md_strip_n(ctx: &mut Context, args: &mut [&mut ValueRef]) -> Result<()> {
    let value = strip_with_cmark(args[0].get_str()?.to_owned());
    let len = args[1].get_i64();

    let output = if len > 0 {
        value.chars().take(len as usize).collect()
    } else {
        value
    };

    ctx.set_result(output)
}

#[sqlite3_ext_main]
fn init(db: &Connection) -> Result<()> {
    db.create_scalar_function("md_strip", &Default::default(), md_strip)?;
    db.create_scalar_function("md_strip_n", &Default::default(), md_strip_n)?;
    Ok(())
}

fn strip_with_cmark(value: String) -> String {
    let parser = Parser::new_ext(&value, Options::all());
    let mut output = String::with_capacity(value.len());
    for event in parser {
        let text = match event {
            Event::Text(t)
            | Event::Code(t)
            | Event::InlineMath(t)
            | Event::DisplayMath(t)
            | Event::FootnoteReference(t) => t,
            _ => continue,
        };
        output.push_str(&text);
    }
    drop(value);
    output.split_whitespace().collect::<Vec<_>>().join(" ")
}

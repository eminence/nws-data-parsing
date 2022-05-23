use std::{fs::File, io::Write, path::Path, process::Command};

struct Product<'a> {
    trigram: &'a str,
    desc: &'a str,
}

pub fn main() -> Result<(), anyhow::Error> {
    let gen_rs = Path::new("nws-product-list").join("src").join("gen.rs");
    let mut f = File::options()
        .create(false)
        .append(false)
        .write(true)
        .truncate(true)
        .open(&gen_rs)?;

    let all_text = std::fs::read_to_string("xtask/data/all_text.txt")?;

    let mut products = Vec::new();

    for line in all_text.split('\n') {
        let mut split = line.splitn(2, '\t');
        let trigram = split.next().unwrap();
        let desc = split.next().unwrap();
        products.push(Product { trigram, desc })
    }

    writeln!(
        f,
        "#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)] pub enum NWSProduct {{"
    )?;
    for product in &products {
        writeln!(f, "/// {}", product.desc)?;
        writeln!(f, "{},", product.trigram)?;
    }
    writeln!(f, "}}")?;

    writeln!(f, "impl NWSProduct {{")?;
    writeln!(f, "pub fn from_str(s: &str) -> Option<Self> {{")?;
    writeln!(f, "match s {{")?;
    for product in &products {
        writeln!(
            f,
            "\"{trigram}\" => Some(NWSProduct::{trigram}),",
            trigram = product.trigram
        )?;
    }
    writeln!(f, "_ => None,")?;
    writeln!(f, "}} }}")?;

    // ##########
    writeln!(f, "pub fn as_str(&self) -> &'static str {{")?;
    writeln!(f, "match self {{")?;
    for product in &products {
        writeln!(
            f,
            "NWSProduct::{} => \"{}\",",
            product.trigram, product.trigram
        )?;
    }
    writeln!(f, "}} }}")?;

    // ##########
    writeln!(f, "pub fn description(&self) -> &'static str {{")?;
    writeln!(f, "match self {{")?;
    for product in &products {
        writeln!(
            f,
            "NWSProduct::{} => \"{}\",",
            product.trigram,
            product.desc.trim()
        )?;
    }
    writeln!(f, "}} }}")?;

    writeln!(f, "}}")?; // end impl block

    f.flush()?;
    drop(f);

    let mut cargo = Command::new("rustfmt")
        .arg("--config")
        .arg("error_on_line_overflow=true,error_on_unformatted=true,max_width=200")
        .arg("-v")
        .arg(gen_rs)
        .spawn()?;

    cargo.wait()?;

    Ok(())
}

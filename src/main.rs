use colored::ColoredString;
use colored::Colorize;
use std::env;
use std::fs;

fn tokenize(line: String) -> Vec<String> {
    let mut out = Vec::new();
    let mut out_line = String::new();
    let mut inside_string = false;

    for c in line.chars() {
        if c == '"' {
            inside_string = !inside_string;
            if out_line.len() > 0 {
                out_line.push('"');
                out.push(out_line);
                out_line = String::new();
                continue;
            }
        }
        if c == ' ' && !inside_string {
            out_line.push(c);
            out.push(out_line);
            out_line = String::new();
        } else {
            out_line.push(c);
        }
    }
    if out_line.len() > 0 {
        out.push(out_line);
    }
    out
}

fn colorize(tokens: &Vec<String>) -> Vec<ColoredString> {
    let mut out: Vec<ColoredString> = Vec::new();
    let mut mode;
    let mut commented = false;
    for token in tokens {
        if token.contains('\n') {
            commented = false;
        }
        if token == "// " {
            mode = "comment";
            commented = true;
        } else if [
            "&& ", "|| ", "+ ", "- ", "* ", "/ ", "> ", "< ", "= ", "== ", "+=", "-=", "*=", "/=",
        ]
        .contains(&token.as_str())
        {
            mode = "operator";
        } else if [
            "as ",
            "use ",
            "extern ",
            "crate ",
            "break ",
            "const ",
            "continue ",
            "crate ",
            "else ",
            "if ",
            "enum ",
            "extern ",
            "false ",
            "fn ",
            "for ",
            "if ",
            "impl ",
            "in ",
            "for ",
            "let ",
            "loop ",
            "match ",
            "mod ",
            "move ",
            "mut ",
            "pub ",
            "impl ",
            "ref ",
            "return ",
            "Self ",
            "self ",
            "static ",
            "struct ",
            "super ",
            "trait ",
            "true ",
            "type ",
            "unsafe ",
            "use ",
            "where ",
            "while ",
            "abstract ",
            "alignof ",
            "become ",
            "box ",
            "do ",
            "final ",
            "macro ",
            "offsetof ",
            "override ",
            "priv ",
            "proc ",
            "pure ",
            "yield ",
        ]
        .contains(&token.as_str())
        {
            mode = "keyword";
        } else if &token.chars().next().unwrap() == &'"' {
            mode = "string"
        } else {
            mode = "normal";
        }

        if commented {
            mode = "comment"
        }

        match mode {
            "string" => out.push(token.truecolor(166, 227, 161).on_truecolor(30, 30, 46)),
            "keyword" => out.push(token.truecolor(203, 166, 247).on_truecolor(30, 30, 46)),
            "comment" => out.push(token.truecolor(108, 112, 134).on_truecolor(30, 30, 46)),
            "normal" => out.push(token.truecolor(205, 214, 244).on_truecolor(30, 30, 46)),
            "operator" => out.push(token.truecolor(137, 220, 235).on_truecolor(30, 30, 46)),

            _ => (),
        }
    }

    out
}

fn split_linewise(content: String) -> Vec<String> {
    let mut out = Vec::new();
    let mut line = String::new();

    for c in content.chars() {
        if c == '\n' {
            out.push(line);
            line = String::new()
        } else {
            line.push(c)
        }
    }
    out
}

fn main() {
    let width = termsize::get().unwrap().cols - 10;
    //     println!("{width:?}");
    let args = env::args().skip(1).collect::<Vec<String>>();
    for filename in args {
        let contents = fs::read_to_string(&filename).expect("file not found or something");

        println!(
            "{}{}{}",
            "╓─────┬─".truecolor(205, 214, 244).on_truecolor(30, 30, 46),
            "─".repeat((width - 1) as usize).on_truecolor(30, 30, 46),
            "╖".truecolor(205, 214, 244).on_truecolor(30, 30, 46),
        );
        println!(
            "{}",
            format!(
                "║     │  {filename:<width$}{}",
                "║",
                width = (width - 2) as usize,
            )
            .on_truecolor(30, 30, 46),
        );
        println!(
            "{}{}{}",
            "╟─────┼─".on_truecolor(30, 30, 46),
            "─".repeat((width - 1) as usize).on_truecolor(30, 30, 46),
            "╢".on_truecolor(30, 30, 46),
        );
        let mut line_no = 0;
        for line in split_linewise(contents) {
            line_no += 1;
            // println!("{}", line.green());
            let tokens = tokenize(line);
            let mut length: i32 = 0;
            print!(
                "{}",
                format!("║{line_no:>4}{}", " │").on_truecolor(30, 30, 46)
            );
            for token in colorize(&tokens) {
                length += token.chars().count() as i32;
                if length as u16 > width {
                    println!("");
                    length = 0;
                }
                if token == " ".red() {
                    print!("·");
                } else {
                    print!("{token}");
                }
            }
            //             println!("{tokens:?}");
            println!(
                "{}{}",
                " ".repeat({
                    if (width as i32 - length) < 0 {
                        0
                    } else {
                        (width as i32 - length) as usize
                    }
                })
                .on_truecolor(30, 30, 46),
                "║".on_truecolor(30, 30, 46),
            );
        }
        println!(
            "{}{}{}",
            "╙─────┴─".on_truecolor(30, 30, 46),
            "─".repeat((width - 1) as usize).on_truecolor(30, 30, 46),
            "╜".on_truecolor(30, 30, 46),
        );
    }
}

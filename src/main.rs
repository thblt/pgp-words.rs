// This file is part of pgp-words.
// Copyright (c) 2020-2023 Thibault Polge <thibault@thb.lt>.
//
// pgp-words is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.
//
// pgp-words is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use clap::Parser;
use lib::*;
use std::{env, io, process};

#[derive(Parser, Debug)]
struct Invocation {
    #[arg(long="no-echo", help="Don't display input string.", action=clap::ArgAction::SetFalse)]
    echo: bool,
    #[arg(long, help = "Echo back all input lines, unmodified")]
    passthrough: bool,
    #[arg(short, long, help="Words per line", default_value_t = 4)]
    words_per_line: usize,
    #[arg(short, long, help = "Silently ignore invalid inputs.")]
    silent_errors: bool,
    #[arg(help = "Hex string to convert.")]
    hex: Option<Vec<String>>,
    #[arg(short, long, help = "Prefix for output lines.", default_value = "\t")]
    prefix: String,
    #[arg(long="version", help="Print version number and exit.")]
    version: bool,
}

/// For interactive use.  Convert an hex string to words, and display
/// the result.
fn convert_and_display(l: &str, args: &Invocation) {
    if args.passthrough {
        print!("{}", l)
    }

    let bytes = parse_hex(&l);

    match bytes {
        Ok(bytes) => {
            if bytes.len() == 0 { return };
            // We have bytes!
            let words = convert(&bytes);
            if args.echo {
                print!("{}:\n", l.trim());
            }
            print!("{}", args.prefix);
            for (i, w) in words.iter().enumerate() {
                if i > 0 && args.words_per_line > 0 && i % args.words_per_line == 0 {
                        print!("\n{}", args.prefix);
                    }
                    print!("{} ", w);
                }
                println!("");
            }
        Err(e) => {
            if !args.silent_errors {
                println!("Error: {}", e);
            }
        }
        }
    }

    fn main() {
        let args = Invocation::parse();

        if args.version {
            println!("{} {}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));
            return
        }

        if let Some(lines) = args.hex.clone() {
            // Work with command line
            let line = lines.into_iter().collect::<String>();
            convert_and_display(&line, &args);
        } else {
            // Work with stdin
            loop {
                let mut line: String = String::new();
                if 0 == io::stdin().read_line(&mut line).unwrap() {
                    process::exit(0);
                }
                convert_and_display(&line, &args);
            }
        }
    }

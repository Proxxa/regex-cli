mod command;

use std::io::{stdin, IsTerminal, Read};

use anyhow::anyhow;
use clap::Parser;
use command::RegexCommand;
use regex::RegexBuilder;

fn main() -> Result<(), anyhow::Error> {
    let mut cli = RegexCommand::parse();

    // A very easy way to skip unnecessary computation.
    if Some(&cli.pattern) == cli.haystack.as_ref() {
        cli.debug("Pattern and haystack are equivalent. Exiting.");
        return Ok(());
    }

    if cli.pattern == "-" {
        cli.debug("Taking pattern from standard input.");
        let mut buffer = vec![];
        stdin().lock().read_to_end(&mut buffer)?;
        cli.pattern = String::from_utf8(buffer)?;
    }

    if (cli.pattern != "-" && !stdin().is_terminal()) || cli.haystack == Some("-".to_string()) {
        cli.debug("Taking haystack from standard input.");
        let mut buffer = vec![];
        stdin().lock().read_to_end(&mut buffer)?;
        cli.haystack = Some(String::from_utf8(buffer)?);
    } else {
        cli.debug("Haystack is provided by argument.");
    }

    if cli.haystack == Some("".to_string()) || cli.haystack == None {
        Err(anyhow!("Cannot have empty haystack."))?;
    }

    cli.debug_fn(|cli| {
        println!("Pattern: {}", &cli.pattern);
        println!("Haystack: {}", cli.haystack.as_ref().unwrap());
        println!(
            "Line terminator: {}",
            if let Some(terminator) = cli.line_endings.line_ending {
                terminator.escape_default().to_string()
            } else {
                if cli.line_endings.crlf {
                    "\\r\\n".to_string()
                } else {
                    "\\n".to_string()
                }
            }
        );
        println!("Case ignored? {}", cli.case_insensitive);
        println!("Whitespace ignored? {}", cli.ignore_whitespace);
        println!("Dotall? {}", cli.single_line);
        println!("Multiline? {}", cli.multiline);
        println!("Octal? {}", cli.octal);
        println!("Greed swapped? {}", cli.swap_greed);
        println!("Unicode? {}", !cli.disable_unicode);
    });

    let mut regex = RegexBuilder::new(&cli.pattern);
    regex
        .case_insensitive(cli.case_insensitive)
        .ignore_whitespace(cli.ignore_whitespace)
        .dot_matches_new_line(cli.single_line)
        .multi_line(cli.multiline)
        .octal(cli.octal)
        .swap_greed(cli.swap_greed)
        .unicode(!cli.disable_unicode);

    if let Some(terminator) = cli.line_endings.line_ending {
        if !terminator.is_ascii() {
            Err(anyhow!("Cannot have non-ASCII line terminator."))?;
        }

        regex.line_terminator(terminator as u8);
    } else {
        regex.crlf(cli.line_endings.crlf);
    }

    let regex = regex.build()?;

    if regex.replace_all(&cli.haystack.unwrap(), "").to_owned() != "" {
        std::process::exit(1);
    }

    Ok(())
}

use std::collections::HashMap;
use std::env;
use std::fs;
use tinyjson::JsonValue;

fn parse_as_jsonl(data: String) -> Vec<JsonValue> {
    let mut datas = Vec::new();
    for line in data.lines() {
        datas.push(line.parse().unwrap());
    }
    datas
}
fn parse_telegram(data: String) {
    let datas = parse_as_jsonl(data);
    for entry in datas {
        let links = match entry["outlinks"].clone() {
            JsonValue::Array(a) => a,
            _ => vec!(),
        };
        for link in links {
            println!("{}", match link {
                JsonValue::String(s) => s,
                _ => unreachable!("This is not Telegram JSONL.")
            });
        }
        println!("{}", match entry["url"].clone() {
            JsonValue::String(s) => s,
            _ => unreachable!("This is not Telegram JSONL.")});
        let maybe_object = entry.get::<HashMap<_, _>>();
        match maybe_object.unwrap().contains_key("image") {
            true => {
                if entry["image"].is_null() {
                    ()
                }
                match entry["image"].clone() {
                    JsonValue::String(s) => println!("{}", s),
                    _ => (),
                }
            },
            false => ()
        };
        if !(entry["linkPreview"].is_null() || entry["linkPreview"]["image"].is_null()) {
            match entry["linkPreview"]["image"].clone() {
                JsonValue::String(s) => println!("{}", s),
                _ => (),
            };
        }
    }
}
fn parse_twitter(data: String) {
    let datas = parse_as_jsonl(data);
    for entry in datas {
        match entry["url"].clone() {
            JsonValue::Null => (),
            JsonValue::String(s) => println!("{}", s),
            _ => unreachable!("This is not Twitter JSONL.")
        };
        let media = match entry["media"].clone() {
            JsonValue::Array(a) => a,
            JsonValue::Null => vec!(JsonValue::String("".to_string())),
            _ => unreachable!("This is not Twitter JSONL.")
        };
        for filee in media {
            let file = match filee {
                JsonValue::Object(o) => o,
                JsonValue::String(_) => continue,
                _ => unreachable!("This is not Twitter JSONL.")
            };
            let mut full_urls: Vec<String> = Vec::new();
            let mut url = "".to_string();
            if file.contains_key("fullUrl") {
                url = match file["fullUrl"].clone() {
                    JsonValue::String(s) => s,
                    JsonValue::Null => "".to_string(),
                    _ => unreachable!("Invalid twitter jsonl"),
                };
            }
            if url.is_empty() {
                let variants = match file["variants"].clone() {
                    JsonValue::Array(a) => a,
                    _ => unreachable!("This {:?} is not twitter jsonl", file["variants"].clone()),
                };
                for video in variants {
                    let videoo = match video.clone() {
                        JsonValue::Object(o) => o,
                        _ => unreachable!("Invalid twitter jsonl"),
                    };
                    let video_url = match videoo["url"].clone() {
                        JsonValue::String(s) => s,
                        _ => unreachable!("Invalid twitter jsonl.."),
                    };
                    full_urls.push(video_url);
                }
            }
            else {
                full_urls.push(url)
            }
            for outlink_full_url in full_urls {
                println!("{}", outlink_full_url);
            }
        }
    }
}
fn main() {
    let mut args = Vec::new();
    for arg in env::args() {
        args.push(arg);
    }
    if args.len() < 3 {
        let pan = match args.len() {
            1 => "You need to provide the JSONL type.",
            2 => "You need to provide the file to scrape.",
            _ => "Your arguments are a bit fishy."
        };
        eprintln!("Usage:
        {} ('twitter', 'telegram'), <file to scrape>
        Created by TheTechRobo
        Thank you for using", args[0]);
        panic!("{}", pan);
    }
    eprintln!("Using {} scraper on file {}.", args[1], args[2]);
    let contents = fs::read_to_string(&args[2])
        .expect("Something went wrong reading the file");
    if args[1] == "telegram" {
        parse_telegram(contents);
    }
    else if args[1] == "twitter" {
        parse_twitter(contents);
    }
    else {
        panic!("That's not a valid scraper. Try {} help", args[0]);
    }
}

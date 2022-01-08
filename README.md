# snscrape-jsonl-urls-extractor
extracts urls from jsonl produced by snscrape

Usage
-----
Just `python3` the one for the scraper you're using

Yes, this uses input() on stderr. That'll be fixed when I [switch this to Rust](#switching-to-rust)
(because sys.argv is confusing in interpreted languages - sometimes the thing you want is arg 1, sometimes arg 0, sometimes arg 342e+332, I dont want to deal with that.);
until then, deal with it.
(It'll also be unified into one project once I switch to rust.

Switching to Rust
-----------------
Will happen when I have access to my desktop again, because F A S T

Dupes
-----
This program seems to produce a LOT of duplicates. Use the `sort` command with the `-u` param to fix this; however this will disrupt the order of URLs.

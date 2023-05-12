# Watch (quiet) (clear) (only source) (then execute run on changes)
#cargo watch -q -c -w src/ -x run & cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
export DATABASE_URL = "postgresql://c9artuser:c9artpassword@locahost:5432/c9art"

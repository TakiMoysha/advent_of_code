set dotenv-load

run name="":
  cargo run --bin {{name}}

test name="" test_name="":
  cargo test --bin {{name}} {{test_name}} -- --nocapture

tag year:
  git tag -a {{year}} -m "Pazzles {{year}}"

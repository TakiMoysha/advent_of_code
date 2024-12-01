run name="":
  cargo run --bin {{name}}

test name="":
  cargo test --bin {{name}} -- --nocapture

tag year:
  git tag -a {{year}} -m "Pazzles {{year}}"

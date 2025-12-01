set dotenv-load

run package_year name:
  cargo run -p {{package_year}} --bin {{name}}

test package_year name test_name="":
  cargo test -p {{package_year}} --bin {{name}} {{test_name}} -- --nocapture

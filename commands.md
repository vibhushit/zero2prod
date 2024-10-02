# Commands

## When you want to see all logs comming out of a certaint est case to debug it you can run

```bash
# We are using the `bunyan` CLI to prettify the outputted logs
# The original `bunyan` requires NPM, but you can install a Rust-port with
# `cargo install bunyan`
TEST_LOG=true cargo test health_check_works | bunyan
```

### Curl Requests
- Post

```bash
curl -i -X POST -d 'email=yourname@gmail.com&name=yourname' \
http://127.0.0.1:8000/subscriptions

```

- Get

```bash
curl http://127.0.0.1:8000/health_check -v

```
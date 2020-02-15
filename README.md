# Lambda Rust Test

### Description
Test AWS Lambda function written in Rust. Inspired by this [Rust Runtime for AWS Lambda] blog post.

### Setup
AWS Lambda instances run on Amazon Linux and therefore need the `x86_64-unknown-linux-musl`
target configured in Cargo.

To build a binary for Lambda, use command `cargo build --release --target x86_64-unknown-linux-musl`.
Note that this assumes you have `musl-gcc` and supporting dependencies installed.

Once the binary is built, run `zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap`
to zip up the package and deploy to Lambda.

When configuring in Lambda for the first time, specify the runtime as `Provided`.


[Rust Runtime for AWS Lambda]: https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/

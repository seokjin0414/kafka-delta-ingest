#[cfg(not(feature = "s3"))]
compile_error!("The \"s3\" feature must be enabled to compile kafka-delta-ingest");
fn main() {}

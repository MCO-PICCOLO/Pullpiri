fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/piccolo.proto")?;
    tonic_build::compile_protos("proto/filter.proto")?;
    Ok(())
}

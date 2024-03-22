fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../protos/greeting.proto")?;
    tonic_build::compile_protos("../protos/bluetooth.proto")?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    msfs::msfs::StandaloneModule::simulate(autothrottle::module)?;
    Ok(())
}

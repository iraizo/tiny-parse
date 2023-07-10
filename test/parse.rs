#[cfg(test)]
mod tests {

    use crate::TinyMap;

    #[test]
    fn parse_file() {
        dotenvy::dotenv().unwrap();
        pretty_env_logger::init();
        
        let map = TinyMap::new("./test/mappings.tiny").unwrap();

        let desc = map.intermediate_to_named("net/minecraft/class_3713");

        assert_eq!(desc, Some(&"net/minecraft/block/GrindstoneBlock".to_owned()));

        log::info!("Parsed map and found named descriptior.");

        //serde_json::to_writer_pretty(OpenOptions::new().write(true).create(true).truncate(true).open("./map-dump.json").unwrap(), &map).unwrap();
    }
}
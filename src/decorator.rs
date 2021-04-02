use std::io::Error;
use std::time::{SystemTime, UNIX_EPOCH};

trait Plugin {
    fn version(&self) -> String;

    fn invoke(&self) -> Result<String, Error>;
}

struct PluginWarp<T: Plugin> {
    plugin: T
}

impl<T: Plugin> PluginWarp<T> {
    fn new(plugin: T) -> Self {
        Self {
            plugin
        }
    }
    fn version(&self) -> String {
        self.plugin.version()
    }

    fn invoke(&self) -> Result<String, Error> {
        self.plugin.invoke()
    }

    fn download_time(&self) -> u32 {
        let start = SystemTime::now();
        match start
            .duration_since(UNIX_EPOCH) {
            Ok(time) => {
                time.subsec_millis()
            }
            Err(_) => {
                0
            }
        }
    }
}

struct IdeaPlugin;

impl Plugin for IdeaPlugin {
    fn version(&self) -> String {
        String::from("v1.0.0")
    }

    fn invoke(&self) -> Result<String, Error> {
        Ok(String::from("success"))
    }
}

#[test]
fn test() {
    let warp = PluginWarp::new(IdeaPlugin);
    ok!(warp.invoke(),(|x| assert_eq!(x,"success")));
    assert_eq!(warp.version(), "v1.0.0");

    let start = SystemTime::now();
    let now = start.duration_since(UNIX_EPOCH).unwrap();
    assert_eq!(warp.download_time().le(&now.subsec_millis()),true)
}


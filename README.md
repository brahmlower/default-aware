
# Default-Aware

A tiny struct for indicating if a value was generated via the Default trait or not.

Installation:

```toml
[dependencies]
default_aware = "0.1.0"
```

## Usage

If you're deserializing a struct and want to behavior based on a value being explicitly set or not, you can use the `DefaultAware` struct to wrap the type in question, and then inspect it after derserializing to see if the value was created by the `Default` trait, or if it came from the deserialized source:

```rust
use default_aware::DefaultAware;
use serde::{self, Deserialize};
use serde_json;

// This represents some value you might deserialize, which implements the `Default` trait.
#[derive(Deserialize, PartialEq, Debug)]
struct Port(u32);

impl Default for Port {
    fn default() -> Self {
        Port(8000)
    }
}

#[derive(Deserialize)]
pub struct MyConfig {
    #[serde(default)]
    http_port: DefaultAware<Port>,
}

fn main() {

    // The first config sets the port number to the same as the default. If we weren't
    // using the `DefaultAware` wrapper, the value recieved after deserializing would be
    // indistinguishable from the default value. Therefore we could not know if the
    // `http_port` field was actually provided in the document or not.
    let config1_json: &str = r#"{ "http_port": 8000 }"#;

    // But since we are using the `DefaultAware` wrapper, we can easily tell that the value
    // was not created via the `Default` implementation.
    let config1: MyConfig = serde_json::from_str(config1_json).unwrap();
    assert_eq!(false, config1.http_port.is_default());
    assert_eq!(Port(8000), config1.http_port.unwrap());


    // We can demonstrate the behavior when the default value is used by checking the output
    // on a document that omits the `http_port` field.
    const config2_json: &str = r#"{  }"#;

    // The field has the same wrapped value as before, but notice that we tell it
    // was created via the Default implementation.

    let config2: MyConfig = serde_json::from_str(config2_json).unwrap();
    assert_eq!(Port(8000), *config2.http_port.as_ref());
    assert_eq!(true, config2.http_port.is_default());

    // The type is just an enun, so we can use all the typical enum patterns
    match config2.http_port {
        DefaultAware::Default(_) => println!("value set by default"),
        DefaultAware::Declared(_) => println!("value set in the config"),
    }

    if let DefaultAware::Default(_) = config2.http_port {
        println!("Port number not set! Running on port 8000 by default. Silence this \
            warning by setting the `http_port` field in your config.");
    }
}
```


## Features

Serde functionality is guarded by the feature `serde` and is enabled by default. You can use this crate without serde by listing the dependency like so:

```toml
[dependencies]
default_aware = { version = "0.1.0", default-features = false }
```

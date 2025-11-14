# kuksa-rust-sdk

The Rust SDK for the Eclipse KUKSA Databroker.

The following protocols are supported by the Rust SDK:

- kuksa.val.v2
- kuksa.val.v1
- sdv.databroker.v1 (deprecated)

## Quickstart

The following samples show to initiate a Client and how to subscribe, read and update a value.
You can find the sample
code [here](https://github.com/eclipse-kuksa/kuksa-rust-sdk/tree/main/databroker-examples/examples/kuksa_rust_sdk.rs)

### cargo.toml

To use the Rust SDK for KUKSA add the following entry to the cargo.toml of the corresponding project.

```toml
kuksa-rust-sdk = "<<latest-version>>"
```

To check a development version try:

```toml
kuksa-rust-sdk = { git = "https://github.com/eclipse-kuksa/kuksa-rust-sdk.git", branch = "main" }
```

### kuksa.val.v2

#### Create an SDVClient

```rust
async fn create_sdv_client() {
    let host = "http://localhost:55555";
    let mut v2_client: KuksaClientV2 = KuksaClientV2::from_host(host);
}
```

#### Subscribe to a VSS Path

```rust
async fn subscribe_to_vehicle_speed() {
    match v2_client.subscribe(vec!["Vehicle.Speed".to_owned()], None, None).await
    {
        Ok(mut stream) => {
            println!("Successfully subscribed to {:?}!", "Vehicle.Speed");
            tokio::spawn(async move {
                match stream.message().await {
                    Ok(option) => {
                        let response = option.unwrap();
                        for entry_update in response.entries {
                            let datapoint = entry_update.1;
                            println!("Vehicle.Speed: {:?}", datapoint);
                        }
                    }
                    Err(err) => {
                        println!("Error: Could not receive response {:?}", err);
                    }
                }
            });
        }
        Err(err) => {
            println!("Failed to subscribe to {:?}: {:?}", "Vehicle.Speed", err);
        }
    }
}
```

#### Update a VSS Path

```rust
async fn update_vehicle_speed() {
    match v2_client.publish_value(
        "Vehicle.Speed".to_owned(),
        v2_proto::Value {
            typed_value: Some(v2_proto::value::TypedValue::Float(30.0)),
        },
    )
        .await
    {
        Ok(_) => {
            println!(
                "Value published successful for signal {:?}",
                "Vehicle.Speed"
            );
        }
        Err(err) => {
            println!(
                "Publishing value for signal {:?} failed: {:?}",
                "Vehicle.Speed", err
            );
        }
    }
}
```

#### Read a VSS Path

```rust
async fn read_vehicle_speed() {
    match v2_client.get_value("Vehicle.Speed".to_owned()).await {
        Ok(response) => {
            println!("Got value for Vehicle.Speed: {:?}", response);
        }
        Err(err) => {
            println!(
                "Getting value for signal {:?} failed: {:?}",
                "Vehicle.Speed", err
            );
        }
    }
}
```

### kuksa.val.v1

#### Create an SDVClient

```rust
async fn create_sdv_client() {
    let host = "http://localhost:55555";
    let mut v1_client: KuksaClient = KuksaClient::from_host(host);
}
```

#### Subscribe to a VSS Path

```rust
async fn subscribe_to_vehicle_speed() {
    match v1_client.subscribe(vec!["Vehicle.Speed".to_owned()]).await {
        Ok(mut stream) => {
            println!("Successfully subscribed to {:?}!", "Vehicle.Speed");
            tokio::spawn(async move {
                match stream.message().await {
                    Ok(option) => {
                        let response = option.unwrap();
                        for entry_update in response.updates {
                            let entry = entry_update.entry.unwrap();
                            println!("Vehicle.Speed: {:?}", entry);
                        }
                    }
                    Err(err) => {
                        println!("Error: Could not receive response {:?}", err);
                    }
                }
            });
        }
        Err(err) => {
            println!("Failed to subscribe to {:?}: {:?}", "Vehicle.Speed", err);
        }
    }
}
```

#### Update a VSS Path

```rust
async fn update_vehicle_speed() {
    let mut datapoints = HashMap::with_capacity(1);
    datapoints.insert(
        "Vehicle.Speed".to_owned(),
        v1_proto::Datapoint {
            timestamp: None,
            value: Some(v1_proto::datapoint::Value::Float(40.0)),
        },
    );

    match v1_client.set_current_values(datapoints).await {
        Ok(_) => {
            println!("Successfully set datapoints")
        }
        Err(err) => {
            println!("Failed to set datapoints: {:?}", err)
        }
    }
}
```

#### Read a VSS Path

```rust
async fn read_vehicle_speed() {
    match v1_client.get_current_values(vec!["Vehicle.Speed".to_owned()]).await
    {
        Ok(response) => {
            println!("Got value for Vehicle.Speed: {:?}", response);
        }
        Err(err) => {
            println!("Couldn't get value for Vehicle.Speed: {:?}", err)
        }
    }
}
```

### sdv.databroker.v1

The sdv.databroker.v1 protocol is deprecated and only supported for a limited time. To use it, it is required to start
databroker with parameter `--enable-databroker-v1`

#### Create an SDVClient

```rust
async fn create_sdv_client() {
    let host = "http://localhost:55555";
    let mut sdv_client: SDVClient = SDVClient::from_host(host);
}
```

#### Subscribe to a VSS Path

```rust
async fn subscribe_to_vehicle_speed() {
    match sdv_client.subscribe("SELECT Vehicle.Speed".to_owned()).await {
        Ok(mut stream) => {
            println!("Successfully subscribed to {:?}!", "Vehicle.Speed");
            tokio::spawn(async move {
                match stream.message().await {
                    Ok(option) => {
                        let response = option.unwrap();
                        if let Some(speed) = response.fields.get("Vehicle.Speed") {
                            println!("Vehicle.Speed: {:?}", speed);
                        };
                    }
                    Err(err) => {
                        println!("Error: Could not receive response {:?}", err);
                    }
                }
            });
        }
        Err(err) => {
            println!("Failed to subscribe to {:?}: {:?}", "Vehicle.Speed", err);
        }
    }
}
```

#### Update a VSS Path

```rust
async fn update_vehicle_speed() {
    let mut datapoints = HashMap::with_capacity(1);
    datapoints.insert(
        "Vehicle.Speed".to_owned(),
        sdv_proto::Datapoint {
            timestamp: None,
            value: Some(sdv_proto::datapoint::Value::FloatValue(50.0)),
        },
    );

    match sdv_client.update_datapoints(datapoints).await {
        Ok(_) => {
            println!("Successfully set datapoints")
        }
        Err(err) => {
            println!("Failed to set datapoints: {:?}", err)
        }
    }
}
```

#### Read a VSS Path

```rust
async fn read_vehicle_speed() {
    match sdv_client.get_datapoints(vec!["Vehicle.Speed".to_owned()])
    .await
    {
        Ok(response) => {
            println!("Got value for Vehicle.Speed: {:?}", response);
        }
        Err(err) => {
            println!("Failed to get value for Vehicle.Speed: {:?}", err)
        }
    }
}
```

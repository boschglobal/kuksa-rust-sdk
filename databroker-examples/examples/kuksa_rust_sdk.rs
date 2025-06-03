/*
 * *******************************************************************************
 *  Copyright (c) 2025 Contributors to the Eclipse Foundation
 *
 *  See the NOTICE file(s) distributed with this work for additional
 *  information regarding copyright ownership.
 *
 *  This program and the accompanying materials are made available under the
 *  terms of the Apache License 2.0 which is available at
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 *  SPDX-License-Identifier: Apache-2.0
 * ******************************************************************************
 */
use kuksa_rust_sdk::kuksa::common;
use kuksa_rust_sdk::kuksa::val::v1::KuksaClient;
use kuksa_rust_sdk::kuksa::val::v2::KuksaClientV2;
use kuksa_rust_sdk::sdv::databroker::v1::SDVClient;
use kuksa_rust_sdk::{sdv_proto, v1_proto, v2_proto};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let host = if cfg!(target_os = "macos") {
        "http://localhost:55556"
    } else {
        "http://localhost:55555"
    };

    execute_v2_calls(host).await;
    execute_v1_calls(host).await;
    execute_sdv_calls(host).await;
}

async fn execute_v2_calls(host: &'static str) {
    let mut v2_client: KuksaClientV2 = KuksaClientV2::from_host(host);

    match common::ClientTraitV2::subscribe(
        &mut v2_client,
        vec!["Vehicle.Speed".to_owned()],
        None,
        None,
    )
    .await
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

    match common::ClientTraitV2::publish_value(
        &mut v2_client,
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

    match common::ClientTraitV2::get_value(&mut v2_client, "Vehicle.Speed".to_owned()).await {
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

async fn execute_v1_calls(host: &'static str) {
    let mut v1_client: KuksaClient = KuksaClient::from_host(host);

    match common::ClientTraitV1::subscribe(&mut v1_client, vec!["Vehicle.Speed".to_owned()]).await {
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

    let mut datapoints = HashMap::with_capacity(1);
    datapoints.insert(
        "Vehicle.Speed".to_owned(),
        v1_proto::Datapoint {
            timestamp: None,
            value: Some(v1_proto::datapoint::Value::Float(40.0)),
        },
    );

    match common::ClientTraitV1::set_current_values(&mut v1_client, datapoints).await {
        Ok(_) => {
            println!("Successfully set datapoints")
        }
        Err(err) => {
            println!("Failed to set datapoints: {:?}", err)
        }
    }

    match common::ClientTraitV1::get_current_values(
        &mut v1_client,
        vec!["Vehicle.Speed".to_owned()],
    )
    .await
    {
        Ok(response) => {
            println!("Got value for Vehicle.Speed: {:?}", response);
        }
        Err(err) => {
            println!("Couldn't get value for Vehicle.Speed: {:?}", err)
        }
    }
}

async fn execute_sdv_calls(host: &'static str) {
    let mut sdv_client: SDVClient = SDVClient::from_host(host);

    match common::SDVClientTraitV1::subscribe(&mut sdv_client, "SELECT Vehicle.Speed".to_owned())
        .await
    {
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

    let mut datapoints = HashMap::with_capacity(1);
    datapoints.insert(
        "Vehicle.Speed".to_owned(),
        sdv_proto::Datapoint {
            timestamp: None,
            value: Some(sdv_proto::datapoint::Value::FloatValue(50.0)),
        },
    );

    match common::SDVClientTraitV1::update_datapoints(&mut sdv_client, datapoints).await {
        Ok(_) => {
            println!("Successfully set datapoints")
        }
        Err(err) => {
            println!("Failed to set datapoints: {:?}", err)
        }
    }

    match common::SDVClientTraitV1::get_datapoints(
        &mut sdv_client,
        vec!["Vehicle.Speed".to_owned()],
    )
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

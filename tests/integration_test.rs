use splatoon3_rs::client::SplaClient;

#[tokio::test]
async fn test_get_next_bankara_open_stages() {
    let client = SplaClient::new().expect("Failed to create client");
    let result = client.get_next_bankara_open_stages().await;

    assert!(result.is_ok(), "API request failed: {:?}", result.err());

    let schedules = result.unwrap();
    println!("Received schedule: {:?}", schedules);

    for schedule in schedules {
        assert!(!schedule.stages.is_empty(), "No stages returned");
        assert!(!schedule.start_time.is_empty(), "Start time is empty");
        assert!(!schedule.end_time.is_empty(), "End time is empty");

        for stage in &schedule.stages {
            assert!(stage.id > 0, "Invalid stage ID");
            assert!(!stage.name.is_empty(), "Stage name is empty");
        }
    }
}

#[tokio::test]
async fn test_get_now_regular_stages() {
    let client = SplaClient::new().expect("Failed to create client");
    let result = client.get_now_regular_open_stages().await;

    assert!(result.is_ok(), "API request failed: {:?}", result.err());

    let schedules = result.unwrap();
    println!("Received schedule: {:?}", schedules);

    for schedule in schedules {
        assert!(!schedule.stages.is_empty(), "No stages returned");
        assert!(!schedule.start_time.is_empty(), "Start time is empty");
        assert!(!schedule.end_time.is_empty(), "End time is empty");

        for stage in &schedule.stages {
            assert!(stage.id > 0, "Invalid stage ID");
            assert!(!stage.name.is_empty(), "Stage name is empty");
        }
    }
}

#[tokio::test]
async fn test_get_x_schedule() {
    let client = SplaClient::new().expect("Failed to create client");
    let result = client.get_x_schedule().await;

    assert!(result.is_ok(), "API request failed: {:?}", result.err());

    let schedules = result.unwrap();
    println!("Received schedule: {:?}", schedules);

    for schedule in schedules {
        assert!(!schedule.stages.is_empty(), "No stages returned");
        assert!(!schedule.start_time.is_empty(), "Start time is empty");
        assert!(!schedule.end_time.is_empty(), "End time is empty");

        for stage in &schedule.stages {
            assert!(stage.id > 0, "Invalid stage ID");
            assert!(!stage.name.is_empty(), "Stage name is empty");
        }
    }
}

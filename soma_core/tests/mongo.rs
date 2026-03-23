#![cfg(feature = "mongo")]

#[tokio::test]
async fn mongo_add_remove_patient() {
    use soma_core::mongo::model::address::Address;
    use soma_core::mongo::model::id_number::IdNumber;
    use soma_core::mongo::model::insurance_number::InsuranceNumber;
    use soma_core::mongo::model::phone_number::PhoneNumber;
    use soma_core::mongo::model::patient::Patient;
    use soma_core::mongo::connect;
    use soma_core::mongo::patient_collection::{ get_patient, remove_patient, add_patient};
    use mongodb::bson::doc;
    use log::{ info, error };

    let patient = Patient {
        _id: None,
        first_name: String::from("John"),
        last_name: String::from("Smith"),
        dob: String::from("1991-01-01"),
        sex: String::from("M"),
        address: Address {
            street: String::from("123 Some St"),
            city: String::from("Anytown"),
            state: String::from("CA"),
            zip: String::from("12345"),
        },
        phone_numbers: vec![
            PhoneNumber {
                r#type: String::from("home"),
                number: String::from("124-456-7890"),
            },
            PhoneNumber {
                r#type: String::from("work"),
                number: String::from("096-765-4321"),
            },
        ],
        insurance_numbers: vec![InsuranceNumber {
            provider: String::from("Health Insurance"),
            policy_number: String::from("553456789"),
        }],
        id_numbers: vec![IdNumber {
            r#type: String::from("SSN"),
            number: String::from("123-88-6789"),
        }],
    };

    let client = connect().await.expect("Failed to connect to MongoDB");

    let result = add_patient(&client, &patient).await;
    match result {
        Ok(_) => {}
        Err(e) => error!("Test failed whilst adding patient: {}", e),
    }

    // Clean up test data
    let patient_filter = doc! { "first_name": "John", "last_name": "Smith" };
    let patient_matches = get_patient(&client, patient_filter).await;
    let result = match patient_matches {
        Ok(matches) => {
            matches
        }
        Err(e) => {
            println!("Error retrieving patient: {}", e);
            assert!(false);
            vec![]
        }
    };

    let patient_id = result[0]._id.as_ref().unwrap().to_hex();
    let result = remove_patient(&client, &patient_id).await;

    match result {
        Ok(_) => info!("Test data cleaned up successfully!"),
        Err(e) => {
            error!("Error cleaning up test data: {}", e);
            assert!(false);
        }
    }
    drop(client);
}

#[tokio::test]
async fn mongo_get_and_update_patient() {
    use soma_core::mongo::patient_collection::get_patient;
    use soma_core::mongo::connect;
    use mongodb::bson::doc;
    use log::{ info, error };

    let client = connect().await.expect("Failed to connect to MongoDB");
    let patient_results =
        get_patient(&client, doc! { "first_name": "John", "last_name": "Doe" }).await;

    match patient_results {
        Ok(result) => {
            assert!(result.len() == 1);

            for patient in result {
                info!("Patient: {} {}", patient.first_name, patient.last_name);
                assert_eq!(patient.first_name, "John");
                assert_eq!(patient.last_name, "Doe");
                assert_eq!(patient.dob, "1990-01-01");
                assert!(patient._id.is_some());
            }
        }
        Err(e) => {
            error!("Error retrieving patient: {}", e);
            assert!(false);
        }
    }
    drop(client);
}

#[tokio::test]
async fn mongo_get_patient_by_name() {
    use soma_core::mongo::connect;
    use soma_core::mongo::patient_collection::get_patient_by_name;
    use log::{ info };

    let client = connect().await.unwrap();
    let patients = get_patient_by_name(&client, "John", "Doe").await;
    match patients {
        Ok(patients) => {
            assert_eq!(patients.len(), 1);
            info!("Patient found: {:?}", patients[0]);
            println!("Patient: {:?}", patients[0]);
        }
        Err(e) => {
            println!("Error retrieving patient: {}", e);
            assert!(false)
        }
    }
    drop(client);
}


/// User collection tests
#[tokio::test]
async fn mongo_add_remove_user() {
    use soma_core::mongo::model::id_number::IdNumber;
    use soma_core::mongo::model::phone_number::PhoneNumber;
    use soma_core::mongo::model::user::User;
    use soma_core::mongo::user_collection::{ get_user, remove_user, add_user};
    use soma_core::mongo::connect;
    use mongodb::bson::doc;
    use log::{ info, error };

    let user = User {
        _id: None,
        first_name: "Andrew".to_string(),
        last_name: "Warren".to_string(),
        email: "andrew@someemail.com".to_string(),
        phone_numbers: vec![
            PhoneNumber {
                r#type: "home".to_string(),
                number: "123-456-7890".to_string(),
            },
            PhoneNumber {
                r#type: "work".to_string(),
                number: "098-765-4321".to_string(),
            },
        ],
        specialty: "Clinical Geneticist".to_string(),
        level: "Consultant".to_string(),
        id_numbers: vec![IdNumber {
            r#type: "SSN".to_string(),
            number: "123-45-6789".to_string(),
        }]
    };

    let client = connect().await.expect("Failed to connect to MongoDB");

    let result = add_user(&client, &user).await;
    match result {
        Ok(_) => {}
        Err(e) => panic!("Test failed: {}", e),
    }

    // Clean up test data
    let user_filter = doc! { "last_name": "Warren" };
    let user_matches = get_user(&client, user_filter).await;
    let result = match user_matches {
        Ok(matches) => {
            info!("Found user: {:?}", matches[0]);
            matches
        }
        Err(e) => {
            panic!("Error retrieving user: {}", e);
        }
    };

    let user_id = result[0]._id.as_ref().unwrap().to_hex();
    info!("user id: {:?}", user_id);
    let result = remove_user(&client, &user_id).await;

    match result {
        Ok(_) => info!("Test data cleaned up successfully!"),
        Err(e) => error!("Error cleaning up test data: {}", e),
    }
    drop(client);
}

#[tokio::test]
async fn mongo_get_and_update_user() {
    use soma_core::mongo::user_collection::get_user;
    use soma_core::mongo::connect;
    use mongodb::bson::doc;
    use log::{ info, error };

    let client = connect().await.expect("Failed to connect to MongoDB");
    let user_results =
        get_user(&client, doc! { "last_name": "Mayou" }).await;

    match user_results {
        Ok(result) => {
            info!("number of users found: {}", result.len());
            assert!(result.len() == 1);

            for user in result {
                info!("User: {} {}", user.first_name, user.last_name);
                assert_eq!(user.first_name, "Caroline");
                assert_eq!(user.last_name, "Mayou");
                assert!(user._id.is_some());
            }
        }
        Err(e) => {
            error!("Error retrieving user: {}", e);
            assert!(false);
        }
    }
    drop(client);
}
#[cfg(test)]
mod api_tests {
    use actix_web::{test, web, App, HttpResponse};
    use secure_encryption_api::{
        config::Config,
        errors::ApiError,
        handlers::{decrypt_handler, encrypt_handler},
        models::encrypt_decrypt::{DecryptRequest, EncryptRequest},
        services::{decryption, encryption},
    };

    /// Helper function to initialize the Actix Web app
    async fn setup_app() -> App {
        let config = Config::from_env().expect("Failed to load configuration");

        test::init_service(
            App::new()
                .app_data(web::Data::new(config))
                .route("/encrypt", web::post().to(encrypt_handler::encrypt))
                .route("/decrypt", web::post().to(decrypt_handler::decrypt)),
        )
        .await
    }

    /// -------------------------
    /// VALIDATION TESTS
    /// -------------------------

    #[test]
    fn test_encrypt_request_validate_success() {
        let req = EncryptRequest {
            message: "Hello".to_string(),
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_encrypt_request_validate_empty_message() {
        let req = EncryptRequest {
            message: "".to_string(),
        };
        let result = req.validate();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Validation error: Message cannot be empty."
        );
    }

    #[test]
    fn test_decrypt_request_validate_success() {
        let req = DecryptRequest {
            encrypted_message: "validencryptedtext".to_string(),
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_decrypt_request_validate_empty_message() {
        let req = DecryptRequest {
            encrypted_message: "".to_string(),
        };
        let result = req.validate();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Validation error: Encrypted message cannot be empty."
        );
    }

    /// -------------------------
    /// SERVICE TESTS
    /// -------------------------

    #[test]
    fn test_encrypt_message_success() {
        let config = Config::from_env().expect("Failed to load configuration");
        let message = "Hello, World!";
        let result = encryption::encrypt_message(message, &config);

        assert!(result.is_ok());
        let encrypted_message = result.unwrap();
        assert!(!encrypted_message.is_empty());
    }

    #[test]
    fn test_encrypt_message_invalid_key() {
        let config = Config {
            cipher: Config::create_cipher(&[0u8; 32]), // Invalid key for testing
        };
        let message = "Hello, World!";
        let result = encryption::encrypt_message(message, &config);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap().to_string(), "Encryption error.");
    }

    #[test]
    fn test_decrypt_message_success() {
        let config = Config::from_env().expect("Failed to load configuration");
        let message = "Hello, World!";
        let encrypted =
            encryption::encrypt_message(message, &config).expect("Failed to encrypt message");
        let result = decryption::decrypt_message(&encrypted, &config);

        assert!(result.is_ok());
        let decrypted_message = result.unwrap();
        assert_eq!(decrypted_message, message);
    }

    #[test]
    fn test_decrypt_message_invalid_ciphertext() {
        let config = Config::from_env().expect("Failed to load configuration");
        let invalid_ciphertext = "invalidciphertext";
        let result = decryption::decrypt_message(invalid_ciphertext, &config);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap().to_string(), "Decryption error.");
    }

    /// -------------------------
    /// HANDLER TESTS (INTEGRATION)
    /// -------------------------

    #[actix_web::test]
    async fn test_encrypt_handler_success() {
        let app = setup_app().await;

        let req_payload = EncryptRequest {
            message: "Hello, World!".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/encrypt")
            .set_json(&req_payload)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body.get("encrypted_message").is_some());
    }

    #[actix_web::test]
    async fn test_decrypt_handler_success() {
        let app = setup_app().await;

        // First encrypt a message
        let req_payload = EncryptRequest {
            message: "Hello, World!".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/encrypt")
            .set_json(&req_payload)
            .to_request();
        let resp = test::call_service(&app, req).await;
        let body: serde_json::Value = test::read_body_json(resp).await;
        let encrypted_message = body
            .get("encrypted_message")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        // Then decrypt it
        let req_payload = DecryptRequest { encrypted_message };

        let req = test::TestRequest::post()
            .uri("/decrypt")
            .set_json(&req_payload)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body.get("original_message").unwrap(), "Hello, World!");
    }
}

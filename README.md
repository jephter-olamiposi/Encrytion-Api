# Secure Encryption API

  Secure Encryption API is a high-performance, production-ready RESTful service for securely encrypting and decrypting messages. Built with Rust and powered by AES-256-GCM, it provides robust security, scalability, and fault tolerance for modern applications. This API is designed for developers and teams who prioritize secure data handling and seamless integration.

# Features

# 🔒 Advanced Encryption
- Implements AES-256-GCM, a widely trusted cryptographic algorithm ensuring data confidentiality, integrity, and authenticity.
  
# ⚡ High Performance
- Built on Actix Web, enabling high concurrency and minimal latency for enterprise-grade workloads.

# 🚀 Developer-Friendly
- Fully documented endpoints with Swagger UI for interactive exploration.
- Postman Collection for easy testing and debugging.

# ✅ Error-Resilient
- Comprehensive input validation and detailed error handling, ensuring security and reliability.

# 🛠️ Configurable and Scalable
- Environment-driven configuration for seamless deployment across environments.
- Dockerized for rapid container-based deployment and scalability.

# Endpoints:
- Encrypt: POST /encrypt – Send a message to receive encrypted text.
- Decrypt: POST /decrypt – Send encrypted text to retrieve the original message.

# Interactive API Documentation
- Explore the API in Swagger UI: [Secure Encryption API Docs](https://encryption-api-cdn0.onrender.com/docs/)
- Test the API using Postman: [Postman collection](https://www.postman.com/lunar-module-geoscientist-9458215/jephter-olaifa/collection/9yjmi3d/secure-encryption-api?action=share&creator=33056158)

# How It Works 👌🏼
 Encryption Logic 
- Converts plaintext into an encrypted Base64 string using AES-256-GCM.
- Ensures confidentiality and integrity through authenticated encryption.

 Decryption Logic
- Reverses the encryption process, validating the ciphertext and decrypting it back to plaintext.

 Error Handling
- Comprehensive error responses, including validation for key length, port configuration, and payload structure.

 Shared Cipher
- Optimized for performance by reusing a shared cipher instance, reducing overhead during heavy usage.

# Why Choose Secure Encryption API?
- Secure by Design: Adheres to cryptographic best practices.
- Production-Ready: Handles high-concurrency environments efficiently.
- Developer Focused: Easy to integrate, test, and deploy.
- Portable: Containerized for seamless deployment across any cloud provider or infrastructure.

# Contributing 💫
Contributions are welcome! Fork this repository and submit a pull request with your improvements. For major changes, please open an issue first to discuss your ideas.

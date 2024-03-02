# Rust REST Microservice Example

This README.md template is intended to serve as base for your repository. 

This project is a Rust-based REST microservice that demonstrates handling user data using Actix-web. It features a dummy database ```(dummy_db.rs)``` for simulating user data operations and showcases how to retrieve specific and random user entries. The project is structured to promote best practices in Rust development, including modularity, testing, and efficient error handling.

## Table of Contents

- [Installation](#installation)
  - [Prerequisites](#prerequisites)
  - [Setup](#setup)
- [Usage](#usage)
- [Features](#features)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Installation

### Prerequisites

Before installing, ensure you have the following tools installed:

- Rust and Cargo (latest stable version recommended)
- Git

### Setup

To set up the project on your local machine, follow these steps:

```bash
git clone https://github.com/rbbozkurt/rust-rest-microservice
cd rust_rest_microservice
./setup.sh
```
## Usage

To start the server, run:

```bash
make run
```
Once the server is running, you can access the ```/users``` endpoint to retrieve a random user:

```bash
curl http://localhost:8080/users
```

## Features
- **User Data Management:** Simulate user data retrieval from a dummy database.
- **Random User Retrieval:** Endpoint to fetch a random user's details.
- **Specific User Retrieval:** Functionality to fetch details of a user by ID (demonstrated in tests).
- **Actix-web Framework:** Utilizes the Actix-web framework for robust and efficient web server capabilities.
- **Modular Design:** Code is organized into modules (dummy_data and dummy_db) for clarity and maintainability.

## Contributing

Contributions to the rust-rest-microservice are welcome! Here's how you can contribute:

* Bug Reports: Notice a bug? Open an issue with a clear title and description.
* Feature Requests: Have an idea for an improvement? I'd love to hear it. Open an issue to suggest new features.
* Pull Requests: Want to contribute directly? Fork the repository, make your changes, and submit a pull request.

## License 

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

For any questions or feedback, please contact resitberkaybozkurt@gmail.com.

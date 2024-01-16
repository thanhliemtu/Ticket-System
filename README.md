# Rust Axum Ticket System

This is a simple ticket system built using the Rust Axum framework. The system provides endpoints for user authentication, ticket creation, listing all tickets, and deleting tickets.

## Routes

### 1. Authentication

#### POST /api/login
- Handles user authentication through a POST request.
- Hardcoded username and password for now.
- Upon successful login, a cookie is assigned to the user.

### 2. Ticket Management

#### POST /api/tickets
- Creates a new ticket through a POST request.
- Requires authentication (cookie check).
  
#### GET /api/tickets
- Retrieves a list of all tickets on the server.
- Requires authentication (cookie check).

#### DELETE /api/tickets/:id
- Deletes a ticket with the specified ID through a DELETE request.
- Requires authentication (cookie check).

## Middleware

- All endpoints under "/api/tickets" are layered with a middleware that ensures authentication.
- Authentication is verified by checking for the existence of a valid cookie, which is obtained after a successful login from "/api/login".

## Error Handling

All routes in the system are equipped with a middleware that maps server errors to client errors. This middleware ensures that confidential information is logged on the server side without revealing any sensitive details to clients.

### Implementation Details

- The error handling middleware intercepts server errors and transforms them into client-friendly error responses.
- Confidential information is logged on the server for debugging purposes without exposing it to clients.

### Logging

- Server-side logs capture detailed information about errors for troubleshooting.
- Ensure that sensitive information is appropriately redacted or masked in the logs.


## Tutorial

For a detailed tutorial on building this ticket system with Rust Axum, check out the video tutorial:

[![Rust Axum Tutorial](https://img.youtube.com/vi/XZtlD_m59sM/0.jpg)](https://www.youtube.com/watch?v=XZtlD_m59sM)

## Original Repository

The original repository for this project can be found on [GitHub](https://github.com/jeremychone-channel/rust-axum-course).

## License

This project is dual-licensed under the MIT License and the Apache License 2.0. See the [MIT LICENSE](https://github.com/thanhliemtu/rust-axum-intro/blob/main/LICENSE-MIT) and [APACHE LICENSE](https://github.com/thanhliemtu/rust-axum-intro/blob/main/LICENSE-APACHE) files for details.



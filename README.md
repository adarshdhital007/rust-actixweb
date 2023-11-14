# Actix Web Simple App

This is a simple web application using Actix Web, a powerful framework for building web applications in Rust.

## Features

- **Hello World**: A basic "Hello World!" endpoint at the root (`/`).
- **Echo Endpoint**: A POST endpoint (`/echo`) that echoes back the received request body.
```bash
  curl -X POST -d "Echoed..." http://localhost:8080/echo
  ```
- **Manual Hello**: An example of a manually implemented endpoint (`/hey`) responding with "Hey there!".
- **Static Files Serving**:
    - Static files are served from the `/static` route.
    - Index HTML file is served at `/index.html`.
    - Demo HTML file is served at `/demo.html`.


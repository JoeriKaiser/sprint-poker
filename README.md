# Project Overview

This project is a comprehensive full-stack web application that integrates a robust backend built with Actix-web and SQLx for Rust, and a dynamic frontend developed using Dioxus. This combination ensures a high-performance, scalable, and user-friendly experience.

## Backend

At the core of our application, the backend is engineered with Actix-web, a high-speed web framework for Rust that emphasizes pragmatism and power. It utilizes SQLx to ensure safe and efficient database interactions. Designed to serve RESTful APIs, the backend supports the frontend with features like user authentication, seamless database migrations, and structured logging, all aimed at enhancing maintainability and scalability.

### Running the Backend Locally
To initiate the backend on your local environment, please proceed to the `api/shuttle/src` directory. From there, execute the following commands:
1. Ensure you have Rust and Cargo installed on your system. If not, visit https://www.rust-lang.org/tools/install to install them.
2. Navigate to the `api/shuttle/src` directory in your terminal.
3. Run `cargo shuttle run` to start the backend server. This command compiles the project and runs the resulting binary. The server will start, and you should see output indicating that the database has successfully started.
4. To verify that the backend is running correctly, you can send a request to `http://localhost:8080/api/health` using a tool like `curl` or Postman. You should receive a response indicating that the service is up and running.

### Useful Commands

`cargo shuttle run`: Run the project locally.
`cargo shuttle deploy`: Deploy the project to the cloud.
`cargo shuttle logs`: Display the logs of a deployment.
`cargo shuttle status`: Display the status of the service.
`cargo shuttle project status`: Display the status of the project.
`cargo shuttle project list`: Display a list of projects and their current status.
`cargo shuttle project restart`: Restart a project. Useful when you need to upgrade the version of your Shuttle dependencies.
`cargo shuttle resource list`: Display a list of resources and their current status. Useful to see connection strings and other information about the resources used by the project.


## Frontend

The frontend of our application is crafted with Dioxus, an innovative framework for building user interfaces with Rust. Dioxus leverages the power of Rust and WebAssembly to deliver high-performance, safe, and reactive web applications. It allows developers to write UI code in a declarative style, similar to React, but with the added benefits of Rust's performance and safety features.

Our frontend architecture is modular, with components for each part of the application, ensuring a clean and maintainable codebase. The use of Dioxus also enables seamless integration with the backend, providing a smooth user experience. The frontend supports dynamic content rendering, state management, and interacts with the backend to fetch and display data in real-time.

### Key Features:
- **High Performance:** By compiling to WebAssembly, our frontend benefits from the speed and efficiency of Rust, providing a fast and responsive user interface.
- **Safety:** Rust's strong type system and ownership model eliminate common bugs found in other languages, making our frontend robust and secure.
- **Reactive UI:** Dioxus's reactive model allows for dynamic UI updates in response to state changes, ensuring a seamless user experience.
- **Modular Design:** Our frontend is built with reusable components, making it easy to maintain and extend the application.

Overall, the frontend is designed to offer a modern, efficient, and user-friendly interface that complements our powerful backend, creating a cohesive and scalable full-stack web application.

### Running the Frontend Locally

To run the frontend locally, follow these steps:
1. Ensure you have the necessary tools installed:
   - Rust Target `rustup target add wasm32-unknown-unknown`
   - Dioxus cli: Install it using `cargo install dioxus-cli`.
2. Navigate to the `front` directory in your terminal.
3. Run `npm install` to install the project dependencies.
4. Execute `dx serve --port 8080` to start the development server. This command compiles the Rust code to WebAssembly and serves it on a local web server.
5. In addition to starting the development server, you might want to launch the tailwind compiler using
`npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch`
6. Open your web browser and go to `http://localhost:8080` to view the frontend application. You should see the user interface rendered, indicating that the frontend is running correctly.

By following these steps, you can work on both the backend and frontend of the application simultaneously, allowing for a seamless full-stack development experience.

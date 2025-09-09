# Todo App

A full-stack todo application built with Rust, featuring a backend API and a frontend web application.

## Architecture

- **Backend**: Rust with Axum web framework, PostgreSQL database, JWT authentication
- **Frontend**: Rust with Yew framework, compiled to WebAssembly

## Prerequisites

- Rust (latest stable version)
- PostgreSQL database
- Cargo (comes with Rust)

## Setup Instructions

### 1. Clone the repository

```bash
git clone <your-repo-url>
cd todo-app
```

### 2. Backend Setup

1. Navigate to the backend directory:
   ```bash
   cd backend
   ```

2. Copy the environment example file and configure it:
   ```bash
   cp .env.example .env
   ```

3. Edit the `.env` file with your actual configuration:
   ```env
   DATABASE_URL=postgres://username:password@localhost:5432/todo_app
   JWT_SECRET=your-super-secret-jwt-key-here
   ```

   **Important**: Generate a strong JWT secret using:
   ```bash
   openssl rand -base64 32
   ```

4. Create the PostgreSQL database:
   ```sql
   CREATE DATABASE todo_app;
   ```

5. Run the backend:
   ```bash
   cargo run
   ```

   The backend will be available at `http://127.0.0.1:3001`

### 3. Frontend Setup

1. Navigate to the frontend directory:
   ```bash
   cd frontend
   ```

2. Install wasm-pack (if not already installed):
   ```bash
   cargo install wasm-pack
   ```

3. Build the frontend:
   ```bash
   wasm-pack build --target web --out-dir dist
   ```

4. Serve the frontend:
   ```bash
   # Using Python (if available)
   python -m http.server 8000
   
   # Or using Node.js (if available)
   npx serve .
   
   # Or any other static file server
   ```

   The frontend will be available at `http://localhost:8000`

## API Endpoints

### Authentication
- `POST /api/register` - Register a new user
- `POST /api/login` - Login with email and password

### Todos (requires authentication)
- `GET /api/todos` - Get all todos for the authenticated user
- `POST /api/todos` - Create a new todo
- `PATCH /api/todos/:id` - Update a todo
- `DELETE /api/todos/:id` - Delete a todo

## Security Features

- JWT-based authentication
- Password hashing with bcrypt
- CORS enabled for cross-origin requests
- Environment variable configuration for sensitive data

## Development

### Running in Development Mode

1. Start the backend:
   ```bash
   cd backend
   cargo run
   ```

2. In another terminal, start the frontend:
   ```bash
   cd frontend
   wasm-pack build --target web --out-dir dist --dev
   python -m http.server 8000
   ```

### Building for Production

1. Backend:
   ```bash
   cd backend
   cargo build --release
   ```

2. Frontend:
   ```bash
   cd frontend
   wasm-pack build --target web --out-dir dist --release
   ```

## Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `DATABASE_URL` | PostgreSQL connection string | Yes |
| `JWT_SECRET` | Secret key for JWT token signing | Yes |

## License

This project is open source and available under the MIT License.

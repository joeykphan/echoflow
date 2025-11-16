# Alm - Backend

Rust backend API for personal finance budgeting and tracking application.

## Features

- User authentication with JWT
- Plaid integration for automatic transaction sync
- Account management
- Transaction tracking and categorization
- Budget creation and monitoring
- Analytics and reporting
- RESTful API

## Tech Stack

- **Language**: Rust
- **Web Framework**: Axum
- **Database**: PostgreSQL with SQLx
- **Authentication**: JWT with bcrypt
- **API Integration**: Plaid

## Prerequisites

- Rust 1.70+
- PostgreSQL 14+
- Plaid API credentials (get free sandbox credentials at <https://dashboard.plaid.com/signup>)

## Setup

1. Install dependencies:

```bash
cargo build
```

1. Create PostgreSQL database:

```bash
createdb financeapp
```

1. Copy `.env.example` to `.env` and update values:

```bash
cp .env.example .env
```

1. Run migrations:

```bash
sqlx migrate run
```

1. Start the server:

```bash
cargo run
```

The API will be available at `http://localhost:8000`

## API Endpoints

### Authentication

- `POST /api/auth/register` - Register new user
- `POST /api/auth/login` - Login user

### Plaid Integration

- `POST /api/plaid/link-token` - Create Plaid Link token
- `POST /api/plaid/exchange-token` - Exchange public token for access token
- `POST /api/plaid/sync` - Sync transactions from Plaid

### Accounts

- `GET /api/accounts` - List all accounts
- `GET /api/accounts/:id` - Get account details
- `DELETE /api/accounts/:id` - Delete account

### Transactions

- `GET /api/transactions` - List transactions (with filters)
- `GET /api/transactions/:id` - Get transaction details
- `POST /api/transactions` - Create manual transaction
- `PUT /api/transactions/:id` - Update transaction
- `DELETE /api/transactions/:id` - Delete transaction

### Categories

- `GET /api/categories` - List all categories
- `GET /api/categories/:id` - Get category details
- `POST /api/categories` - Create custom category
- `PUT /api/categories/:id` - Update category
- `DELETE /api/categories/:id` - Delete category

### Budgets

- `GET /api/budgets` - List all budgets
- `GET /api/budgets/:id` - Get budget details
- `POST /api/budgets` - Create budget
- `PUT /api/budgets/:id` - Update budget
- `DELETE /api/budgets/:id` - Delete budget
- `GET /api/budgets/:id/performance` - Get budget performance

### Analytics

- `GET /api/analytics/net-worth` - Get total net worth
- `GET /api/analytics/spending-by-category?start_date=YYYY-MM-DD&end_date=YYYY-MM-DD` - Spending breakdown
- `GET /api/analytics/income-over-time?start_date=YYYY-MM-DD&end_date=YYYY-MM-DD` - Income trends
- `GET /api/analytics/spending-over-time?start_date=YYYY-MM-DD&end_date=YYYY-MM-DD` - Spending trends

## Database Schema

### Tables

- `users` - User accounts
- `plaid_items` - Plaid connected institutions
- `accounts` - Bank/financial accounts
- `transactions` - Financial transactions
- `categories` - Transaction categories
- `budgets` - User budgets

## Development

### Run tests

```bash
cargo test
```

### Format code

```bash
cargo fmt
```

### Lint

```bash
cargo clippy
```

### Watch mode

```bash
cargo watch -x run
```

## Security

- Passwords are hashed using bcrypt
- JWT tokens expire after 24 hours
- Plaid access tokens are stored encrypted
- All endpoints (except auth) require authentication
- CORS enabled for frontend integration

## Environment Variables

- `DATABASE_URL` - PostgreSQL connection string
- `JWT_SECRET` - Secret key for JWT signing
- `PLAID_CLIENT_ID` - Plaid API client ID
- `PLAID_SECRET` - Plaid API secret
- `PLAID_ENV` - Plaid environment (sandbox/development/production)

## Production Deployment

1. Set `PLAID_ENV=production` in environment
2. Use strong `JWT_SECRET`
3. Enable HTTPS
4. Set up database backups
5. Configure rate limiting
6. Enable logging and monitoring

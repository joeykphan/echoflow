# Finance Budgeting App - Technical Specification

## Tech Stack

- **Backend**: Rust (Axum web framework)
- **Frontend**: Next.js 14+ (TypeScript, React)
- **Database**: PostgreSQL
- **API Integration**: Plaid API
- **Styling**: Tailwind CSS + shadcn/ui components
- **Charts**: Recharts
- **State Management**: React Context + hooks

## Core Features

### 1. Account Management

- Plaid Link integration for secure bank connections
- Multi-account syncing
- Real-time balance tracking
- Net worth calculation across all accounts

### 2. Transaction Management

- Automatic transaction imports via Plaid
- Manual transaction entry
- Transaction categorization (income, groceries, health, housing, utilities, subscriptions, etc.)
- Quick categorization UI with keyboard shortcuts
- Bulk categorization
- Search and filter transactions

### 3. Budgeting System

- Monthly budget creation by category
- Budget vs. actual spending comparison
- Budget rollover options
- Alerts for approaching/exceeding limits
- Daily, weekly, monthly, annual views

### 4. Analytics & Visualizations

- **Line Charts**: Income over time, spending over time, net worth trends
- **Pie Charts**: Spending by category, income sources
- **Bar Charts**: Monthly comparisons, category trends
- **Custom date ranges**: Daily, weekly, monthly, yearly
- Export reports to PDF/CSV

### 5. Security

- JWT authentication
- Encrypted storage for sensitive data
- Plaid security best practices
- HTTPS only

## Project Structure

``` fs
alms/
├── alm/
│   ├── src/
│   │   ├── main.rs
│   │   ├── api/
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs
│   │   │   ├── accounts.rs
│   │   │   ├── transactions.rs
│   │   │   ├── budgets.rs
│   │   │   └── analytics.rs
│   │   ├── db/
│   │   │   ├── mod.rs
│   │   │   └── models.rs
│   │   ├── plaid/
│   │   │   ├── mod.rs
│   │   │   └── client.rs
│   │   └── utils/
│   │       ├── mod.rs
│   │       └── auth.rs
│   ├── migrations/
│   └── Cargo.toml
│
├── celica/
│   ├── src/
│   │   ├── app/
│   │   │   ├── layout.tsx
│   │   │   ├── page.tsx
│   │   │   ├── dashboard/
│   │   │   ├── transactions/
│   │   │   ├── budgets/
│   │   │   ├── analytics/
│   │   │   └── settings/
│   │   ├── components/
│   │   │   ├── ui/
│   │   │   ├── charts/
│   │   │   ├── transactions/
│   │   │   └── budgets/
│   │   ├── lib/
│   │   │   ├── api.ts
│   │   │   ├── plaid.ts
│   │   │   └── utils.ts
│   │   └── hooks/
│   ├── public/
│   ├── package.json
│   └── tsconfig.json
│
└── README.md
```

## Database Schema

### users

- id (UUID, PK)
- email (VARCHAR, UNIQUE)
- password_hash (VARCHAR)
- created_at (TIMESTAMP)
- updated_at (TIMESTAMP)

### accounts

- id (UUID, PK)
- user_id (UUID, FK)
- plaid_account_id (VARCHAR)
- plaid_item_id (VARCHAR)
- account_name (VARCHAR)
- account_type (VARCHAR)
- balance (DECIMAL)
- currency (VARCHAR)
- last_synced (TIMESTAMP)
- created_at (TIMESTAMP)

### transactions

- id (UUID, PK)
- account_id (UUID, FK)
- plaid_transaction_id (VARCHAR, UNIQUE)
- date (DATE)
- amount (DECIMAL)
- description (VARCHAR)
- category_id (UUID, FK)
- merchant_name (VARCHAR)
- pending (BOOLEAN)
- created_at (TIMESTAMP)
- updated_at (TIMESTAMP)

### categories

- id (UUID, PK)
- user_id (UUID, FK, NULLABLE)
- name (VARCHAR)
- type (ENUM: income, expense)
- color (VARCHAR)
- icon (VARCHAR)
- is_default (BOOLEAN)

### budgets

- id (UUID, PK)
- user_id (UUID, FK)
- category_id (UUID, FK)
- amount (DECIMAL)
- period (ENUM: monthly, yearly)
- start_date (DATE)
- end_date (DATE)
- created_at (TIMESTAMP)

### plaid_items

- id (UUID, PK)
- user_id (UUID, FK)
- plaid_access_token (VARCHAR, ENCRYPTED)
- plaid_item_id (VARCHAR)
- institution_id (VARCHAR)
- institution_name (VARCHAR)
- status (VARCHAR)
- created_at (TIMESTAMP)

## Implementation Phases

### Phase 1: Backend Foundation

1. Setup Rust project with Axum
2. Configure PostgreSQL connection
3. Implement authentication (JWT)
4. Create database migrations
5. Setup Plaid SDK integration

### Phase 2: Core Backend API

1. User registration/login endpoints
2. Plaid Link token generation
3. Account connection via Plaid
4. Transaction sync from Plaid
5. CRUD operations for categories
6. CRUD operations for budgets

### Phase 3: Frontend Foundation

1. Setup Next.js project
2. Install dependencies (shadcn/ui, Recharts)
3. Create layout and navigation
4. Implement authentication UI
5. Setup API client

### Phase 4: Core Frontend Features

1. Dashboard with net worth overview
2. Plaid Link integration
3. Account list and management
4. Transaction list with filtering
5. Quick categorization UI
6. Budget creation and management

### Phase 5: Analytics & Visualizations

1. Spending by category (pie chart)
2. Income/spending over time (line chart)
3. Monthly comparisons (bar chart)
4. Net worth trends
5. Custom date range selector

### Phase 6: Polish & Optimization

1. Loading states and error handling
2. Responsive design
3. Performance optimization
4. Security audit
5. Testing

## Key API Endpoints

``` openapi
POST   /api/auth/register
POST   /api/auth/login
GET    /api/auth/me

POST   /api/plaid/link-token
POST   /api/plaid/exchange-token
GET    /api/plaid/sync

GET    /api/accounts
GET    /api/accounts/:id
DELETE /api/accounts/:id

GET    /api/transactions
GET    /api/transactions/:id
PUT    /api/transactions/:id
POST   /api/transactions
DELETE /api/transactions/:id

GET    /api/categories
POST   /api/categories
PUT    /api/categories/:id
DELETE /api/categories/:id

GET    /api/budgets
POST   /api/budgets
PUT    /api/budgets/:id
DELETE /api/budgets/:id
GET    /api/budgets/:id/performance

GET    /api/analytics/net-worth
GET    /api/analytics/spending-by-category
GET    /api/analytics/income-over-time
GET    /api/analytics/spending-over-time
```

## Quick Categorization UI Concept

- Display uncategorized transactions at top
- Keyboard shortcuts (1-9 for top categories)
- Swipe gestures on mobile
- Bulk select and categorize
- ML-based category suggestions (future enhancement)
- Recent categories quick access

## Environment Variables

```env
# Backend
DATABASE_URL=postgresql://user:password@localhost/financeapp
JWT_SECRET=your-secret-key
PLAID_CLIENT_ID=your-plaid-client-id
PLAID_SECRET=your-plaid-secret
PLAID_ENV=sandbox # sandbox, development, production

# Frontend
NEXT_PUBLIC_API_URL=http://localhost:8000
```

## Getting Started Commands

```bash
# Backend
cd backend
cargo run

# Frontend
cd frontend
npm install
npm run dev
```

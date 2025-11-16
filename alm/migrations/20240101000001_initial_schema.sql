-- migrations/20240101000001_initial_schema.sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
email VARCHAR(255) UNIQUE NOT NULL,
password_hash VARCHAR(255) NOT NULL,
created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);

CREATE TABLE plaid_items (
id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
plaid_access_token TEXT NOT NULL,
plaid_item_id VARCHAR(255) NOT NULL,
institution_id VARCHAR(255) NOT NULL,
institution_name VARCHAR(255) NOT NULL,
status VARCHAR(50) NOT NULL DEFAULT 'active',
created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_plaid_items_user_id ON plaid_items(user_id);
CREATE INDEX idx_plaid_items_plaid_item_id ON plaid_items(plaid_item_id);

CREATE TABLE accounts (
id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
plaid_account_id VARCHAR(255),
plaid_item_id VARCHAR(255),
account_name VARCHAR(255) NOT NULL,
account_type VARCHAR(100) NOT NULL,
balance DECIMAL(15, 2) NOT NULL DEFAULT 0,
currency VARCHAR(10) NOT NULL DEFAULT 'USD',
last_synced TIMESTAMP WITH TIME ZONE,
created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_accounts_user_id ON accounts(user_id);
CREATE INDEX idx_accounts_plaid_account_id ON accounts(plaid_account_id);

CREATE TABLE categories (
id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
user_id UUID REFERENCES users(id) ON DELETE CASCADE,
name VARCHAR(100) NOT NULL,
category_type VARCHAR(20) NOT NULL CHECK (category_type IN ('income', 'expense')),
color VARCHAR(20) NOT NULL,
icon VARCHAR(50),
is_default BOOLEAN NOT NULL DEFAULT false
);

CREATE INDEX idx_categories_user_id ON categories(user_id);
CREATE INDEX idx_categories_is_default ON categories(is_default);

CREATE TABLE transactions (
id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
plaid_transaction_id VARCHAR(255) UNIQUE,
date DATE NOT NULL,
amount DECIMAL(15, 2) NOT NULL,
description TEXT NOT NULL,
category_id UUID REFERENCES categories(id) ON DELETE SET NULL,
merchant_name VARCHAR(255),
pending BOOLEAN NOT NULL DEFAULT false,
created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_transactions_account_id ON transactions(account_id);
CREATE INDEX idx_transactions_category_id ON transactions(category_id);
CREATE INDEX idx_transactions_date ON transactions(date);
CREATE INDEX idx_transactions_plaid_transaction_id ON transactions(plaid_transaction_id);

CREATE TABLE budgets (
id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
category_id UUID NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
amount DECIMAL(15, 2) NOT NULL,
period VARCHAR(20) NOT NULL CHECK (period IN ('daily', 'weekly', 'monthly', 'yearly')),
start_date DATE NOT NULL,
end_date DATE,
created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_budgets_user_id ON budgets(user_id);
CREATE INDEX idx_budgets_category_id ON budgets(category_id);
CREATE INDEX idx_budgets_start_date ON budgets(start_date);

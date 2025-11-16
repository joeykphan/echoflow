-- migrations/20240101000002_default_categories.sql
INSERT INTO categories (id, user_id, name, category_type, color, icon, is_default) VALUES
(uuid_generate_v4(), NULL, 'Income', 'income', '#10b981', '💰', true),
(uuid_generate_v4(), NULL, 'Salary', 'income', '#10b981', '💼', true),
(uuid_generate_v4(), NULL, 'Investments', 'income', '#10b981', '📈', true),
(uuid_generate_v4(), NULL, 'Groceries', 'expense', '#ef4444', '🛒', true),
(uuid_generate_v4(), NULL, 'Dining Out', 'expense', '#f97316', '🍽️', true),
(uuid_generate_v4(), NULL, 'Housing', 'expense', '#8b5cf6', '🏠', true),
(uuid_generate_v4(), NULL, 'Utilities', 'expense', '#3b82f6', '💡', true),
(uuid_generate_v4(), NULL, 'Transportation', 'expense', '#06b6d4', '🚗', true),
(uuid_generate_v4(), NULL, 'Health', 'expense', '#ec4899', '⚕️', true),
(uuid_generate_v4(), NULL, 'Entertainment', 'expense', '#f59e0b', '🎬', true),
(uuid_generate_v4(), NULL, 'Shopping', 'expense', '#14b8a6', '🛍️', true),
(uuid_generate_v4(), NULL, 'Subscriptions', 'expense', '#6366f1', '📱', true),
(uuid_generate_v4(), NULL, 'Insurance', 'expense', '#84cc16', '🛡️', true),
(uuid_generate_v4(), NULL, 'Personal Care', 'expense', '#a855f7', '💅', true),
(uuid_generate_v4(), NULL, 'Education', 'expense', '#0ea5e9', '📚', true),
(uuid_generate_v4(), NULL, 'Travel', 'expense', '#22c55e', '✈️', true),
(uuid_generate_v4(), NULL, 'Other', 'expense', '#64748b', '📦', true);

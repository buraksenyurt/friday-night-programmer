-- Todo durumu için enum tipi
CREATE TYPE todo_status AS ENUM ('done', 'undone', 'inprogress');

-- Todo zorluk derecesi için enum tipi
CREATE TYPE todo_difficulty AS ENUM ('easy', 'medium', 'hard');

-- Todos tablosu
CREATE TABLE IF NOT EXISTS todos (
    id          UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    title       VARCHAR(500) NOT NULL,
    status      todo_status  NOT NULL DEFAULT 'undone',
    difficulty  todo_difficulty NOT NULL DEFAULT 'medium',
    deadline    TIMESTAMPTZ,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

-- Performans için indeks
CREATE INDEX IF NOT EXISTS idx_todos_status ON todos(status);
CREATE INDEX IF NOT EXISTS idx_todos_deadline ON todos(deadline) WHERE deadline IS NOT NULL;

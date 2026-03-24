-- =============================================
-- SQLite-compatible initialization script
-- =============================================

-- 1. Actors table
CREATE TABLE IF NOT EXISTS actor_table (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    email       TEXT UNIQUE NOT NULL,
    about       TEXT,
    cooperation TEXT
);

-- 2. Audio files table
CREATE TABLE IF NOT EXISTS audio_file (
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    name    TEXT,
    tags    TEXT NOT NULL DEFAULT '[]',     -- Store as JSON array string
    path    TEXT,
    owner   INTEGER NOT NULL,
    FOREIGN KEY (owner) REFERENCES actor_table(id) ON DELETE CASCADE
);

-- 3. Image files table
CREATE TABLE IF NOT EXISTS image_file (
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    name    TEXT,
    tags    TEXT NOT NULL DEFAULT '[]',     -- Store as JSON array string
    path    TEXT,
    owner   INTEGER NOT NULL,
    FOREIGN KEY (owner) REFERENCES actor_table(id) ON DELETE CASCADE
);

-- Optional: Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_audio_owner ON audio_file(owner);
CREATE INDEX IF NOT EXISTS idx_image_owner ON image_file(owner);

-- =============================================
-- Insert Sample Data
-- =============================================

-- Actors
INSERT OR IGNORE INTO actor_table (email, about, cooperation) VALUES
('anna.kowalczyk@voiceworks.pl', 
 'Professional Polish narrator and dubbing actress. Specialized in children’s books, commercials and corporate e-learning.', 
 'Open to long-term audiobook projects, prefers Polish and English'),

('michal.nowak.vo@gmail.com', 
 'Male voice-over artist, 32 y/o, warm & friendly tone, experienced in radio ads, IVR and documentaries.', 
 'Available for remote recording, has own treated booth'),

('sara.lindberg@sevoices.com', 
 'Swedish–English bilingual voice talent. Clear neutral accent, often booked for tech explainers and e-learning.', 
 'Fixed fee per project or hourly rate – please ask for current reel');

-- Audio files (tags stored as JSON array string)
INSERT OR IGNORE INTO audio_file (name, tags, path, owner) VALUES
('Anna – Children Book Sample', '["polish","children","storybook","female"]', '/samples/anna/child-book-demo-2025.mp3', 1),
('Anna – Corporate Narration',   '["polish","corporate","e-learning","neutral"]', '/samples/anna/corp-2026-v2.mp3', 1),
('Michał – Radio Spot 30s',      '["polish","commercial","radio","male","energetic"]', '/demos/michal/radio-30s-2025.mp3', 2);

-- Image files
INSERT OR IGNORE INTO image_file (name, tags, path, owner) VALUES
('Anna – Studio Portrait 2025', '["portrait","female","studio","polish"]', '/photos/anna/studio-2025.jpg', 1),
('Michał – Booth Setup',        '["studio","booth","acoustic"]', '/photos/michal/booth-2024.jpg', 2);

-- Verify
SELECT 'Actors:' as info, COUNT(*) FROM actor_table
UNION ALL
SELECT 'Audio files:', COUNT(*) FROM audio_file
UNION ALL
SELECT 'Image files:', COUNT(*) FROM image_file;
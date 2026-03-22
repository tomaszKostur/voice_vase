-- =============================================
-- Sample data for your voice/media related tables
-- =============================================

-- 1. Actors (voice actors, narrators, producers, sound designers etc.)
INSERT INTO actor_table (email, about, cooperation) VALUES
('anna.kowalczyk@voiceworks.pl', 
 'Professional Polish narrator and dubbing actress. Specialized in children’s books, commercials and corporate e-learning.', 
 'Open to long-term audiobook projects, prefers Polish and English'),

('michal.nowak.vo@gmail.com', 
 'Male voice-over artist, 32 y/o, warm & friendly tone, experienced in radio ads, IVR and documentaries.', 
 'Available for remote recording, has own treated booth'),

('sara.lindberg@sevoices.com', 
 'Swedish–English bilingual voice talent. Clear neutral accent, often booked for tech explainers and e-learning.', 
 'Fixed fee per project or hourly rate – please ask for current reel'),

('tomasz.dabrowski@proaudio.pl', 
 'Sound designer & Foley artist with 8+ years in advertising and short films.', 
 'Cooperation mainly via agencies, open to interesting indie projects'),

('julia.rosa.mendes@gmail.com', 
 'Brazilian Portuguese voice actress living in Lisbon. Very versatile – from youthful to mature characters.', 
 'Loves animation and character work, has home studio'),

('david.reed@voicetalent.uk', 
 'British RP voice, newsreader style, also comfortable with northern & cockney accents on request.', 
 'High demand in Q4 – booking 6–10 weeks ahead');

-- 2. Audio files (samples owned by the actors above)
INSERT INTO audio_file (name, tags, path, owner) VALUES
-- Anna
('Anna – Children Book Sample', ARRAY['polish','children','storybook','female'], '/samples/anna/child-book-demo-2025.mp3', 1),
('Anna – Corporate Narration',   ARRAY['polish','corporate','e-learning','neutral'], '/samples/anna/corp-2026-v2.mp3', 1),

-- Michał
('Michał – Radio Spot 30s',      ARRAY['polish','commercial','radio','male','energetic'], '/demos/michal/radio-30s-2025.mp3', 2),
('Michał – IVR Menu Example',     ARRAY['ivr','polish','professional','male'], '/ivr/michal/menu-pol-v3.wav', 2),

-- Sara
('Sara – Tech Explainer EN',      ARRAY['english','tech','explainer','female','neutral'], '/demos/sara/tech-en-2026.mp3', 3),
('Sara – Swedish Product Video',  ARRAY['swedish','product','promo','female'], '/samples/sara/se-promo-2025.mp3', 3),

-- Tomasz (sound design)
('Tomasz – Foley Pack City',      ARRAY['foley','urban','sound-design','ambience'], '/foley/tomasz/city-pack-01.wav', 4),
('Tomasz – Cinematic Whoosh Set', ARRAY['whoosh','trailer','cinematic','transition'], '/effects/tomasz/whoosh-pack-2025.wav', 4),

-- Julia
('Julia – Animation Character',   ARRAY['portuguese','brazil','character','female','youthful'], '/demos/julia/animation-br-2026.mp3', 5),

-- David
('David – News Intro Stinger',    ARRAY['british','rp','news','male','authoritative'], '/stingers/david/news-intro-2025.mp3', 6),
('David – Documentary Narration', ARRAY['english','documentary','calm','male'], '/narration/david/docu-sample-10min.mp3', 6);

-- 3. Image files (profile pictures, studio photos, waveform screenshots etc.)
INSERT INTO image_file (name, tags, path, owner) VALUES
-- Anna
('Anna – Studio Portrait 2025',      ARRAY['portrait','female','studio','polish'], '/photos/anna/studio-2025.jpg', 1),
('Anna – Microphone Close-up',       ARRAY['equipment','mic','recording'], '/gear/anna/neumann-u87.jpg', 1),

-- Michał
('Michał – Booth Setup',             ARRAY['studio','booth','acoustic','treatment'], '/photos/michal/booth-2024.jpg', 2),

-- Sara
('Sara – Professional Headshot',     ARRAY['headshot','professional','female'], '/portraits/sara/headshot-2026.jpg', 3),
('Sara – Swedish Recording Session', ARRAY['recording','session','sweden'], '/photos/sara/session-se-2025.jpg', 3),

-- Tomasz
('Tomasz – Foley Props',             ARRAY['foley','props','objects','sound-design'], '/studio/tomasz/foley-props-2025.jpg', 4),

-- Julia
('Julia – Animation Voice Session',  ARRAY['animation','character','recording'], '/photos/julia/animation-session-2026.jpg', 5),

-- David
('David – Voice Reel Cover',         ARRAY['cover','reel','british'], '/artwork/david/reel-cover-2025.jpg', 6),
('David – BBC Style Portrait',       ARRAY['portrait','bbc','news','male'], '/portraits/david/bbc-style-2024.jpg', 6);
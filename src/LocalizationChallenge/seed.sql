CREATE TABLE IF NOT EXISTS localizations (
    id           SERIAL       PRIMARY KEY,
    culture      VARCHAR(10)  NOT NULL,
    resource_key VARCHAR(255) NOT NULL,
    value        TEXT         NOT NULL,
    updated_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_culture_key UNIQUE (culture, resource_key)
);

CREATE INDEX IF NOT EXISTS idx_loc_culture_key ON localizations (culture, resource_key);

-- Sample data
INSERT INTO localizations (culture, resource_key, value) VALUES
  ('en-US', 'welcome_message',    'Welcome to our application'),
  ('en-US', 'farewell_message',   'Goodbye, see you soon'),
  ('en-US', 'error_not_found',    'The requested resource was not found'),
  ('en-US', 'error_unauthorized', 'You are not authorized to perform this action'),
  ('en-US', 'button_save',        'Save'),
  ('en-US', 'button_cancel',      'Cancel'),
  ('en-US', 'button_delete',      'Delete'),
  ('tr-TR', 'welcome_message',    'Uygulamamıza hoş geldiniz'),
  ('tr-TR', 'farewell_message',   'Güle güle, yakında görüşürüz'),
  ('tr-TR', 'error_not_found',    'İstenen kaynak bulunamadı'),
  ('tr-TR', 'error_unauthorized', 'Bu işlemi gerçekleştirme yetkiniz yok'),
  ('tr-TR', 'button_save',        'Kaydet'),
  ('tr-TR', 'button_cancel',      'İptal'),
  ('tr-TR', 'button_delete',      'Sil'),
  ('de-DE', 'welcome_message',    'Willkommen in unserer Anwendung'),
  ('de-DE', 'farewell_message',   'Auf Wiedersehen, bis bald'),
  ('de-DE', 'error_not_found',    'Die angeforderte Ressource wurde nicht gefunden'),
  ('de-DE', 'error_unauthorized', 'Sie sind nicht berechtigt, diese Aktion durchzuführen'),
  ('de-DE', 'button_save',        'Speichern'),
  ('de-DE', 'button_cancel',      'Abbrechen'),
  ('de-DE', 'button_delete',      'Löschen')
ON CONFLICT (culture, resource_key) DO NOTHING;
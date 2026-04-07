redis-cli -h localhost -p 6379 HSET loc:en-US \
  welcome_message    "Welcome to our application" \
  farewell_message   "Goodbye, see you soon" \
  error_not_found    "The requested resource was not found" \
  error_unauthorized "You are not authorized to perform this action" \
  button_save        "Save" \
  button_cancel      "Cancel" \
  button_delete      "Delete"

redis-cli -h localhost -p 6379 HSET loc:tr-TR \
  welcome_message    "Uygulamamıza hoş geldiniz" \
  farewell_message   "Güle güle, yakında görüşürüz" \
  error_not_found    "İstenen kaynak bulunamadı" \
  error_unauthorized "Bu işlemi gerçekleştirme yetkiniz yok" \
  button_save        "Kaydet" \
  button_cancel      "İptal" \
  button_delete      "Sil"

redis-cli -h localhost -p 6379 HSET loc:de-DE \
  welcome_message    "Willkommen in unserer Anwendung" \
  farewell_message   "Auf Wiedersehen, bis bald" \
  error_not_found    "Die angeforderte Ressource wurde nicht gefunden" \
  error_unauthorized "Sie sind nicht berechtigt, diese Aktion durchzuführen" \
  button_save        "Speichern" \
  button_cancel      "Abbrechen" \
  button_delete      "Löschen"

echo "Mission accomplished."
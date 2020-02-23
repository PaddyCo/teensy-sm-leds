#define BTN_PIN 5
#define LED_PIN 10
#define STATUS_PIN 13

// the setup routine runs once when you press reset:
void setup() {
  pinMode(BTN_PIN, INPUT);
  pinMode(LED_PIN, OUTPUT);
  pinMode(STATUS_PIN, OUTPUT);
}

// the loop routine runs over and over again forever:
void loop() {
  if (digitalRead(BTN_PIN) == HIGH) {
    digitalWrite(LED_PIN, HIGH);
    digitalWrite(STATUS_PIN, HIGH);
  } else {
    digitalWrite(LED_PIN, LOW);
    digitalWrite(STATUS_PIN, LOW);
  }
}

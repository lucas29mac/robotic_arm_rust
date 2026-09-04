#include <ESP32Servo.h>

const int NUM_SERVOS = 7;
const int pinos[NUM_SERVOS] = {18, 19, 22, 23, 5, 25, 26};

Servo servos[NUM_SERVOS];

// 🔒 limites individuais (ajustar conforme seu braço)
int min_angles[NUM_SERVOS] = {30, 30, 30, 30, 30, 30, 30};
int max_angles[NUM_SERVOS] = {100, 100, 100, 100, 100, 100, 100};

void setup() {
  Serial.begin(9600);

  for (int i = 0; i < NUM_SERVOS; i++) {
    servos[i].attach(pinos[i]);

    int inicial = (min_angles[i] + max_angles[i]) / 2;
    servos[i].write(inicial);
  }

  Serial.println("Sistema pronto");
}

void loop() {
  if (Serial.available()) {

    String cmd = Serial.readStringUntil('\n');

    int start = 0;

    // 🔥 processa múltiplos comandos no mesmo pacote
    while (start < cmd.length()) {

      int comma = cmd.indexOf(',', start);

      if (comma == -1) {
        comma = cmd.length();
      }

      String pair = cmd.substring(start, comma);

      int separador = pair.indexOf(':');

      if (separador != -1) {
        int id = pair.substring(0, separador).toInt();
        int angulo = pair.substring(separador + 1).toInt();

        if (id >= 0 && id < NUM_SERVOS) {

          // 🔒 proteção mecânica
          angulo = constrain(angulo, min_angles[id], max_angles[id]);

          servos[id].write(angulo);
        }
      }

      start = comma + 1;
    }

    // ✅ confirmação única
    Serial.println("OK");
  }
}
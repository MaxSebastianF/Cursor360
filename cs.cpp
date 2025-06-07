#include <windows.h>
#include <Xinput.h>
#include <iostream>
#include <thread>
#include <chrono>
#include "ViGEmClient.h"

// Variables globales para el estado del ratón
POINT lastMousePos;
float camX = 0.0f;
float camY = 0.0f;

// Sensibilidad y fricción
const float sensitivity = 3200.0f;
const float friction = 0.95f;

// Función para obtener el estado de una tecla
bool IsKeyPressed(int vKey) {
    return (GetAsyncKeyState(vKey) & 0x8000) != 0;
}

int main() {
    // Inicializar ViGEm
    PVIGEM_CLIENT client = vigem_alloc();
    if (vigem_connect(client) != VIGEM_ERROR_NONE) {
        std::cerr << "No se pudo conectar con ViGEmBus." << std::endl;
        return -1;
    }

    // Crear mando virtual
    PVIGEM_TARGET target = vigem_target_x360_alloc();
    if (vigem_target_add(client, target) != VIGEM_ERROR_NONE) {
        std::cerr << "No se pudo agregar el mando virtual." << std::endl;
        return -1;
    }

    // Obtener el centro de la pantalla
    int screenWidth = GetSystemMetrics(SM_CXSCREEN);
    int screenHeight = GetSystemMetrics(SM_CYSCREEN);
    int centerX = screenWidth / 2;
    int centerY = screenHeight / 2;

    // Confinar el cursor al área de la pantalla
    RECT screenRect = { 0, 0, screenWidth, screenHeight };
    ClipCursor(&screenRect);

    // Posicionar el cursor en el centro
    SetCursorPos(centerX, centerY);
    lastMousePos.x = centerX;
    lastMousePos.y = centerY;

    // Bucle principal
    while (true) {
        // Salir con Ctrl + C
        if (IsKeyPressed(VK_CONTROL) && IsKeyPressed(0x43)) {
            std::cout << "Ctrl + C detectado. Saliendo..." << std::endl;
            break;
        }

        // Obtener posición actual del ratón
        POINT currentPos;
        GetCursorPos(&currentPos);

        // Calcular deltas
        int deltaX = currentPos.x - lastMousePos.x;
        int deltaY = currentPos.y - lastMousePos.y;

        // Recentrar cursor
        SetCursorPos(centerX, centerY);
        lastMousePos.x = centerX;
        lastMousePos.y = centerY;

        // Actualizar movimiento de cámara
        camX += deltaX * sensitivity;
        camY += deltaY * sensitivity;

        camX *= friction;
        camY *= friction;

        // Crear estado del gamepad
        XUSB_REPORT report = {};
        report.sThumbRX = static_cast<SHORT>(max(min(camX, 32767.0f), -32768.0f));
        report.sThumbRY = static_cast<SHORT>(-max(min(camY, 32767.0f), -32768.0f));

        // Mapear teclas a botones
        if (IsKeyPressed('W')) report.sThumbLY = 32767;
        if (IsKeyPressed('S')) report.sThumbLY = -32768;
        if (IsKeyPressed('A')) report.sThumbLX = -32768;
        if (IsKeyPressed('D')) report.sThumbLX = 32767;

        if (IsKeyPressed(VK_SPACE)) report.wButtons |= XUSB_GAMEPAD_A;
        if (IsKeyPressed('F')) report.wButtons |= XUSB_GAMEPAD_B;
        if (IsKeyPressed('E')) report.wButtons |= XUSB_GAMEPAD_X;
        if (IsKeyPressed('T')) report.wButtons |= XUSB_GAMEPAD_Y;
        if (IsKeyPressed('Q')) report.wButtons |= XUSB_GAMEPAD_LEFT_SHOULDER;
        if (IsKeyPressed('R')) report.wButtons |= XUSB_GAMEPAD_RIGHT_SHOULDER;
        if (IsKeyPressed('B')) report.wButtons |= XUSB_GAMEPAD_START;
        if (IsKeyPressed(VK_ESCAPE)) report.wButtons |= XUSB_GAMEPAD_BACK;

        // Enviar estado al mando virtual
        vigem_target_x360_update(client, target, report);

        // Pequeña pausa para evitar uso excesivo de CPU
        std::this_thread::sleep_for(std::chrono::milliseconds(1));
    }

    // Liberar recursos
    vigem_target_remove(client, target);
    vigem_target_free(target);
    vigem_free(client);

    return 0;
}

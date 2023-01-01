#pragma once

#include <SDL2/SDL.h>
#include "../FractalData.h"

namespace Fractal::Rendering
{
    class FractalRenderer
    {
    private:
        SDL_Window *m_window;
        SDL_Renderer *m_renderer;

    public:
        FractalRenderer(int width, int height);
        ~FractalRenderer();

        void Render(FractalData *data);
    };
}
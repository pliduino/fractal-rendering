#include "FractalRenderer.h"

namespace Fractal::Rendering
{
    FractalRenderer::FractalRenderer(int width, int height)
    {
        SDL_Init(SDL_INIT_EVERYTHING);
        m_window = SDL_CreateWindow("title", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, width, height, SDL_WINDOW_SHOWN);
        m_renderer = SDL_CreateRenderer(m_window, -1, 0);
    }

    FractalRenderer::~FractalRenderer()
    {
        free(m_window);
        free(m_renderer);
    }

    void FractalRenderer::Render(FractalData *data)
    {
        SDL_SetRenderDrawColor(m_renderer, 255, 0, 0, SDL_ALPHA_OPAQUE);

        SDL_RenderClear(m_renderer);

        SDL_RenderPresent(m_renderer);
    }
}
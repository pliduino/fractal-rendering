#include "Generation/IFractalGenerator.h"
#include "Generation/MandelbrotGenerator.h"
#include "Rendering/FractalRenderer.h"

using namespace Fractal;

int main(int argc, char const *argv[])
{
    const int width = 600, height = 400;

    IFractalGenerator *fractalGenerator = new MandelbrotGenerator();

    // Rendering::FractalRenderer *renderer = new Rendering::FractalRenderer(width, height);

    FractalData *data = fractalGenerator->Generate();

    // renderer->Render(data);

    return 0;
}
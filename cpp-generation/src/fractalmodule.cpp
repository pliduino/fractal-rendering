#define PY_SSIZE_T_CLEAN
#include <pybind11.h>
#include <cmath>

int calc_mandelbrot(double x, double y, int iterations, double escape_constant)
{
    const double[] constant = [ x, y ];
    double[] next_z = [ x, y ];

    for (int i; i < iterations; i++)
    {
        // Z squared
        next_z[0] = (next_z[0] * next_z[0]) - (next_z[1] * next_z[1]);
        next_z[1] = 2 * next_z[0] * next_z[1];

        value = sqrt((next_z[0] * next_z[0]) + (next_z[1] * next_z[1]));

        if (value > escape_constant)
        {
            return i;
        }

        next_z[0] += constant[0];
        next_z[1] += constant[1];
    }

    return iterations;
}

double *generate_mandelbrot(int imgSize, int iterations, double[2] offset, double step, double escape_constant)
{
    double *texture_data = malloc(size * size * 4);
    c = 1;

    for (int i; i < imgSize * imgSize; i++)
    {
        x = (i % imgSize - (imgSize / 2)) * step + offset[0];
        y = (floor(i / imgSize) - (imgSize / 2)) * step + offset[1];

        escape_time = calc_mandelbrot(x, y, iterations, escape_constant);

        double red,
            green, blue;

        if (escape_time > iterations / 2)
        {
            double factor = (iterations - 1) - (escape_time) / (iterations - 1);
            factor *= 2;

            red = factor * (200 / 255);
            green = factor * (25 / 255);
            blue = factor * (25 / 255);
        }
        else
        {
            double factor = (iterations - 1) - (escape_time) / (iterations - 1);
            factor = (factor - 0.5) * 2;

            red = (200 / 255);
            green = (25 / 255);
            blue = (25 / 255);

            red += factor * (-200 / 255);
            green += factor * (75 / 255);
            blue += factor * (230 / 255);
        }
        texture_data[i] = red;
        texture_data[i + 1] = blue;
        texture_data[i + 2] = green;
        texture_data[i + 3] = 1.0;
    }

    return texture_data;
}

PYBIND11_MODULE(module_name, handle)
{
    handle.doc() = "Module doc";
    handle.def("generate_mandelbrot", &generate_mandelbrot);
}
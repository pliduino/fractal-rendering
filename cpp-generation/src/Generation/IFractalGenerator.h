#pragma once

#include "../FractalData.h"

class IFractalGenerator
{
public:
    virtual FractalData *Generate() = 0;

protected:
    double step;
    int resolution[2];
};

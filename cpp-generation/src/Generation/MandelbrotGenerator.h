#pragma once

#include "IFractalGenerator.h"

class MandelbrotGenerator : public IFractalGenerator
{
private:
    /* data */
public:
    MandelbrotGenerator(/* args */);
    ~MandelbrotGenerator();
    FractalData *Generate();
};
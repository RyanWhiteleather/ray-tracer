using System;

namespace RayTracer.Models;

public readonly struct Pixel(int x, int y, Color color)
{
    public int X { get; } = x;
    public int Y { get; } = y;
    public Color Color { get; } = color;
}

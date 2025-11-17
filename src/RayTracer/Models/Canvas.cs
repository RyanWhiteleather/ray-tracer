using System;

namespace RayTracer.Models;

public class Canvas
{
    public int Width { get; }
    public int Height { get; }

    /// <summary>
    /// Flat array to contain the pixels in the canvas.
    /// Using a one-dimensional array vs a 2D allows for faster memory access.
    ///
    /// To get a point on the array use, <c>index = y * width + x</c>
    /// </summary>
    /// <value></value>
    public Color[] Pixels { get; }

    public Canvas(int width, int height)
    {
        Width = width;
        Height = height;
        Pixels = new Color[width * height];
    }

    /// <summary>
    /// Writes a color to the canvas at the give coridinates
    /// </summary>
    /// <param name="x"></param>
    /// <param name="y"></param>
    /// <param name="color"></param>
    public void WritePixel(int x, int y, Color color)
    {
        Pixels[y * Width + x] = color;
    }

    public Color PixelAt(int x, int y) => Pixels[y * Width + x];
}

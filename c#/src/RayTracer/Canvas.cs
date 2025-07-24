using RayTracer.Models;

namespace RayTracer;

/// <summary>
/// Canvas to draw the pixels on
/// </summary>
public class Canvas(int width, int height)
{
    public int Width { get; set; } = width;

    public int Height { get; set; } = height;
    private readonly Color[,] _pixels = new Color[width, height];

    /// <summary>
    /// Write a color to the specific x,y cordinate of the canvas.
    /// </summary>
    /// <param name="x"></param>
    /// <param name="y"></param>
    /// <param name="color"></param>
    public void WritePixel(int x, int y, Color color)
    {
        _pixels[x, y] = color;
    }

    public Color GetPixelAt(int x, int y)
    {
        return _pixels[x, y];
    }
}

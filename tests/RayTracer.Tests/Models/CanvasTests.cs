using System;
using RayTracer.Models;

namespace RayTracer.Tests.Models;

public class CanvasTests
{
    [Fact]
    public void NewCanvas_ReturnsCanvas_WithCorrectSizeAndEmptyColors()
    {
        var width = 10;
        var height = 20;
        var canvas = new Canvas(width, height);

        Assert.Equal(10 * 20, canvas.Pixels.Length);

        var emptyColor = new Color(0, 0, 0);
        Assert.All(canvas.Pixels, c => c.Equals(emptyColor));
    }

    [Fact]
    public void WritePixel_SetsPixel_AtCorrectCoridinates()
    {
        var width = 10;
        var height = 20;
        var canvas = new Canvas(width, height);
        var red = new Color(1, 0, 0);
        canvas.WritePixel(2, 3, red);

        Assert.Equal(red, canvas.PixelAt(2, 3));
    }
}

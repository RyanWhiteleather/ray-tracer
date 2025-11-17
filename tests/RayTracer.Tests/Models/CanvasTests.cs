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

    [Fact]
    public void ToPPM_BuildsCorectPPMHeader()
    {
        var width = 5;
        var height = 3;
        var canvas = new Canvas(width, height);

        var result = canvas.ToPPM();
        var lines = result.Split('\n', StringSplitOptions.RemoveEmptyEntries);

        Assert.Equal("P3", lines[0]);
        Assert.Equal($"{canvas.Width} {canvas.Height}", lines[1]);
        Assert.Equal("255", lines[2]);
    }

    [Fact]
    public void ToPPM_BuildsCorectPixelString()
    {
        var width = 5;
        var height = 3;
        var canvas = new Canvas(width, height);
        var color1 = new Color(1.5, 0, 0);
        var color2 = new Color(0, 0.5, 0);
        var color3 = new Color(-0.5, 0, 1);

        canvas.WritePixel(0, 0, color1);
        canvas.WritePixel(2, 1, color2);
        canvas.WritePixel(4, 2, color3);
        var result = canvas.ToPPM();
        var lines = result.Split('\n', StringSplitOptions.RemoveEmptyEntries);

        Assert.Equal("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0", lines[3]);
        Assert.Equal("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0", lines[4]);
        Assert.Equal("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255", lines[5]);
    }

    [Fact]
    public void ToPPM_BuildsCorectPixelString_WithMaxCharacter()
    {
        var width = 10;
        var height = 2;
        var canvas = new Canvas(width, height);
        var color1 = new Color(1, 0.8, 0.6);

        for (int y = 0; y < canvas.Height; y++)
        {
            for (int x = 0; x < canvas.Width; x++)
            {
                canvas.WritePixel(x, y, color1);
            }
        }
        var result = canvas.ToPPM();
        var lines = result.Split('\n', StringSplitOptions.RemoveEmptyEntries);

        Assert.Equal(
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            lines[3]
        );
        Assert.Equal("153 255 204 153 255 204 153 255 204 153 255 204 153", lines[4]);
        Assert.Equal(
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            lines[5]
        );
        Assert.Equal("153 255 204 153 255 204 153 255 204 153 255 204 153", lines[6]);
    }

    [Fact]
    public void ToPPM_BuildsCorectPixelString_EndsWithNewLineCharacter()
    {
        var width = 10;
        var height = 2;
        var canvas = new Canvas(width, height);
        var ppm = canvas.ToPPM();
        Assert.EndsWith("\n", ppm);
    }
}

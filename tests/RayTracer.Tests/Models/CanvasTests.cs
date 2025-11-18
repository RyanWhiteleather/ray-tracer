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
    public void WritePixel_SetsPixel_AtCorrectCoordinates()
    {
        var canvas = new Canvas(10, 20);
        var red = new Color(1, 0, 0);

        canvas[2, 3] = red;

        Assert.Equal(red, canvas[2, 3]);
    }

    [Fact]
    public void WritePixel_Throws_Exception_WhenOutsideCanvasBoundary()
    {
        var canvas = new Canvas(10, 20);
        var red = new Color(1, 0, 0);

        canvas[11, 21] = red;

        // indexer getter should throw
        Assert.Throws<ArgumentOutOfRangeException>(() =>
        {
            var _ = canvas[11, 21];
        });
    }

    [Fact]
    public void ToPPM_BuildsCorrectPPMHeader()
    {
        var canvas = new Canvas(5, 3);

        var result = canvas.ToPPM();
        var lines = result.Split('\n', StringSplitOptions.RemoveEmptyEntries);

        Assert.Equal("P3", lines[0]);
        Assert.Equal($"{canvas.Width} {canvas.Height}", lines[1]);
        Assert.Equal("255", lines[2]);
    }

    [Fact]
    public void ToPPM_BuildsCorrectPixelString()
    {
        var canvas = new Canvas(5, 3);

        var color1 = new Color(1.5, 0, 0);
        var color2 = new Color(0, 0.5, 0);
        var color3 = new Color(-0.5, 0, 1);

        canvas[0, 0] = color1;
        canvas[2, 1] = color2;
        canvas[4, 2] = color3;

        var result = canvas.ToPPM();
        var lines = result.Split('\n', StringSplitOptions.RemoveEmptyEntries);

        Assert.Equal("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0", lines[3]);
        Assert.Equal("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0", lines[4]);
        Assert.Equal("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255", lines[5]);
    }

    [Fact]
    public void ToPPM_BuildsCorrectPixelString_WithMaxCharacterLimit()
    {
        var canvas = new Canvas(10, 2);
        var color = new Color(1, 0.8, 0.6);

        for (int y = 0; y < canvas.Height; y++)
        {
            for (int x = 0; x < canvas.Width; x++)
            {
                canvas[x, y] = color;
            }
        }

        var ppm = canvas.ToPPM();
        var lines = ppm.Split('\n', StringSplitOptions.RemoveEmptyEntries);

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
    public void ToPPM_EndsWithNewline()
    {
        var canvas = new Canvas(10, 2);
        var ppm = canvas.ToPPM();

        Assert.EndsWith("\n", ppm);
    }
}

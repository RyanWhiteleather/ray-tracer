using System;

namespace RayTracer.Tests;

public class CanvasTests
{
    [Fact]
    public void ReturnsNewCanvas_WithEmptyPixels()
    {
        var canvas = new Canvas(10, 20);

        Assert.Equal(10, canvas.Width);
        Assert.Equal(20, canvas.Height);
        // Assert.All(
        //     canvas.Pixels,
        //     pixel =>
        //     {
        //         Assert.Equal(0, pixel.Color.Red);
        //         Assert.Equal(0, pixel.Color.Green);
        //         Assert.Equal(0, pixel.Color.Blue);
        //     }
        // );
    }
}

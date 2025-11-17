using System;
using RayTracer.Models;

namespace RayTracer.Tests.Models;

public class ColorTests
{
    [Fact]
    public void Equals_ReturnsTrue_WhenColorsAreEqual()
    {
        var color1 = new Color(4.3, -4.2, 3.1);
        var color2 = new Color(4.3, -4.2, 3.1);

        Assert.True(color1.Equals(color2));
        Assert.True(color1 == color2);
    }

    [Fact]
    public void Equals_ReturnsFalse_WhenColorsAreNotEqual()
    {
        var color1 = new Color(4.3, -4.2, 3.1);
        var color2 = new Color(10, -4.2, 3.1);

        Assert.False(color1.Equals(color2));
    }

    [Fact]
    public void Equals_ReturnsFalse_WhenComparedWithObjectOfDifferentType()
    {
        var color = new Color(1, 2, 3);
        var notAColor = "hello";

        Assert.False(color.Equals(notAColor));
    }

    [Fact]
    public void EqualOperator_ReturnsTrue_WhenColorsAreEqual()
    {
        var color1 = new Color(4.3, -4.2, 3.1);
        var color2 = new Color(4.3, -4.2, 3.1);

        Assert.True(color1 == color2);
    }

    [Fact]
    public void NotEqualOperator_ReturnsTrue_WhenColorsAreNotEqual()
    {
        var color1 = new Color(4.3, -4.2, 3.1);
        var color2 = new Color(10, -4.2, 3.1);

        Assert.True(color1 != color2);
    }

    [Fact]
    public void GetHashCode_ShouldBeEqual()
    {
        var color1 = new Color(4.3, -4.2, 3.1);
        var color2 = new Color(4.3, -4.2, 3.1);

        var hash1 = color1.GetHashCode();
        var hash2 = color2.GetHashCode();

        Assert.Equal(hash1, hash2);
    }

    [Fact]
    public void GetHashCode_ShouldNotBeEqual()
    {
        var color1 = new Color(4.3, -4.2, 3.1);
        var color2 = new Color(10, -4.2, 3.1);

        var hash1 = color1.GetHashCode();
        var hash2 = color2.GetHashCode();

        Assert.NotEqual(hash1, hash2);
    }

    [Fact]
    public void AddOperator_ReturnsNewColor_WhenAddingTwoColors()
    {
        var color1 = new Color(3, -2, 5);
        var color2 = new Color(-2, 3, 1);

        var result = color1 + color2;
        var expected = new Color(1, 1, 6);

        Assert.Equal(expected, result);
    }

    [Fact]
    public void SubtractOperator_ReturnsNewColor_WhenSubtractingTwoColor()
    {
        var color1 = new Color(3, 2, 1);
        var point2 = new Color(5, 6, 7);

        var result = color1 - point2;
        var expected = new Color(-2, -4, -6);

        Assert.Equal(expected, result);
    }

    [Fact]
    public void MultiplicationOperator_WorksForColorTimesScalarAndScalarTimesColor()
    {
        var color1 = new Color(1, -2, 3);
        var scale = 3.5;

        var result = color1 * scale;
        var expected = new Color(3.5, -7, 10.5);

        Assert.Equal(expected, result);

        var otherResult = scale * color1;
        Assert.Equal(expected, otherResult);
    }

    [Fact]
    public void MultiplicationOperator_WorksForColorTimesColor()
    {
        var color1 = new Color(1, 0.2, 0.4);
        var color2 = new Color(0.9, 1, 0.1);

        var result = color1 * color2;
        var expected = new Color(0.9, 0.2, 0.04);

        Assert.Equal(expected, result);
    }

    [Fact]
    public void DivisionOperator_ReturnsScalarValueColor()
    {
        var color1 = new Color(1, -2, 3);
        var scale = 2;

        var result = color1 / scale;
        var expected = new Color(0.5, -1, 1.5);

        Assert.Equal(expected, result);
    }

    [Theory]
    [InlineData(1, 2, 3, "(1.00, 2.00, 3.00)")]
    [InlineData(0, 0, 0, "(0.00, 0.00, 0.00)")]
    [InlineData(-1.234, 5.678, -9.1011, "(-1.23, 5.68, -9.10)")]
    [InlineData(0.0049, 0.005, 0.006, "(0.00, 0.01, 0.01)")] // Tests rounding
    public void ToString_ReturnsExpectedFormat(double r, double g, double b, string expected)
    {
        var color = new Color(r, g, b);
        var result = color.ToString();

        Assert.Equal(expected, result);
    }
}

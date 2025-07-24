using RayTracer.Models;

namespace RayTracer.Tests.Models;

public class PointTests
{
    [Fact]
    public void Equals_ReturnsTrue_WhenPointsAreEqual()
    {
        var point1 = new Point(4.3, -4.2, 3.1);
        var point2 = new Point(4.3, -4.2, 3.1);

        Assert.True(point1.Equals(point2));
        Assert.True(point1 == point2);
    }

    [Fact]
    public void Equals_ReturnsFalse_WhenPointsAreNotEqual()
    {
        var point1 = new Point(4.3, -4.2, 3.1);
        var point2 = new Point(10, -4.2, 3.1);

        Assert.False(point1.Equals(point2));
    }

    [Fact]
    public void Equals_ReturnsFalse_WhenComparedWithObjectOfDifferentType()
    {
        var point = new Point(1, 2, 3);
        var notAPoint = "hello";

        Assert.False(point.Equals(notAPoint));
    }

    [Fact]
    public void EqualOperator_ReturnsTrue_WhenPointsAreEqual()
    {
        var point1 = new Point(4.3, -4.2, 3.1);
        var point2 = new Point(4.3, -4.2, 3.1);

        Assert.True(point1 == point2);
    }

    [Fact]
    public void NotEqualOperator_ReturnsTrue_WhenPointsAreNotEqual()
    {
        var point1 = new Point(4.3, -4.2, 3.1);
        var point2 = new Point(10, -4.2, 3.1);

        Assert.True(point1 != point2);
    }

    [Fact]
    public void GetHashCode_ShouldBeEqual()
    {
        var point1 = new Point(4.3, -4.2, 3.1);
        var point2 = new Point(4.3, -4.2, 3.1);

        var hash1 = point1.GetHashCode();
        var hash2 = point2.GetHashCode();

        Assert.Equal(hash1, hash2);
    }

    [Fact]
    public void GetHashCode_ShouldNotBeEqual()
    {
        var point1 = new Point(4.3, -4.2, 3.1);
        var point2 = new Point(10, -4.2, 3.1);

        var hash1 = point1.GetHashCode();
        var hash2 = point2.GetHashCode();

        Assert.NotEqual(hash1, hash2);
    }

    [Fact]
    public void AddOperator_ReturnsNewPoint_WhenAddingPointAndVector()
    {
        var point1 = new Point(3, -2, 5);
        var vector1 = new Vector(-2, 3, 1);

        var result = point1 + vector1;
        var expected = new Point(1, 1, 6);

        Assert.Equal(expected, result);
    }

    [Fact]
    public void SubtractOperator_ReturnsNewVector_WhenSubtractingTwoPoints()
    {
        var point1 = new Point(3, 2, 1);
        var point2 = new Point(5, 6, 7);

        var result = point1 - point2;
        var expected = new Vector(-2, -4, -6);

        Assert.Equal(expected, result);
    }

    [Fact]
    public void SubtractOperator_ReturnsNewPoint_WhenSubtractingVectorFromPoint()
    {
        var point1 = new Point(3, 2, 1);
        var vector1 = new Vector(5, 6, 7);

        var result = point1 - vector1;
        var expected = new Point(-2, -4, -6);

        Assert.Equal(expected, result);
    }

    [Fact]
    public void MultiplicationOperator_WorksForPointTimesScalarAndScalarTimesPoint()
    {
        var point1 = new Point(1, -2, 3);
        var scale = 3.5;

        var result = point1 * scale;
        var expected = new Point(3.5, -7, 10.5);

        Assert.Equal(expected, result);

        var otherResult = scale * point1;
        Assert.Equal(expected, otherResult);
    }

    [Fact]
    public void DivisionOperator_ReturnsScalarValuePoint()
    {
        var point1 = new Point(1, -2, 3);
        var scale = 2;

        var result = point1 / scale;
        var expected = new Point(0.5, -1, 1.5);

        Assert.Equal(expected, result);
    }

    [Theory]
    [InlineData(1, 2, 3, "(1.00, 2.00, 3.00)")]
    [InlineData(0, 0, 0, "(0.00, 0.00, 0.00)")]
    [InlineData(-1.234, 5.678, -9.1011, "(-1.23, 5.68, -9.10)")]
    [InlineData(0.0049, 0.005, 0.006, "(0.00, 0.01, 0.01)")] // Tests rounding
    public void ToString_ReturnsExpectedFormat(double x, double y, double z, string expected)
    {
        var point = new Point(x, y, z);
        var result = point.ToString();

        Assert.Equal(expected, result);
    }
}

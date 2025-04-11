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
    public void AddOperator_ReturnsNewPoint_WhenAddingPointAndVector()
    {
        var point1 = new Point(3, -2, 5); 
        var vector1 = new Vector(-2, 3, 1); 

        var result = point1 + vector1;
        var expected = new Point(1, 1, 6);
        
        Assert.Equal(result, expected);
    }
}

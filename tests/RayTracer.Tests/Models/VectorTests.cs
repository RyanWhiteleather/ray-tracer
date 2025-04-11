using RayTracer.Models;

namespace RayTracer.Tests.Models;

public class VectorTests
{
    [Fact]
    public void Equals_ReturnsTrue_WhenVectorsAreEqual()
    {
        var Vector1 = new Vector(4.3, -4.2, 3.1); 
        var Vector2 = new Vector(4.3, -4.2, 3.1); 

        Assert.True(Vector1.Equals(Vector2));
        Assert.True(Vector1 == Vector2);
    }

    [Fact]
    public void Equals_ReturnsFalse_WhenVectorsAreNotEqual()
    {
        var Vector1 = new Vector(4.3, -4.2, 3.1); 
        var Vector2 = new Vector(10, -4.2, 3.1); 

        Assert.False(Vector1.Equals(Vector2));
    }

    [Fact]
    public void Equals_ReturnsFalse_WhenComparedWithObjectOfDifferentType()
    {
        var Vector = new Vector(1, 2, 3);
        var notAVector = "hello";

        Assert.False(Vector.Equals(notAVector));
    }

    [Fact]
    public void EqualOperator_ReturnsTrue_WhenVectorsAreEqual()
    {
        var Vector1 = new Vector(4.3, -4.2, 3.1); 
        var Vector2 = new Vector(4.3, -4.2, 3.1); 

        Assert.True(Vector1 == Vector2);
    }

    [Fact]
    public void NotEqualOperator_ReturnsTrue_WhenVectorsAreNotEqual()
    {
        var Vector1 = new Vector(4.3, -4.2, 3.1); 
        var Vector2 = new Vector(10, -4.2, 3.1); 

        Assert.True(Vector1 != Vector2);
    }

    [Fact]
    public void Add_ReturnsNewVector_WhenAddingVectorAndVector()
    {
        var vector1 = new Vector(3, -2, 5); 
        var vector2 = new Vector(-2, 3, 1); 

        var result = vector1 + vector2;
        var expected = new Vector(1, 1, 6);
        
        Assert.Equal(result, expected);
    }

    [Fact]
    public void Add_ReturnsNewPoint_WhenAddingVectorAndPoint()
    {
        var vector1 = new Vector(3, -2, 5); 
        var point1 = new Point(-2, 3, 1); 

        var result = vector1 + point1;
        var expected = new Point(1, 1, 6);
        
        Assert.Equal(result, expected);
    }
}

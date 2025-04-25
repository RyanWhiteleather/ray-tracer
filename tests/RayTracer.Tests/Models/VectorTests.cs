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
    public void AddOperator_ReturnsNewVector_WhenAddingVectorAndVector()
    {
        var vector1 = new Vector(3, -2, 5);
        var vector2 = new Vector(-2, 3, 1);

        var result = vector1 + vector2;
        var expected = new Vector(1, 1, 6);

        Assert.Equal(expected, result);
    }

    [Fact]
    public void AddOperator_ReturnsNewPoint_WhenAddingVectorAndPoint()
    {
        var vector1 = new Vector(3, -2, 5);
        var point1 = new Point(-2, 3, 1);

        var result = vector1 + point1;
        var expected = new Point(1, 1, 6);

        Assert.Equal(expected, result);
    }

    [Fact]
    public void SubtractOperator_ReturnsNewVector_WhenSubtractingTwoVectors()
    {
        var vector1 = new Vector(3, 2, 1);
        var vector2 = new Vector(5, 6, 7);

        var result = vector1 - vector2;
        var expected = new Vector(-2, -4, -6);

        Assert.Equal(expected, result);
    }

    [Fact]
    public void NegateOperator_ReturnsOppositeVector()
    {
        var vector1 = new Vector(1, -2, 3);

        var result = -vector1;
        var expected = new Vector(-1, 2, -3);

        Assert.Equal(expected, result);
    }

    [Fact]
    public void MultiplicationOperator_WorksForVectorTimesScalarAndScalarTimesVector()
    {
        var vector1 = new Vector(1, -2, 3);
        var scale = 3.5;

        var result = vector1 * scale;
        var expected = new Vector(3.5, -7, 10.5);

        Assert.Equal(expected, result);

        var otherResult = scale * vector1;
        Assert.Equal(expected, otherResult);
    }

    [Fact]
    public void DivisionOperator_ReturnsScalarVAlueVector()
    {
        var vector1 = new Vector(1, -2, 3);
        var scale = 2;

        var result = vector1 / scale;
        var expected = new Vector(0.5, -1, 1.5);

        Assert.Equal(expected, result);
    }
}

using Microsoft.VisualStudio.TestPlatform.Common.Utilities;
using RayTracer.Models;
namespace RayTracer.Tests.Models;

public class Tuple4Tests
{

    [Fact]
    public void Point_CreatesTuple4WithWEqualTo1(){
        var point = Tuple4.Point(4.3, -4.2, 3.1); 

        Assert.Equal(4.3, point.X);
        Assert.Equal(-4.2, point.Y);
        Assert.Equal(3.1, point.Z);
        Assert.Equal(Tuple4.POINT_W, point.W);
    }

    [Fact]
    public void Vector_CreatesTupl4WithWEqualTo0(){
        var point = Tuple4.Vector(4.3, -4.2, 3.1); 

        Assert.Equal(4.3, point.X);
        Assert.Equal(-4.2, point.Y);
        Assert.Equal(3.1, point.Z);
        Assert.Equal(Tuple4.VECTOR_W, point.W);
    }

    [Fact]
    public void Point_IsNotVector()
    {
        var point = Tuple4.Point(0, 0, 0);
        Assert.NotEqual(Tuple4.VECTOR_W, point.W);
    }

    [Fact]
    public void Vector_IsNotPoint()
    {
        var vector = Tuple4.Vector(0, 0, 0);
        Assert.NotEqual(Tuple4.POINT_W, vector.W);
    }

    [Fact]
    public void IsEqual_ReturnsTrue_WhenTuple4sAreEqual()
    {
        var point1 = Tuple4.Point(4.3, -4.2, 3.1); 
        var point2 = Tuple4.Point(4.3, -4.2, 3.1); 

        Assert.True(point1.Equals(point2));
    }

    [Fact]
    public void IsEqual_ReturnsFalse_WhenTuple4sAreNotEqual()
    {
        var point1 = Tuple4.Point(4.3, -4.2, 3.1); 
        var vector1 = Tuple4.Vector(4.3, -4.2, 3.1); 

        Assert.False(point1.Equals(vector1));
    }

    [Fact]
    public void Add_ReturnsNewPoint_WhenAddingPointAndVector()
    {
        var point1 = Tuple4.Point(3, -2, 5); 
        var vector1 = Tuple4.Vector(-2, 3, 1); 

        var result = point1.Add(vector1);
        var expected =Tuple4.Point(1, 1, 6);
        

        Assert.Equal(result, expected);
        Assert.True(result.IsPoint);
    }

    [Fact]
    public void Add_ReturnsNewVector_WhenAddingVectorAndVector()
    {
        var vector1 = Tuple4.Vector(3, -2, 5); 
        var vector2 = Tuple4.Vector(-2, 3, 1); 

        var result = vector1.Add(vector2);
        var expected =Tuple4.Vector(1, 1, 6);
        

        Assert.Equal(result, expected);
        Assert.True(result.IsVector);
    }

    [Fact]
    public void Add_ThrowsInvalidOperation_WhenAddingPointAndPoint()
    {
        var point1 = Tuple4.Point(3, -2, 5); 
        var point2 = Tuple4.Point(-2, 3, 1); 

        Assert.Throws<InvalidOperationException>(() => point1.Add(point2));
    }
}

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
    public void Add_ReturnsNewTuple4()
    {
        var point1 = Tuple4.Point(3, -2, 5); 
        var vector1 = Tuple4.Vector(-2, 3, 1); 

        var expected =Tuple4.Point(1, 1, 6);

        Assert.Equal(point1.Add(vector1), expected);

    }
}

using System;
using RayTracer.Models;
namespace RayTracer.Tests.Models;

public class RayTracerTupleTests
{

    [Fact]
    public void Point_CreatesRayTracerTupleWithWEqualTo1(){
        var point = RayTracerTuple.Point(4.3, -4.2, 3.1); 

        Assert.Equal(4.3, point.X);
        Assert.Equal(-4.2, point.Y);
        Assert.Equal(3.1, point.Z);
        Assert.Equal(RayTracerTuple.POINT_W, point.W);
    }

    [Fact]
    public void Vector_CreatesRayTracerTupleWithWEqualTo0(){
        var point = RayTracerTuple.Vector(4.3, -4.2, 3.1); 

        Assert.Equal(4.3, point.X);
        Assert.Equal(-4.2, point.Y);
        Assert.Equal(3.1, point.Z);
        Assert.Equal(RayTracerTuple.VECTOR_W, point.W);
    }

    [Fact]
    public void Point_IsNotVector()
    {
        var point = RayTracerTuple.Point(0, 0, 0);
        Assert.NotEqual(RayTracerTuple.VECTOR_W, point.W);
    }

    [Fact]
    public void Vector_IsNotPoint()
    {
        var vector = RayTracerTuple.Vector(0, 0, 0);
        Assert.NotEqual(RayTracerTuple.POINT_W, vector.W);
    }
}

using System;
using System.Runtime.CompilerServices;

namespace RayTracer.Models;

/// <summary>
/// A Tuple is just an ordered list of things, like numbers.
/// </summary>
public class RayTracerTuple
{
    public const double POINT_W = 1.0;
    public const double VECTOR_W = 0.0;

    public double X {get; set;}
    public double Y {get; set;}
    public double Z {get; set;}
    public double W {get; set;}

    /// <summary>
    /// Creates a new Point Tuple
    /// </summary>
    /// <param name="x"></param>
    /// <param name="y"></param>
    /// <param name="z"></param>
    /// <returns></returns>
    public static RayTracerTuple Point(double x, double y, double z) => new(x, y, z, POINT_W);

    /// <summary>
    /// Creates a new Vector Tuple
    /// </summary>
    /// <param name="x"></param>
    /// <param name="y"></param>
    /// <param name="z"></param>
    /// <returns></returns>
    public static RayTracerTuple Vector(double x, double y, double z) => new(x, y, z, VECTOR_W);

    /// <summary>
    /// Helper method to create a new Tuple
    /// </summary>
    /// <param name="x"></param>
    /// <param name="y"></param>
    /// <param name="z"></param>
    /// <param name="w"></param>
    /// <returns></returns>
    private RayTracerTuple(double x, double y, double z, double w) => (X,Y,Z,W) = (x, y, z, w);

    
}

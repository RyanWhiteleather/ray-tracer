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

    public const double EPSILON = 0.00001;

    public double X {get; set;}
    public double Y {get; set;}
    public double Z {get; set;}
    public double W {get; set;}

    /// <summary>
    /// Create a new RayTracerTuple
    /// </summary>
    /// <param name="x"></param>
    /// <param name="y"></param>
    /// <param name="z"></param>
    /// <param name="w"></param>
    /// <returns></returns>
    public RayTracerTuple(double x, double y, double z, double w) => (X,Y,Z,W) = (x, y, z, w);

    /// <summary>
    /// Helper method to create a new Point Tuple
    /// </summary>
    /// <param name="x"></param>
    /// <param name="y"></param>
    /// <param name="z"></param>
    /// <returns></returns>
    public static RayTracerTuple Point(double x, double y, double z) => new(x, y, z, POINT_W);

    /// <summary>
    /// Helper method to create a new Vector Tuple
    /// </summary>
    /// <param name="x"></param>
    /// <param name="y"></param>
    /// <param name="z"></param>
    /// <returns></returns>
    public static RayTracerTuple Vector(double x, double y, double z) => new(x, y, z, VECTOR_W);

    /// <summary>
    /// Checks the equality of two RayTracerTuples
    /// </summary>
    /// <param name="other"></param>
    /// <returns></returns>
    public bool Equals(RayTracerTuple? other)
    {
        if (other is null)
        {
            return false;
        } 

        return Math.Abs(X - other.X) < EPSILON &&
               Math.Abs(Y - other.Y) < EPSILON &&
               Math.Abs(Z - other.Z) < EPSILON &&
               Math.Abs(W - other.W) < EPSILON;
    }

    /// <summary>
    /// Add two RayTracerTuples together
    /// </summary>
    /// <param name="other"></param>
    /// <returns></returns>
    public RayTracerTuple Add(RayTracerTuple other)
    {
        //TODO I don't think adding of the w's is correct
        return new RayTracerTuple(X + other.X, Y + other.Y, Z + other.Z, W + other.W);
    }

    public override bool Equals(object? obj)
    {
        return obj is RayTracerTuple other && Equals(other);
    }
        
    public override int GetHashCode() {
        return HashCode.Combine(Math.Round(X / EPSILON),
                         Math.Round(Y / EPSILON),
                         Math.Round(Z / EPSILON),
                         Math.Round(W / EPSILON));
    }
}

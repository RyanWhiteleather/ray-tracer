using System;
using System.Runtime.CompilerServices;

namespace RayTracer.Models;

/// <summary>
/// A Tuple is just an ordered list of things, like numbers.
/// The Tuple4 represents a Tuple with 4 items in it.
/// </summary>
public class Tuple4
{
    public const double POINT_W = 1.0;
    public const double VECTOR_W = 0.0;

    public const double EPSILON = 0.00001;

    public double X {get; set;}
    public double Y {get; set;}
    public double Z {get; set;}
    public double W {get; set;}

    /// <summary>
    /// Helper method to create a new Point Tuple
    /// </summary>
    /// <param name="x"></param>
    /// <param name="y"></param>
    /// <param name="z"></param>
    /// <returns></returns>
    public static Tuple4 Point(double x, double y, double z) => new(x, y, z, POINT_W);

    /// <summary>
    /// Helper method to create a new Vector Tuple
    /// </summary>
    /// <param name="x"></param>
    /// <param name="y"></param>
    /// <param name="z"></param>
    /// <returns></returns>
    public static Tuple4 Vector(double x, double y, double z) => new(x, y, z, VECTOR_W);

    /// <summary>
    /// Checks the equality of two Tuple4s
    /// </summary>
    /// <param name="other"></param>
    /// <returns></returns>
    public bool Equals(Tuple4? other)
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
    /// Add two Tuple4s together
    /// </summary>
    /// <param name="other"></param>
    /// <returns></returns>
    public Tuple4 Add(Tuple4 other)
    {
        //TODO I don't think adding of the w's is correct
        return new Tuple4(X + other.X, Y + other.Y, Z + other.Z, W + other.W);
    }

    public override bool Equals(object? obj)
    {
        return obj is Tuple4 other && Equals(other);
    }
        
    public override int GetHashCode() {
        return HashCode.Combine(Math.Round(X / EPSILON),
                         Math.Round(Y / EPSILON),
                         Math.Round(Z / EPSILON),
                         Math.Round(W / EPSILON));
    }

    /// <summary>
    /// Create a new Tuple4
    /// </summary>
    /// <param name="x"></param>
    /// <param name="y"></param>
    /// <param name="z"></param>
    /// <param name="w"></param>
    /// <returns></returns>
    private Tuple4(double x, double y, double z, double w) => (X,Y,Z,W) = (x, y, z, w);
}

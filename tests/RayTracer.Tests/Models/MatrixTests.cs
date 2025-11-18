using System;
using RayTracer.Models;

namespace RayTracer.Tests.Models;

public class MatrixTests
{
    [Fact]
    public void NewMatrix_CreatesAndSets_4x4()
    {
        var matrix = new Matrix(
            new double[,]
            {
                { 1, 2, 3, 4 },
                { 5.5, 6.5, 7.5, 8.5 },
                { 9, 10, 11, 12 },
                { 13.5, 14.5, 15.5, 16.5 },
            }
        );

        Assert.Equal(1, matrix[0, 0]);
        Assert.Equal(4, matrix[0, 3]);
        Assert.Equal(5.5, matrix[1, 0]);
        Assert.Equal(7.5, matrix[1, 2]);
        Assert.Equal(11, matrix[2, 2]);
        Assert.Equal(13.5, matrix[3, 0]);
        Assert.Equal(15.5, matrix[3, 2]);
    }

    [Fact]
    public void NewMatrix_CreatesAndSets_2x2()
    {
        var matrix = new Matrix(
            new double[,]
            {
                { -3, 5 },
                { 1, -2 },
            }
        );

        Assert.Equal(-3, matrix[0, 0]);
        Assert.Equal(5, matrix[0, 1]);
        Assert.Equal(1, matrix[1, 0]);
        Assert.Equal(-2, matrix[1, 1]);
    }

    [Fact]
    public void NewMatrix_CreatesAndSets_3x3()
    {
        var matrix = new Matrix(
            new double[,]
            {
                { -3, 5, 0 },
                { 1, -2, -7 },
                { 0, 1, 1 },
            }
        );

        Assert.Equal(-3, matrix[0, 0]);
        Assert.Equal(-2, matrix[1, 1]);
        Assert.Equal(1, matrix[2, 2]);
    }

    [Fact]
    public void Equals_ReturnsTrue_WhenMatrixsAreEqual()
    {
        var a = new Matrix(
            new double[,]
            {
                { 1, 2, 3, 4 },
                { 5, 6, 7, 8 },
                { 9, 10, 11, 12 },
                { 13, 14, 15, 16 },
            }
        );

        var b = new Matrix(
            new double[,]
            {
                { 1, 2, 3, 4 },
                { 5, 6, 7, 8 },
                { 9, 10, 11, 12 },
                { 13, 14, 15, 16 },
            }
        );

        Assert.True(a == b);
        Assert.False(a != b);
    }

    [Fact]
    public void Equals_ReturnsFalse_WhenMatrixsAreNotEqual()
    {
        var a = new Matrix(
            new double[,]
            {
                { 1, 2, 3, 4 },
                { 5, 6, 7, 8 },
                { 9, 10, 11, 12 },
                { 13, 14, 15, 16 },
            }
        );

        var b = new Matrix(
            new double[,]
            {
                { 0, 0, 0, 0 },
                { 5, 6, 7, 8 },
                { 9, 10, 11, 12 },
                { 13, 14, 15, 16 },
            }
        );

        Assert.True(a != b);
        Assert.False(a == b);
    }
}

using System;

namespace RayTracer.Models;

public readonly struct Matrix
{
    public int Columns { get; }
    public int Rows { get; }

    /// <summary>
    /// Contains the matrix elements in a flat array stored in row-major order.
    /// The index for a given row and column is calculated as:
    /// <c>index = Columns * row + column</c>.
    ///
    /// Example layout:
    ///  0  |  1 |  2 |  3
    ///  4  |  5 |  6 |  7
    ///  8  |  9 | 10 | 11
    /// 12  | 13 | 14 | 15
    ///
    /// For example: Elements[2,3] → row 2, column 3 → 4 * 2 + 3 = 11.
    /// </summary>
    public double[] Elements { get; }

    /// <summary>
    /// Allows calling the matrix like this matrix[1,2] =  7.5 or matrix[1,2].
    /// Still goes through the correct logic to actuall update and get the matrix.
    /// </summary>
    /// <value></value>
    public double this[int row, int column]
    {
        get => GetElement(row, column);
        set => SetElement(row, column, value);
    }

    public Matrix(int rows, int columns)
    {
        Rows = rows;
        Columns = columns;
        Elements = new double[Columns * Rows];
    }

    public Matrix(double[,] values)
    {
        Rows = values.GetLength(0);
        Columns = values.GetLength(1);
        Elements = new double[Rows * Columns];

        for (int r = 0; r < Rows; r++)
        {
            for (int c = 0; c < Columns; c++)
            {
                Elements[r * Columns + c] = values[r, c];
            }
        }
    }

    /// <summary>
    /// Get Elements from the Matrix array
    /// </summary>
    /// <param name="row"></param>
    /// <param name="column"></param>
    /// <returns></returns>
    public double GetElement(int row, int column)
    {
        if (row < 0 || row >= Rows)
        {
            throw new ArgumentOutOfRangeException(nameof(row));
        }

        if (column < 0 || column >= Columns)
        {
            throw new ArgumentOutOfRangeException(nameof(column));
        }

        return Elements[row * Columns + column];
    }

    public void SetElement(int row, int column, double value)
    {
        if (row < 0 || row >= Rows)
        {
            throw new ArgumentOutOfRangeException(nameof(row));
        }

        if (column < 0 || column >= Columns)
        {
            throw new ArgumentOutOfRangeException(nameof(column));
        }

        Elements[row * Columns + column] = value;
    }
}

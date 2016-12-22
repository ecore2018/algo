// swiftlint:disable variable_name

enum Direction {
    case Clockwise, CounterClockwise, Collinear
}

func direction(_ a: Point, _ b: Point, _ c: Point) -> Direction {
    // The signed area of the triangle (a, b, c) is half the determinant of the following matrix:
    //     [ ax, ay, 1 ]
    //     [ bx, by, 1 ]
    //     [ cx, cy, 1 ]
    // If the signed area is > 0 then a-b-c is a counter-clockwise turn.
    // If the signed area is < 0 then a-b-c is a clockwise turn.
    // If the signed area is = 0 then a-b-c are collinear.
    let det = a.x * b.y + b.x * c.y + c.x * a.y - c.x * b.y - a.x * c.y - b.x * a.y
    return det > 0 ? .CounterClockwise : det < 0 ? .Clockwise : .Collinear
}

func deduplicate(_ edges: [Edge]) -> [Edge] {
    var deduplicated = [Edge]()

    for edge in edges.sorted(by: <) {
        if deduplicated.isEmpty || deduplicated.last! != edge {
            deduplicated.append(edge)
        }
    }

    return deduplicated
}

public func triangulate(_ points: [Point]) -> [Edge] {
    guard points.count > 1 else {
        return []
    }

    // Sort the points lexicographically.
    let ordered = points.sorted(by: <)

    var edges = [Edge]()

    // Construct the lower and upper hulls incrementally. Every time a point is removed from the
    // hull, we add edges to the two nearest points; this will build the interior edges of the
    // triangulation.
    var lower = [Point]()
    var upper = [Point]()
    for point in ordered {
        while lower.count > 1 &&
            direction(lower[lower.count - 2], lower.last!, point) == .Clockwise {
            let removed = lower.removeLast()
            edges += [(lower.last!, removed), (removed, point)]
        }
        lower.append(point)

        while upper.count > 1 &&
            direction(upper[upper.count - 2], upper.last!, point) == .CounterClockwise {
            let removed = upper.removeLast()
            edges += [(upper.last!, removed), (removed, point)]
        }
        upper.append(point)
    }

    // Add the hull edges to the triangulation.
    for i in 0..<lower.count - 1 {
        edges.append((lower[i], lower[i + 1]))
    }
    for i in 0..<upper.count - 1 {
        edges.append((upper[i], upper[i + 1]))
    }

    return deduplicate(edges)
}

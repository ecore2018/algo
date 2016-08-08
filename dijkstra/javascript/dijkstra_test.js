'use strict';

var assert = require('assert');

var dijkstra = require('./dijkstra.js');

var tests = [
    {
        graph: {
            vertexCount: 1,
            directed: false,
            edges: []
        },
        problems: [
            {start: 0, end: 0, solution: [0]}
        ]
    },
    {
        graph: {
            vertexCount: 2,
            directed: false,
            edges: []
        },
        problems: [
            {start: 0, end: 1, solution: null}
        ]
    },
    {
        graph: {
            vertexCount: 2,
            directed: false,
            edges: [
                {x: 0, y: 1, weight: 7}
            ]
        },
        problems: [
            {start: 0, end: 1, solution: [0, 1]}
        ]
    },
    {
        graph: {
            vertexCount: 2,
            directed: true,
            edges: [
                {x: 0, y: 1, weight: 7}
            ]
        },
        problems: [
            {start: 0, end: 1, solution: [0, 1]},
            {start: 1, end: 0, solution: null}
        ]
    },
    {
        graph: {
            vertexCount: 6,
            directed: false,
            edges: [
                {x: 0, y: 1, weight: 7},
                {x: 0, y: 2, weight: 9},
                {x: 0, y: 5, weight: 14},
                {x: 1, y: 2, weight: 10},
                {x: 1, y: 3, weight: 15},
                {x: 2, y: 3, weight: 11},
                {x: 2, y: 5, weight: 2},
                {x: 3, y: 4, weight: 6},
                {x: 4, y: 5, weight: 9}
            ]
        },
        problems: [
            // See an animated illustration of this example at:
            // http://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
            {start: 0, end: 4, solution: [0, 2, 5, 4]}
        ]
    }
];

function runTest(test) {
    test.problems.forEach(function(problem) {
        var solution = dijkstra(test.graph, problem.start, problem.end);
        assert.deepStrictEqual(solution, problem.solution);
    });
}

tests.forEach(runTest);
